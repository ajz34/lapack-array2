#[allow(unused)]

use crate::util::*;
use ndarray::prelude::*;

/* #region ArrayOut */

#[derive(Debug)]
pub enum ArrayOut<'a, F, D>
where
    D: Dimension,
{
    ViewMut(ArrayViewMut<'a, F, D>),
    Owned(Array<F, D>),
    ToBeCloned(ArrayViewMut<'a, F, D>, Array<F, D>),
}

impl<F, D> ArrayOut<'_, F, D>
where
    F: Clone,
    D: Dimension,
{
    pub fn view(&self) -> ArrayView<'_, F, D> {
        match self {
            Self::ViewMut(arr) => arr.view(),
            Self::Owned(arr) => arr.view(),
            Self::ToBeCloned(_, arr) => arr.view(),
        }
    }

    pub fn view_mut(&mut self) -> ArrayViewMut<'_, F, D> {
        match self {
            Self::ViewMut(arr) => arr.view_mut(),
            Self::Owned(arr) => arr.view_mut(),
            Self::ToBeCloned(_, arr) => arr.view_mut(),
        }
    }

    pub fn into_owned(self) -> Array<F, D> {
        match self {
            Self::ViewMut(arr) => arr.to_owned(),
            Self::Owned(arr) => arr,
            Self::ToBeCloned(mut arr_view, arr_owned) => {
                arr_view.assign(&arr_owned);
                arr_owned
            },
        }
    }

    pub fn is_view_mut(&mut self) -> bool {
        match self {
            Self::ViewMut(_) => true,
            Self::Owned(_) => false,
            Self::ToBeCloned(_, _) => true,
        }
    }

    pub fn is_owned(&mut self) -> bool {
        match self {
            Self::ViewMut(_) => false,
            Self::Owned(_) => true,
            Self::ToBeCloned(_, _) => false,
        }
    }

    pub fn clone_to_view_mut(self) -> Self {
        match self {
            ArrayOut::ToBeCloned(mut arr_view, arr_owned) => {
                arr_view.assign(&arr_owned);
                ArrayOut::ViewMut(arr_view)
            },
            _ => self,
        }
    }

    pub fn reversed_axes(self) -> Self {
        match self {
            ArrayOut::ViewMut(arr) => ArrayOut::ViewMut(arr.reversed_axes()),
            ArrayOut::Owned(arr) => ArrayOut::Owned(arr.reversed_axes()),
            ArrayOut::ToBeCloned(mut arr_view, arr_owned) => {
                arr_view.assign(&arr_owned);
                ArrayOut::ViewMut(arr_view.reversed_axes())
            },
        }
    }

    pub fn get_data_mut_ptr(&mut self) -> *mut F {
        match self {
            Self::ViewMut(arr) => arr.as_mut_ptr(),
            Self::Owned(arr) => arr.as_mut_ptr(),
            Self::ToBeCloned(_, arr) => arr.as_mut_ptr(),
        }
    }
}

pub type ArrayOut1<'a, F> = ArrayOut<'a, F, Ix1>;
pub type ArrayOut2<'a, F> = ArrayOut<'a, F, Ix2>;
pub type ArrayOut3<'a, F> = ArrayOut<'a, F, Ix3>;

pub trait ArrayOutTuple2<F1, D1, F2, D2> {
    fn into_owned(self) -> (Array<F1, D1>, Array<F2, D2>);
}

impl<F1, D1, F2, D2> ArrayOutTuple2<F1, D1, F2, D2> for (ArrayOut<'_, F1, D1>, ArrayOut<'_, F2, D2>)
where
    F1: Clone,
    F2: Clone,
    D1: Dimension,
    D2: Dimension,
{
    fn into_owned(self) -> (Array<F1, D1>, Array<F2, D2>) {
        (self.0.into_owned(), self.1.into_owned())
    }
}

/* #endregion */

/* #region ArrayViewOrMut */

pub enum ArrayViewOrMut<'a, F, D>
where
    D: Dimension,
{
    View(ArrayView<'a, F, D>),
    ViewMut(ArrayViewMut<'a, F, D>),
}

pub type ArrayViewOrMut1<'a, F> = ArrayViewOrMut<'a, F, Ix1>;
pub type ArrayViewOrMut2<'a, F> = ArrayViewOrMut<'a, F, Ix2>;

impl<'a, F, D> ArrayViewOrMut<'a, F, D>
where
    D: Dimension,
{
    pub fn view(&self) -> ArrayView<'_, F, D> {
        match self {
            Self::View(arr) => arr.view(),
            Self::ViewMut(arr) => arr.view(),
        }
    }

    pub fn reversed_axes(self) -> Self {
        match self {
            Self::View(arr) => Self::View(arr.reversed_axes()),
            Self::ViewMut(arr) => Self::ViewMut(arr.reversed_axes()),
        }
    }
}

impl<'a, F, D> From<ArrayViewMut<'a, F, D>> for ArrayViewOrMut<'a, F, D>
where
    D: Dimension,
{
    fn from(arr: ArrayViewMut<'a, F, D>) -> Self {
        Self::ViewMut(arr)
    }
}

impl<'a, F, D> From<ArrayView<'a, F, D>> for ArrayViewOrMut<'a, F, D>
where
    D: Dimension,
{
    fn from(arr: ArrayView<'a, F, D>) -> Self {
        Self::View(arr)
    }
}

/* #endregion */

/* #region Strides */

#[inline]
pub fn get_layout_array2<F>(arr: &ArrayView2<F>) -> LapackLayout {
    // Note that this only shows order of matrix (dimension information)
    // not c/f-contiguous (memory layout)
    // So some sequential (both c/f-contiguous) cases may be considered as only row or col major
    // Examples:
    // RowMajor     ==>   shape=[1, 4], strides=[0, 1], layout=CFcf (0xf)
    // ColMajor     ==>   shape=[4, 1], strides=[1, 0], layout=CFcf (0xf)
    // Sequential   ==>   shape=[1, 1], strides=[0, 0], layout=CFcf (0xf)
    // NonContig    ==>   shape=[4, 1], strides=[10, 0], layout=Custom (0x0)
    let (d0, d1) = arr.dim();
    let [s0, s1] = arr.strides().try_into().unwrap();
    if d0 == 0 || d1 == 0 {
        // empty array
        return LapackLayout::Sequential;
    } else if d0 == 1 && d1 == 1 {
        // one element
        return LapackLayout::Sequential;
    } else if s1 == 1 {
        // row-major
        return LapackRowMajor;
    } else if s0 == 1 {
        // col-major
        return LapackColMajor;
    } else {
        // non-contiguous
        return LapackLayout::NonContiguous;
    }
}

/* #endregion */

/* #region contiguous preference */

pub(crate) trait LayoutPref {
    fn is_fpref(&self) -> bool;
    fn is_cpref(&self) -> bool;
}

impl<A> LayoutPref for ArrayView2<'_, A> {
    fn is_fpref(&self) -> bool {
        get_layout_array2(self).is_fpref()
    }

    fn is_cpref(&self) -> bool {
        get_layout_array2(self).is_cpref()
    }
}

