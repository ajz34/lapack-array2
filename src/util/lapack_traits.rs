use num_complex::*;
use num_traits::*;

#[allow(non_camel_case_types)]
pub type c32 = Complex<f32>;
#[allow(non_camel_case_types)]
pub type c64 = Complex<f64>;

pub trait LapackFloat:
    Num + NumAssignOps + Send + Sync + Copy + Clone + Default + core::fmt::Debug + core::fmt::Display
{
    type RealFloat: LapackFloat;
    const EPSILON: Self::RealFloat;
    fn is_complex() -> bool;
    fn conj(x: Self) -> Self;
    fn abs(x: Self) -> Self::RealFloat;
}

impl LapackFloat for f32 {
    type RealFloat = f32;
    const EPSILON: Self::RealFloat = f32::EPSILON;
    #[inline]
    fn is_complex() -> bool {
        false
    }
    #[inline]
    fn conj(x: Self) -> Self {
        x
    }
    #[inline]
    fn abs(x: Self) -> Self::RealFloat {
        x.abs()
    }
}

impl LapackFloat for f64 {
    type RealFloat = f64;
    const EPSILON: Self::RealFloat = f64::EPSILON;
    #[inline]
    fn is_complex() -> bool {
        false
    }
    #[inline]
    fn conj(x: Self) -> Self {
        x
    }
    #[inline]
    fn abs(x: Self) -> Self::RealFloat {
        x.abs()
    }
}

impl LapackFloat for c32 {
    type RealFloat = f32;
    const EPSILON: Self::RealFloat = f32::EPSILON;
    #[inline]
    fn is_complex() -> bool {
        true
    }
    #[inline]
    fn conj(x: Self) -> Self {
        x.conj()
    }
    #[inline]
    fn abs(x: Self) -> Self::RealFloat {
        x.abs()
    }
}

impl LapackFloat for c64 {
    type RealFloat = f64;
    const EPSILON: Self::RealFloat = f64::EPSILON;
    #[inline]
    fn is_complex() -> bool {
        true
    }
    #[inline]
    fn conj(x: Self) -> Self {
        x.conj()
    }
    #[inline]
    fn abs(x: Self) -> Self::RealFloat {
        x.abs()
    }
}

/// Trait marker for complex symmetry (whether it is symmetric or hermitian)
pub trait LapackSymmetric {
    type Float: LapackFloat;
    type HermitianFloat: LapackFloat;
    fn is_hermitian() -> bool;
}

/// Struct marker for symmetric matrix
pub struct LapackSymm<F>
where
    F: LapackFloat,
{
    _phantom: core::marker::PhantomData<F>,
}

impl<F> LapackSymmetric for LapackSymm<F>
where
    F: LapackFloat,
{
    type Float = F;
    type HermitianFloat = F;
    #[inline]
    fn is_hermitian() -> bool {
        false
    }
}

/// Struct marker for hermitian matrix
pub struct LapackHermi<F>
where
    F: LapackFloat,
{
    _phantom: core::marker::PhantomData<F>,
}

impl<F> LapackSymmetric for LapackHermi<F>
where
    F: LapackFloat,
{
    type Float = F;
    type HermitianFloat = <F as LapackFloat>::RealFloat;
    #[inline]
    fn is_hermitian() -> bool {
        true
    }
}

/// Marker struct of Lapack functions.
///
/// This struct will be implemented in modules of each function.
pub struct LapackFunc {}
