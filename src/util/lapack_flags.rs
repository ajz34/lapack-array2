use core::ffi::c_char;

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

pub type LapackOrder = LapackLayout;
pub use LapackLayout::{ColMajor as LapackColMajor, RowMajor as LapackRowMajor};

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