/* #endregion */

/* #region warn on clone */

pub(crate) trait ToLayoutCowArray2<A> {
    fn to_row_layout(&self) -> Result<CowArray<'_, A, Ix2>, LapackError>;
    fn to_col_layout(&self) -> Result<CowArray<'_, A, Ix2>, LapackError>;
}

impl<A> ToLayoutCowArray2<A> for ArrayView2<'_, A>
where
    A: Clone,
{
    fn to_row_layout(&self) -> Result<CowArray<'_, A, Ix2>, LapackError> {
        if self.is_cpref() {
            Ok(CowArray::from(self))
        } else {
            lapack_warn_layout_clone!(self)?;
            let owned = self.into_owned();
            Ok(CowArray::from(owned))
        }
    }

    fn to_col_layout(&self) -> Result<CowArray<'_, A, Ix2>, LapackError> {
        if self.is_fpref() {
            Ok(CowArray::from(self))
        } else {
            lapack_warn_layout_clone!(self)?;
            let owned = self.t().into_owned().reversed_axes();
            Ok(CowArray::from(owned))
        }
    }
}

pub(crate) trait ToLayoutCowArray1<A> {
    fn to_seq_layout(&self) -> Result<CowArray<'_, A, Ix1>, LapackError>;
}

impl<A> ToLayoutCowArray1<A> for ArrayView1<'_, A>
where
    A: Clone,
{
    fn to_seq_layout(&self) -> Result<CowArray<'_, A, Ix1>, LapackError> {
        let cow = self.as_standard_layout();
        if cow.is_owned() {
            lapack_warn_layout_clone!(self)?;
        }
        Ok(cow)
    }
}

pub(crate) trait ToSeqArrayOut1<'a, A> {
    fn into_seq_array_out(self) -> ArrayOut1<'a, A>;
}

impl<'a, F> ToSeqArrayOut1<'a, F> for ArrayViewOrMut1<'a, F>
where
    F: Clone,
{
    fn into_seq_array_out(self) -> ArrayOut1<'a, F> {
        match self {
            Self::View(arr) => ArrayOut::Owned(arr.to_owned()),
            Self::ViewMut(arr) => {
                if arr.is_standard_layout() {
                    ArrayOut::ViewMut(arr)
                } else {
                    lapack_warn_layout_clone!(arr);
                    let arr_own = arr.view().to_owned();
                    ArrayOut::ToBeCloned(arr, arr_own)
                }
            },
        }
    }
}

pub(crate) trait ToSeqArrayOut2<'a, A> {
    fn into_row_array_out(self) -> ArrayOut2<'a, A>;
    fn into_col_array_out(self) -> ArrayOut2<'a, A>;
}

impl<'a, F> ToSeqArrayOut2<'a, F> for ArrayViewOrMut2<'a, F>
where
    F: Clone,
{
    fn into_row_array_out(self) -> ArrayOut2<'a, F> {
        match self {
            Self::View(arr) => ArrayOut::Owned(arr.as_standard_layout().to_owned()),
            Self::ViewMut(arr) => {
                if arr.view().is_cpref() {
                    ArrayOut::ViewMut(arr)
                } else {
                    lapack_warn_layout_clone!(arr);
                    let arr_own = arr.view().as_standard_layout().to_owned();
                    ArrayOut::ToBeCloned(arr, arr_own)
                }
            },
        }
    }

    fn into_col_array_out(self) -> ArrayOut2<'a, F> {
        match self {
            Self::View(arr) => ArrayOut::Owned(arr.t().as_standard_layout().into_owned().reversed_axes()),
            Self::ViewMut(arr) => {
                if arr.view().is_fpref() {
                    ArrayOut::ViewMut(arr)
                } else {
                    lapack_warn_layout_clone!(arr);
                    let arr_own = arr.t().as_standard_layout().into_owned().reversed_axes();
                    ArrayOut::ToBeCloned(arr, arr_own)
                }
            },
        }
    }
}

/* #endregion */
