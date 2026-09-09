use std::fmt;

#[repr(u8)] 
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl File {
    #[inline]
    pub const fn try_index(index: u8) -> Option<File> {
        if index > 0b0000_0111 {return None;}
        Some(unsafe { core::mem::transmute::<u8, File>(index) })
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}