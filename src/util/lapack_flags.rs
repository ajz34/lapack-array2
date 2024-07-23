use crate::util::*;
use crate::{ffi::c_char, lapack_invalid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LapackLayout {
    #[default]
    Undefined = -1,
    RowMajor = 101,
    ColMajor = 102,
    // extension of current crate
    Sequential = 103,
    NonContiguous = 104,
}

pub type BLASOrder = LapackLayout;
pub use LapackLayout::{ColMajor as LapackColMajor, RowMajor as LapackRowMajor};

impl From<char> for LapackLayout {
    #[inline]
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'R' => LapackRowMajor,
            'C' => LapackColMajor,
            _ => Self::Undefined,
        }
    }
}

impl TryFrom<LapackLayout> for char {
    type Error = LapackError;
    #[inline]
    fn try_from(layout: LapackLayout) -> Result<Self, LapackError> {
        match layout {
            LapackRowMajor => Ok('R'),
            LapackColMajor => Ok('C'),
            _ => lapack_invalid!(layout),
        }
    }
}

impl TryFrom<LapackLayout> for c_char {
    type Error = LapackError;
    #[inline]
    fn try_from(layout: LapackLayout) -> Result<Self, LapackError> {
        match layout {
            LapackRowMajor => Ok('R' as c_char),
            LapackColMajor => Ok('C' as c_char),
            _ => lapack_invalid!(layout),
        }
    }
}

impl LapackLayout {
    #[inline]
    pub fn flip(&self) -> Result<Self, LapackError> {
        match self {
            LapackRowMajor => Ok(LapackColMajor),
            LapackColMajor => Ok(LapackRowMajor),
            _ => lapack_invalid!(self),
        }
    }
}

unsafe impl Send for LapackLayout {}
unsafe impl Sync for LapackLayout {}

impl LapackLayout {
    #[inline]
    pub fn is_cpref(&self) -> bool {
        match self {
            LapackRowMajor => true,
            LapackLayout::Sequential => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_fpref(&self) -> bool {
        match self {
            LapackColMajor => true,
            LapackLayout::Sequential => true,
            _ => false,
        }
    }
}

pub fn flip_uplo(uplo: char) -> Result<char, LapackError> {
    Ok(match uplo {
        'U' => 'L',
        'L' => 'U',
        _ => lapack_invalid!(uplo, "UpLo")?,
    })
}

pub fn flip_side(side: char) -> Result<char, LapackError> {
    Ok(match side {
        'R' => 'L',
        'L' => 'R',
        _ => lapack_invalid!(side, "Side")?,
    })
}

pub fn flip_layout(layout: char) -> Result<char, LapackError> {
    Ok(match layout {
        'R' => 'C',
        'C' => 'R',
        _ => lapack_invalid!(layout, "Layout")?,
    })
}

pub fn flip_trans(trans: char, hermi: bool) -> Result<char, LapackError> {
    Ok(match trans {
        'N' => {
            if hermi {
                'C'
            } else {
                'T'
            }
        },
        'T' => 'N',
        'C' => 'N',
        _ => lapack_invalid!(trans, "Trans")?,
    })
}
