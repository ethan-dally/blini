use std::fmt;

use crate::common::bitboard::Bitboard;

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
    H,
}

impl File {
    #[inline]
    pub const fn try_index(index: u8) -> Option<File> {
        if index > 0b0000_0111 {
            return None;
        }
        // SAFETY: guard above ensures less than 7, the size of the enum
        Some(unsafe { core::mem::transmute::<u8, File>(index) })
    }

    #[inline]
    pub const fn to_bb(self) -> Bitboard {
        Bitboard((0b0000_0001 << self as u8) * 0x101010101010101)
    }

    #[inline]
    pub const fn try_from_char(c: char) -> Option<File> {
        match c.to_ascii_uppercase() {
            'A' => Some(File::A),
            'B' => Some(File::B),
            'C' => Some(File::C),
            'D' => Some(File::D),
            'E' => Some(File::E),
            'F' => Some(File::F),
            'G' => Some(File::G),
            'H' => Some(File::H),
            _ => None,
        }
    }

    pub const ALL: [File; 8] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                File::A => "a",
                File::B => "b",
                File::C => "c",
                File::D => "d",
                File::E => "e",
                File::F => "f",
                File::G => "g",
                File::H => "h",
            }
        )
    }
}

impl From<File> for char {
    fn from(value: File) -> Self {
        match value {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

#[test]
fn file() {
    assert_eq!(File::try_index(0b0000_0000), Some(File::A));
    assert_eq!(File::try_index(0b0000_0001), Some(File::B));
    assert_eq!(File::try_index(0b0000_1000), None);
}

#[test]
fn file_to_bb() {
    let file_a_bb: u64 = 0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
    assert_eq!(File::A.to_bb(), Bitboard(file_a_bb));
    assert_eq!(File::B.to_bb(), Bitboard(file_a_bb << 1));
    assert_eq!(File::E.to_bb(), Bitboard(file_a_bb << 4));
    assert_eq!(File::H.to_bb(), Bitboard(file_a_bb << 7));
}
