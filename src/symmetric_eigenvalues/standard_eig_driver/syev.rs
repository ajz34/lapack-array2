use crate::ffi::{self, c_char, lapack_int};
use crate::{lapack_info, util::*};
use core::cmp::max;
use derive_builder::Builder;
use ndarray::prelude::*;

/* #region Lapack driver */

pub struct SYEV_Driver<'a, 'w, 'work, 'rwork, F>
where
    F: LapackFloat,
{
    jobz: c_char,
    uplo: c_char,
    n: lapack_int,
    a: ArrayOut2<'a, F>,
    lda: lapack_int,
    w: ArrayOut1<'w, F::RealFloat>,
    work: ArrayOut1<'work, F>,
    lwork: lapack_int,
    rwork: ArrayOut1<'rwork, F::RealFloat>,
    info: lapack_int,
}

pub trait SYEVNum: LapackFloat {
    unsafe fn run_syev(driver: &mut SYEV_Driver<Self>) -> Result<(), LapackError>;
}

macro_rules! impl_func_real {
    ($type:ty, $func:ident) => {
        impl SYEVNum for $type {
            unsafe fn run_syev(driver: &mut SYEV_Driver<Self>) -> Result<(), LapackError> {
                ffi::$func(
                    &driver.jobz,
                    &driver.uplo,
                    &driver.n,
                    driver.a.get_data_mut_ptr(),
                    &driver.lda,
                    driver.w.get_data_mut_ptr(),
                    driver.work.get_data_mut_ptr(),
                    &driver.lwork,
                    &mut driver.info,
                );
                lapack_info!(driver.info)?;
                return Ok(());
            }
        }
    };
}

macro_rules! impl_func_comp {
    ($type:ty, $func:ident) => {
        impl SYEVNum for $type {
            unsafe fn run_syev(driver: &mut SYEV_Driver<Self>) -> Result<(), LapackError> {
                ffi::$func(
                    &driver.jobz,
                    &driver.uplo,
                    &driver.n,
                    driver.a.get_data_mut_ptr(),
                    &driver.lda,
                    driver.w.get_data_mut_ptr(),
                    driver.work.get_data_mut_ptr(),
                    &driver.lwork,
                    driver.rwork.get_data_mut_ptr(),
                    &mut driver.info,
                );
                lapack_info!(driver.info)?;
                return Ok(());
            }
        }
    };
}

impl_func_real!(f32, ssyev_);
impl_func_real!(f64, dsyev_);
impl_func_comp!(c32, cheev_);
impl_func_comp!(c64, zheev_);

/* #endregion */

/* #region Lapack builder */

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(error = "LapackError"), no_std)]
pub struct SYEV_<'a, 'w, 'work, 'rwork, F>
where
    F: SYEVNum,
{
    // input, input/output
    #[builder(setter(into))]
    pub a: ArrayViewOrMut2<'a, F>,

    // output
    #[builder(setter(strip_option), default = "None")]
    pub w: Option<ArrayViewMut1<'w, F::RealFloat>>,

    // option
    #[builder(default = "'V'")]
    pub jobz: char,
    #[builder(default = "'U'")]
    pub uplo: char,

    // buffer
    #[builder(setter(strip_option), default = "None")]
    pub work: Option<ArrayViewMut1<'work, F>>,
    #[builder(setter(strip_option), default = "None")]
    pub rwork: Option<ArrayViewMut1<'rwork, F::RealFloat>>,
}

impl<'a, 'w, 'work, 'rwork, F> SYEV_<'a, 'w, 'work, 'rwork, F>
where
    F: SYEVNum,
{
    pub fn driver(self) -> Result<(ArrayOut2<'a, F>, ArrayOut1<'w, F::RealFloat>), LapackError> {
        let Self { a, w, jobz, uplo, work, rwork } = self;

        // 1. dim assign, flag check
        let a = a.into_col_array_out();
        let n = a.view().nrows();
        let jobz = jobz.to_ascii_uppercase();
        let uplo = uplo.to_ascii_uppercase();
        lapack_check_flag!(jobz, ['N', 'V'])?;
        lapack_check_flag!(uplo, ['U', 'L'])?;

        // 2. dim check & alloc
        lapack_assert_eq!(a.view().dim(), (n, n), InvalidDim)?;
        let w = ArrayOut1::optional_alloc(w, n, true)?;
        let lda = a.view().stride_of(Axis(1));

        // 3. buffer check & alloc
        let _lwork_size = max(2 * n as isize - 1, 1) as usize;
        let _rwork_size = max(3 * n as isize - 2, 1) as usize;
        let (work, query_work) = ArrayOut1::optional_buffer(work, _lwork_size, true)?;
        let (rwork, _) = ArrayOut1::optional_buffer(rwork, _rwork_size, false)?;
        let lwork = if query_work { -1 } else { work.view().len() as isize };

        // 4. struct build
        let mut driver = SYEV_Driver {
            jobz: jobz as c_char,
            uplo: uplo as c_char,
            n: n.try_into()?,
            a,
            lda: lda.try_into()?,
            w,
            work,
            lwork: lwork.try_into()?,
            rwork,
            info: 0,
        };

        // 5. buffer query
        if query_work {
            unsafe {
                F::run_syev(&mut driver)?;
            }
            if query_work {
                let lwork = F::ftoi(driver.work.view()[0]);
                driver.lwork = lwork.try_into()?;
                driver.work = ArrayOut1::optional_alloc(None, lwork as usize, false)?;
            }
        }

        // 6. perform comput
        unsafe {
            F::run_syev(&mut driver)?;
        }

        // 7. finalize
        let SYEV_Driver { a, w, .. } = driver;
        return Ok((a.clone_to_view_mut(), w.clone_to_view_mut()));
    }
}

/* #endregion */

/* #region Lapack wrapper */

pub type SYEV<'a, 'w, 'work, 'rwork, F> = SYEV_Builder<'a, 'w, 'work, 'rwork, F>;
pub type HEEV<'a, 'w, 'work, 'rwork, F> = SYEV_Builder<'a, 'w, 'work, 'rwork, F>;
pub type SSYEV<'a, 'w, 'work, 'rwork> = SYEV<'a, 'w, 'work, 'rwork, f32>;
pub type DSYEV<'a, 'w, 'work, 'rwork> = SYEV<'a, 'w, 'work, 'rwork, f64>;
pub type CHEEV<'a, 'w, 'work, 'rwork> = HEEV<'a, 'w, 'work, 'rwork, c32>;
pub type ZHEEV<'a, 'w, 'work, 'rwork> = HEEV<'a, 'w, 'work, 'rwork, c64>;

impl<'a, 'w, 'work, 'rwork, F> SYEV_Builder<'a, 'w, 'work, 'rwork, F>
where
    F: SYEVNum,
{
    pub fn run(self) -> Result<(ArrayOut2<'a, F>, ArrayOut1<'w, F::RealFloat>), LapackError> {
        let obj = self.build()?;
        if obj.a.view().is_fpref() {
            return obj.driver();
        } else {
            let (a, w) = obj.driver()?;
            let a = match a {
                ArrayOut::Owned(a) => ArrayOut::Owned(a.as_standard_layout().into_owned()),
                ArrayOut::ViewMut(a) => ArrayOut::ViewMut(a),
                _ => lapack_raise!(RuntimeError)?,
            };
            return Ok((a, w));
        }
    }
}

/* #endregion */
