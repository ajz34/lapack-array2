#[cfg(feature = "std")]
extern crate std;

use blas_array2::prelude::BLASError;
pub type LapackError = BLASError;

/* #region macros */

#[macro_export]
macro_rules! lapack_assert {
    ($cond:expr, $errtype:ident, $($arg:tt)*) => {
        if $cond {
            Ok(())
        } else {
            Err(LapackError::$errtype(concat!(
                file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype), " : ",
                $($arg),*, ": ", stringify!($cond)
            )))
        }
    };
    ($cond:expr, $errtype:ident) => {
        if $cond {
            Ok(())
        } else {
            Err(LapackError::$errtype(concat!(
                file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype), " : ",
                stringify!($cond)
            )))
        }
    };
}

#[macro_export]
macro_rules! lapack_assert_eq {
    ($a:expr, $b:expr, $errtype:ident, $($arg:tt)*) => {
        if $a == $b {
            Ok(())
        } else {
            Err(LapackError::$errtype(concat!(
                file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype), " : ",
                $($arg),*, ": ", stringify!($a), " = ", stringify!($b)
            )))
        }
    };
    ($a:expr, $b:expr, $errtype:ident) => {
        if $a == $b {
            Ok(())
        } else {
            Err(LapackError::$errtype(concat!(
                file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype), " : ",
                stringify!($a), " = ", stringify!($b)
            )))
        }
    };
}

#[macro_export]
macro_rules! lapack_raise {
    ($errtype:ident) => {
        Err(LapackError::$errtype(concat!(
            file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype)
        )))
    };
    ($errtype:ident, $($arg:tt)*) => {
        Err(LapackError::$errtype(concat!(
            file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype), " : ",
            $($arg),*
        )))
    };
}

#[macro_export]
macro_rules! lapack_invalid {
    ($word:expr) => {
        Err(LapackError::InvalidFlag(concat!(
            file!(),
            ":",
            line!(),
            ": ",
            "LapackError::InvalidFlag",
            " : ",
            stringify!($word)
        )))
    };
}

/* #endregion */
