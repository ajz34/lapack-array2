use approx::*;
use lapack_array2::util::*;
use ndarray::{prelude::*, SliceInfo, SliceInfoElem};
use num_traits::*;
use rand::{thread_rng, Rng};

/* #region TestFloat and Random matrix */

pub trait TestFloat: LapackFloat {
    fn from_real(x: Self::RealFloat) -> Self;
    fn re(x: Self) -> Self::RealFloat;
    fn im(x: Self) -> Self::RealFloat;
    fn rand() -> Self;
}

impl TestFloat for f32 {
    fn from_real(x: Self::RealFloat) -> Self {
        x
    }
    fn re(x: Self) -> Self::RealFloat {
        x
    }
    fn im(x: Self) -> Self::RealFloat {
        x
    }
    fn rand() -> f32 {
        thread_rng().gen()
    }
}

impl TestFloat for f64 {
    fn from_real(x: Self::RealFloat) -> Self {
        x
    }
    fn re(x: Self) -> Self::RealFloat {
        x
    }
    fn im(x: Self) -> Self::RealFloat {
        x
    }
    fn rand() -> f64 {
        thread_rng().gen()
    }
}

impl TestFloat for c32 {
    fn from_real(x: Self::RealFloat) -> Self {
        Self::new(x, 0.0)
    }
    fn re(x: Self) -> Self::RealFloat {
        x.re
    }
    fn im(x: Self) -> Self::RealFloat {
        x.im
    }
    fn rand() -> c32 {
        let re = thread_rng().gen();
        let im = thread_rng().gen();
        c32::new(re, im)
    }
}

impl TestFloat for c64 {
    fn from_real(x: Self::RealFloat) -> Self {
        Self::new(x, 0.0)
    }
    fn re(x: Self) -> Self::RealFloat {
        x.re
    }
    fn im(x: Self) -> Self::RealFloat {
        x.im
    }
    fn rand() -> c64 {
        let re = thread_rng().gen();
        let im = thread_rng().gen();
        c64::new(re, im)
    }
}

pub fn random_matrix<F>(row: usize, col: usize, layout: LapackLayout) -> Array2<F>
where
    F: TestFloat + LapackFloat,
{
    let mut matrix = match layout {
        LapackRowMajor => Array2::zeros((row, col)),
        LapackColMajor => Array2::zeros((row, col).f()),
        _ => panic!("Invalid layout"),
    };
    for x in matrix.iter_mut() {
        *x = F::rand();
    }
    return matrix;
}

pub fn random_array<F>(size: usize) -> Array1<F>
where
    F: TestFloat + LapackFloat,
{
    let mut array = Array1::zeros(size);
    for x in array.iter_mut() {
        *x = F::rand();
    }
    return array;
}

/* #endregion */

/* #region Sized subatrix */

pub fn slice(nrow: usize, ncol: usize, srow: usize, scol: usize) -> SliceInfo<[SliceInfoElem; 2], Ix2, Ix2> {
    s![5..(5+nrow*srow);srow, 10..(10+ncol*scol);scol]
}

pub fn slice_1d(n: usize, s: usize) -> SliceInfo<[SliceInfoElem; 1], Ix1, Ix1> {
    s![50..(50+n*s);s]
}

/* #endregion */

/* #region Basic matrix operations */

pub fn gemm<F>(a: &ArrayView2<F>, b: &ArrayView2<F>) -> Array2<F>
where
    F: LapackFloat,
{
    let (m, k) = a.dim();
    let n = b.len_of(Axis(1));
    assert_eq!(b.len_of(Axis(0)), k);
    let mut c = Array2::<F>::zeros((m, n));
    for i in 0..m {
        for j in 0..n {
            let mut sum = F::zero();
            for l in 0..k {
                sum += a[[i, l]] * b[[l, j]];
            }
            c[[i, j]] = sum;
        }
    }
    return c;
}

pub fn gemv<F>(a: &ArrayView2<F>, x: &ArrayView1<F>) -> Array1<F>
where
    F: LapackFloat,
{
    let (m, n) = a.dim();
    assert_eq!(x.len(), n);
    let mut y = Array1::<F>::zeros(m);
    for i in 0..m {
        let mut sum = F::zero();
        for j in 0..n {
            sum += a[[i, j]] * x[j];
        }
        y[i] = sum;
    }
    return y;
}

pub fn transpose<F>(a: &ArrayView2<F>, trans: char) -> Array2<F>
where
    F: LapackFloat,
{
    match trans {
        'N' => a.into_owned(),
        'T' => a.t().into_owned(),
        'C' => match F::is_complex() {
            true => {
                let a = a.t().into_owned();
                a.mapv(|x| F::conj(x))
            },
            false => a.t().into_owned(),
        },
        _ => panic!("Invalid LapackTrans"),
    }
}

pub fn symmetrize<F>(a: &ArrayView2<F>, uplo: char) -> Array2<F>
where
    F: LapackFloat,
{
    let mut a = a.into_owned();
    if uplo == 'L' {
        for i in 0..a.len_of(Axis(0)) {
            for j in 0..i {
                a[[j, i]] = a[[i, j]];
            }
        }
    } else if uplo == 'U' {
        for i in 0..a.len_of(Axis(0)) {
            for j in i + 1..a.len_of(Axis(1)) {
                a[[j, i]] = a[[i, j]];
            }
        }
    }
    return a;
}

