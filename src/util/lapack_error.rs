#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

use alloc::string::String;
use core::num::TryFromIntError;
use derive_builder::UninitializedFieldError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LapackError {
    OverflowDimension(String),
    InvalidDim(String),
    InvalidFlag(String),
    FailedCheck(String),
    UninitializedField(&'static str),
    ExplicitCopy(String),
    RuntimeError(String),
    Info(i64),
    Miscellaneous(String),
}

/* #region impl LapackError */

#[cfg(feature = "std")]
impl std::error::Error for LapackError {}

impl From<UninitializedFieldError> for LapackError {
    fn from(e: UninitializedFieldError) -> LapackError {
        LapackError::UninitializedField(e.field_name())
    }
}

impl From<TryFromIntError> for LapackError {
    fn from(_: TryFromIntError) -> LapackError {
        LapackError::OverflowDimension(String::from("TryFromIntError"))
    }
}

impl core::fmt::Display for LapackError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/* #endregion */

/* #region macros */

#[macro_export]
macro_rules! lapack_assert {
    ($cond:expr, $errtype:ident, $($arg:tt)*) => {
        if $cond {
            Ok(())
        } else {
            extern crate alloc;
            use alloc::string::String;
            Err(LapackError::$errtype(String::from(concat!(
                file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype), " : ",
                $($arg),*, ": ", stringify!($cond)
            ))))
        }
    };
    ($cond:expr, $errtype:ident) => {
        if $cond {
            Ok(())
        } else {
            extern crate alloc;
            use alloc::string::String;
            Err(LapackError::$errtype(String::from(concat!(
                file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype), " : ",
                stringify!($cond)
            ))))
        }
    };
}

#[macro_export]
macro_rules! lapack_assert_eq {
    ($a:expr, $b:expr, $errtype:ident) => {
        if $a == $b {
            Ok(())
        } else {
            extern crate alloc;
            use alloc::string::String;
            use core::fmt::Write;
            let mut s = String::from(concat!(
                file!(),
                ":",
                line!(),
                ": ",
                "LapackError::",
                stringify!($errtype),
                " : "
            ));
            write!(s, "{:?} = {:?} not equal to {:?} = {:?}", stringify!($a), $a, stringify!($b), $b)
                .unwrap();
            Err(LapackError::$errtype(s))
        }
    };
}

#[macro_export]
macro_rules! lapack_raise {
    ($errtype:ident) => {{
        extern crate alloc;
        use alloc::string::String;
        Err(LapackError::$errtype(String::from(concat!(
            file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype)
        ))))
    }};
    ($errtype:ident, $($arg:tt)*) => {{
        extern crate alloc;
        use alloc::string::String;
        Err(LapackError::$errtype(String::from(concat!(
            file!(), ":", line!(), ": ", "LapackError::", stringify!($errtype), " : ",
            $($arg),*
        ))))
    }};
}

#[macro_export]
macro_rules! lapack_invalid {
    ($word:expr) => {{
        extern crate alloc;
        use alloc::string::String;
        use core::fmt::Write;
        let mut s = String::from(concat!(file!(), ":", line!(), ": ", "LapackError::InvalidFlag", " : "));
        write!(s, "{:?} = {:?}", stringify!($word), $word).unwrap();
        Err(LapackError::InvalidFlag(s))
    }};
    ($word:expr, $keyword:tt) => {{
        extern crate alloc;
        use alloc::string::String;
        use core::fmt::Write;
        let mut s =
            String::from(concat!(file!(), ":", line!(), ": ", "LapackError::InvalidFlag ", $keyword, " : "));
        write!(s, "{:?} = {:?}", stringify!($word), $word).unwrap();
        Err(LapackError::InvalidFlag(s))
    }};
}

#[macro_export]
macro_rules! lapack_check_flag {
    ($word:expr, [$($poss:expr),+]) => {{
        extern crate alloc;
        use alloc::string::String;
        use core::fmt::Write;
        match $word {
            $($poss)|+ => Ok(()),
            _ => {
                let mut s = String::from(concat!(file!(), ":", line!(), ": ", "LapackError::InvalidFlag", " : "));
                write!(s, "{:?} = {:?}, where valid values are {:?}", stringify!($word), $word, [$($poss),+]).unwrap();
                Err(LapackError::InvalidFlag(s))
            }
        }
    }};
}

/* #endregion */

/* #region macros (warning) */

#[macro_export]
macro_rules! lapack_warn_layout_clone {
    ($array:expr) => {{
        #[cfg(feature = "std")]
        extern crate std;

        if cfg!(all(feature = "std", feature = "warn_on_copy")) {
            std::eprintln!(
                "Warning: Copying array due to non-standard layout, shape={:?}, strides={:?}",
                $array.shape(),
                $array.strides()
            );
            Result::<(), LapackError>::Ok(())
        } else if cfg!(feature = "error_on_copy") {
            lapack_raise!(ExplicitCopy)
        } else {
            Result::<(), LapackError>::Ok(())
        }
    }};
    ($array:expr, $msg:tt) => {{
        #[cfg(feature = "std")]
        extern crate std;

        if cfg!(all(feature = "std", feature = "warn_on_copy")) {
            std::eprintln!("Warning: {:?}, shape={:?}, strides={:?}", $msg, $array.shape(), $array.strides());
            Result::<(), LapackError>::Ok(())
        } else if cfg!(feature = "error_on_copy") {
            lapack_raise!(ExplicitCopy)
        } else {
            Result::<(), LapackError>::Ok(())
        }
    }};
}

/* #endregion */

#[macro_export]
macro_rules! lapack_info {
    ($info:expr) => {
        if $info == 0 {
            Ok(())
        } else {
            Err(LapackError::Info($info as i64))
        }
    };
}

/* #endregion */
