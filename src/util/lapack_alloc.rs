use crate::util::*;
use ndarray::prelude::*;

pub trait LapackAlloc1<F> {
    fn optional_alloc(
        arr: Option<ArrayViewMut1<'_, F>>,
        dim: usize,
        overwrite: bool,
    ) -> Result<ArrayOut1<'_, F>, LapackError>;
    fn optional_buffer(
        arr: Option<ArrayViewMut1<'_, F>>,
        dim: usize,
        query: bool,
    ) -> Result<(ArrayOut1<'_, F>, bool), LapackError>;
    // fn optional_buffer(arr: Option<ArrayViewMut1<'_, F>>, dim: usize) -> Result<ArrayOut1<'_, F>, LapackError>;
}

impl<F> LapackAlloc1<F> for ArrayOut1<'_, F>
where
    F: LapackFloat,
{
    /// Allocate array with dimension if not provided.
    /// If array has been provided, also check if dimension is correct.
    /// If overwrite == true, then explicitly clone; but that requires input arr not to be None.
    fn optional_alloc(
        arr: Option<ArrayViewMut1<'_, F>>,
        dim: usize,
        overwrite: bool,
    ) -> Result<ArrayOut1<'_, F>, LapackError> {
        match arr {
            Some(arr) => {
                lapack_assert_eq!(arr.dim(), dim, InvalidDim)?;
                if !overwrite {
                    let arr_clone = arr.to_owned();
                    Ok(ArrayOut::Owned(arr_clone))
                } else {
                    match arr.is_standard_layout() {
                        true => Ok(ArrayOut::ViewMut(arr)),
                        false => {
                            lapack_warn_layout_clone!(&arr)?;
                            let arr_clone = arr.to_owned();
                            Ok(ArrayOut::ToBeCloned(arr, arr_clone))
                        },
                    }
                }
            },
            None => Ok(ArrayOut::Owned(Array::zeros(dim))),
        }
    }

    /// If array has been provided, check if size is larger than given dimension.
    /// If array is not sequential or not large enough, allocate array instead of using user-given buffer.
    /// If query == true, then allocate query array.
    fn optional_buffer(
        arr: Option<ArrayViewMut1<'_, F>>,
        dim: usize,
        query: bool,
    ) -> Result<(ArrayOut1<'_, F>, bool), LapackError> {
        if arr.as_ref().is_some_and(|arr| arr.is_standard_layout() && arr.dim() > dim) {
            // if arr is larger than expected dim, do not allocate anything
            Ok((ArrayOut::ViewMut(arr.unwrap()), false))
        } else if query {
            if arr.as_ref().is_some_and(|arr| arr.dim() > 1) {
                // not a query array if arr is larger than one element, show warning message
                lapack_warn_layout_clone!(arr.as_ref().unwrap())?
            };
            Ok((ArrayOut::Owned(Array::zeros(1)), true))
        } else {
            Ok((ArrayOut::Owned(Array::zeros(dim)), false))
        }
    }
}

pub trait LapackAlloc2<F> {
    fn optional_alloc_fpref(
        arr: Option<ArrayViewMut2<'_, F>>,
        dim: (usize, usize),
        overwrite: bool,
    ) -> Result<ArrayOut2<'_, F>, LapackError>;
}

impl<F> LapackAlloc2<F> for ArrayOut2<'_, F>
where
    F: LapackFloat,
{
    /// Allocate array with dimension if not provided.
    /// If array has been provided, also check if dimension is correct.
    /// If overwrite == true, then explicitly clone; but that requires input arr not to be None.
    fn optional_alloc_fpref(
        arr: Option<ArrayViewMut2<'_, F>>,
        dim: (usize, usize),
        overwrite: bool,
    ) -> Result<ArrayOut2<'_, F>, LapackError> {
        match arr {
            Some(arr) => {
                lapack_assert_eq!(arr.dim(), dim, InvalidDim)?;
                if !overwrite {
                    let arr_clone = arr.t().as_standard_layout().t().to_owned();
                    Ok(ArrayOut::Owned(arr_clone))
                } else {
                    match arr.view().is_fpref() {
                        true => Ok(ArrayOut::ViewMut(arr)),
                        false => {
                            let arr_clone = arr.view().to_col_layout()?.into_owned();
                            Ok(ArrayOut::ToBeCloned(arr, arr_clone))
                        },
                    }
                }
            },
            None => Ok(ArrayOut::Owned(Array::zeros(dim.f()))),
        }
    }
}