pub fn hermitianize<F>(a: &ArrayView2<F>, uplo: char) -> Array2<F>
where
    F: TestFloat,
{
    let mut a = a.into_owned();
    if uplo == 'L' {
        for i in 0..a.len_of(Axis(0)) {
            a[[i, i]] = F::from_real(F::re(a[[i, i]]));
            for j in 0..i {
                a[[j, i]] = F::conj(a[[i, j]]);
            }
        }
    } else if uplo == 'U' {
        for i in 0..a.len_of(Axis(0)) {
            a[[i, i]] = F::from_real(F::re(a[[i, i]]));
            for j in i + 1..a.len_of(Axis(1)) {
                a[[j, i]] = F::conj(a[[i, j]]);
            }
        }
    }
    return a;
}

pub fn tril_assign<F>(c: &mut ArrayViewMut2<F>, a: &ArrayView2<F>, uplo: char)
where
    F: LapackFloat,
{
    if uplo == 'L' {
        for i in 0..a.len_of(Axis(0)) {
            for j in 0..=i {
                c[[i, j]] = a[[i, j]];
            }
        }
    } else if uplo == 'U' {
        for i in 0..a.len_of(Axis(0)) {
            for j in i..a.len_of(Axis(1)) {
                c[[i, j]] = a[[i, j]];
            }
        }
    }
}

pub fn unpack_tril<F>(ap: &ArrayView1<F>, a: &mut ArrayViewMut2<F>, layout: char, uplo: char)
where
    F: LapackFloat,
{
    let n = a.len_of(Axis(0));
    if layout == 'R' {
        let mut k = 0;
        if uplo == 'L' {
            for i in 0..n {
                for j in 0..=i {
                    a[[i, j]] = ap[k];
                    k += 1;
                }
            }
        } else if uplo == 'U' {
            for i in 0..n {
                for j in i..n {
                    a[[i, j]] = ap[k];
                    k += 1;
                }
            }
        }
    } else if layout == 'C' {
        let mut k = 0;
        if uplo == 'U' {
            for j in 0..n {
                for i in 0..=j {
                    a[[i, j]] = ap[k];
                    k += 1;
                }
            }
        } else if uplo == 'L' {
            for j in 0..n {
                for i in j..n {
                    a[[i, j]] = ap[k];
                    k += 1;
                }
            }
        }
    }
}

pub fn check_same<F, D>(a: &ArrayView<F, D>, b: &ArrayView<F, D>, eps: <F::RealFloat as AbsDiffEq>::Epsilon)
where
    F: LapackFloat,
    D: Dimension,
    <F as LapackFloat>::RealFloat: approx::AbsDiffEq,
{
    let err: F::RealFloat = (a - b).mapv(F::abs).sum();
    let acc: F::RealFloat = a.mapv(F::abs).sum();
    let err_div = err / acc;
    assert_abs_diff_eq!(err_div, F::RealFloat::zero(), epsilon = eps);
}

/* #endregion */

/* #region array alignment */

pub fn ndarray_to_colmajor<A, D>(arr: Array<A, D>) -> Array<A, D>
where
    A: Clone,
    D: Dimension,
{
    let arr = arr.reversed_axes(); // data not copied
    if arr.is_standard_layout() {
        // arr is f-contiguous = reversed arr is c-contiguous
        // CowArray `into_owned` will not copy if own data, but will copy if it represents view
        // So, though `arr.as_standard_layout().reversed_axes().into_owned()` works, it clones data instead of move it
        return arr.reversed_axes(); // data not copied
    } else {
        // arr is not f-contiguous
        // make reversed arr c-contiguous, then reverse arr again
        return arr.as_standard_layout().reversed_axes().into_owned();
    }
}

pub fn ndarray_to_rowmajor<A, D>(arr: Array<A, D>) -> Array<A, D>
where
    A: Clone,
    D: Dimension,
{
    if arr.is_standard_layout() {
        return arr;
    } else {
        return arr.as_standard_layout().into_owned();
    }
}

pub fn ndarray_to_layout<A, D>(arr: Array<A, D>, layout: char) -> Array<A, D>
where
    A: Clone,
    D: Dimension,
{
    match layout {
        'R' => ndarray_to_rowmajor(arr),
        'C' => ndarray_to_colmajor(arr),
        _ => panic!("invalid layout"),
    }
}

/* #endregion */

/* #region compare */

pub fn allclose_epsilon<F, D>(v1: &ArrayView<'_, F, D>, v2: &ArrayView<'_, F, D>, epsilon: f64)
where
    F: TestFloat,
    D: Dimension,
{
    let err = (v1 - v2).mapv(F::abs).sum().to_f64().unwrap();
    let acc = v2.mapv(F::abs).sum().to_f64().unwrap();
    let err_div = err / (acc + epsilon);
    assert_relative_eq!(err_div.to_f64().unwrap(), 0.0, epsilon = 1.0e-10);
}

/* #endregion */
