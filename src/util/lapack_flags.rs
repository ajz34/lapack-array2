use crate::ffi::c_char;

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

pub type BALSOrder = LapackLayout;
pub use LapackLayout::{ColMajor as LapackColMajor, RowMajor as LapackRowMajor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LapackTranspose {
    #[default]
    Undefined = -1,
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
    ConjNoTrans = 114,
}

pub use LapackTranspose::{ConjTrans as LapackConjTrans, NoTrans as LapackNoTrans, Trans as LapackTrans};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LapackUpLo {
    #[default]
    Undefined = -1,
    Upper = 121,
    Lower = 122,
}

pub use LapackUpLo::{Lower as LapackLower, Upper as LapackUpper};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LapackDiag {
    #[default]
    Undefined = -1,
    NonUnit = 131,
    Unit = 132,
}

pub use LapackDiag::{NonUnit as LapackNonUnit, Unit as LapackUnit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LapackSide {
    #[default]
    Undefined = -1,
    Left = 141,
    Right = 142,
}

pub use LapackSide::{Left as LapackLeft, Right as LapackRight};

impl From<char> for LapackLayout {
    #[inline]
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'R' => LapackRowMajor,
            'C' => LapackColMajor,
            _ => panic!("Invalid character for LapackOrder: {}", c),
        }
    }
}

impl From<LapackLayout> for char {
    #[inline]
    fn from(layout: LapackLayout) -> Self {
        match layout {
            LapackRowMajor => 'R',
            LapackColMajor => 'C',
            _ => panic!("Invalid LapackOrder: {:?}", layout),
        }
    }
}

impl From<LapackLayout> for c_char {
    #[inline]
    fn from(layout: LapackLayout) -> Self {
        match layout {
            LapackRowMajor => 'R' as c_char,
            LapackColMajor => 'C' as c_char,
            _ => panic!("Invalid LapackOrder: {:?}", layout),
        }
    }
}

impl From<char> for LapackTranspose {
    #[inline]
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'N' => LapackNoTrans,
            'T' => LapackTrans,
            'C' => LapackConjTrans,
            _ => panic!("Invalid character for LapackTrans: {}", c),
        }
    }
}

impl From<LapackTranspose> for char {
    #[inline]
    fn from(trans: LapackTranspose) -> Self {
        match trans {
            LapackNoTrans => 'N',
            LapackTrans => 'T',
            LapackConjTrans => 'C',
            _ => panic!("Invalid LapackTrans: {:?}", trans),
        }
    }
}

impl From<LapackTranspose> for c_char {
    #[inline]
    fn from(trans: LapackTranspose) -> Self {
        match trans {
            LapackNoTrans => 'N' as c_char,
            LapackTrans => 'T' as c_char,
            LapackConjTrans => 'C' as c_char,
            _ => panic!("Invalid LapackTrans: {:?}", trans),
        }
    }
}

impl From<char> for LapackUpLo {
    #[inline]
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'U' => LapackUpper,
            'L' => LapackLower,
            _ => panic!("Invalid character for LapackUpLo: {}", c),
        }
    }
}

impl From<LapackUpLo> for char {
    #[inline]
    fn from(uplo: LapackUpLo) -> Self {
        match uplo {
            LapackUpper => 'U',
            LapackLower => 'L',
            _ => panic!("Invalid LapackUpLo: {:?}", uplo),
        }
    }
}

impl From<LapackUpLo> for c_char {
    #[inline]
    fn from(uplo: LapackUpLo) -> Self {
        match uplo {
            LapackUpper => 'U' as c_char,
            LapackLower => 'L' as c_char,
            _ => panic!("Invalid LapackUpLo: {:?}", uplo),
        }
    }
}

impl From<char> for LapackDiag {
    #[inline]
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'N' => LapackNonUnit,
            'U' => LapackUnit,
            _ => panic!("Invalid character for LapackDiag: {}", c),
        }
    }
}

impl From<LapackDiag> for char {
    #[inline]
    fn from(diag: LapackDiag) -> Self {
        match diag {
            LapackNonUnit => 'N',
            LapackUnit => 'U',
            _ => panic!("Invalid LapackDiag: {:?}", diag),
        }
    }
}

impl From<LapackDiag> for c_char {
    #[inline]
    fn from(diag: LapackDiag) -> Self {
        match diag {
            LapackNonUnit => 'N' as c_char,
            LapackUnit => 'U' as c_char,
            _ => panic!("Invalid LapackDiag: {:?}", diag),
        }
    }
}

impl From<char> for LapackSide {
    #[inline]
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'L' => LapackLeft,
            'R' => LapackRight,
            _ => panic!("Invalid character for LapackSide: {}", c),
        }
    }
}

impl From<LapackSide> for char {
    #[inline]
    fn from(side: LapackSide) -> Self {
        match side {
            LapackLeft => 'L',
            LapackRight => 'R',
            _ => panic!("Invalid LapackSide: {:?}", side),
        }
    }
}

impl From<LapackSide> for c_char {
    #[inline]
    fn from(side: LapackSide) -> Self {
        match side {
            LapackLeft => 'L' as c_char,
            LapackRight => 'R' as c_char,
            _ => panic!("Invalid LapackSide: {:?}", side),
        }
    }
}

impl LapackLayout {
    #[inline]
    pub fn flip(&self) -> Self {
        match self {
            LapackRowMajor => LapackColMajor,
            LapackColMajor => LapackRowMajor,
            _ => panic!("Invalid LapackOrder: {:?}", self),
        }
    }
}

impl LapackUpLo {
    #[inline]
    pub fn flip(&self) -> Self {
        match self {
            LapackUpper => LapackLower,
            LapackLower => LapackUpper,
            _ => panic!("Invalid LapackUpLo: {:?}", self),
        }
    }
}

impl LapackSide {
    #[inline]
    pub fn flip(&self) -> Self {
        match self {
            LapackLeft => LapackRight,
            LapackRight => LapackLeft,
            _ => panic!("Invalid LapackSide: {:?}", self),
        }
    }
}

impl LapackTranspose {
    #[inline]
    pub fn flip(&self, hermi: bool) -> Self {
        match self {
            LapackNoTrans => match hermi {
                false => LapackTrans,
                true => LapackConjTrans,
            },
            LapackTrans => LapackNoTrans,
            LapackConjTrans => LapackNoTrans,
            _ => panic!("Invalid LapackTranspose: {:?}", self),
        }
    }
}

unsafe impl Send for LapackLayout {}
unsafe impl Send for LapackTranspose {}
unsafe impl Send for LapackUpLo {}
unsafe impl Send for LapackDiag {}
unsafe impl Send for LapackSide {}

unsafe impl Sync for LapackLayout {}
unsafe impl Sync for LapackTranspose {}
unsafe impl Sync for LapackUpLo {}
unsafe impl Sync for LapackDiag {}
unsafe impl Sync for LapackSide {}

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

pub(crate) fn get_layout_row_preferred(
    by_first: &[Option<LapackLayout>],
    by_all: &[LapackLayout],
) -> LapackLayout {
    for x in by_first {
        if let Some(x) = x {
            if x.is_cpref() {
                return LapackRowMajor;
            } else if x.is_fpref() {
                return LapackColMajor;
            }
        }
    }

    if by_all.iter().all(|f| f.is_cpref()) {
        return LapackRowMajor;
    } else if by_all.iter().all(|f| f.is_fpref()) {
        return LapackColMajor;
    } else {
        return LapackRowMajor;
    }
}
