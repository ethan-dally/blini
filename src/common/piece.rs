use enum_map::Enum;

use crate::common::colour::Colour;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }
}

impl Piece {
    pub fn display(&self, colour: Colour) -> char {
        let c = match &self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        };
        match colour {
            Colour::White => c.to_ascii_uppercase(),
            Colour::Black => c,
        }
    }

    pub const ALL: [Piece; 6] = {
        [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ]
    };
}

impl TryFrom<char> for Piece {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'p' => Ok(Piece::Pawn),
            'n' => Ok(Piece::Knight),
            'b' => Ok(Piece::Bishop),
            'r' => Ok(Piece::Rook),
            'q' => Ok(Piece::Queen),
            'k' => Ok(Piece::King),
            _ => Err("invalid char".to_string()),
        }
    }
}

impl TryFrom<&str> for Piece {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &*value.to_ascii_lowercase() {
            "p" => Ok(Piece::Pawn),
            "n" => Ok(Piece::Knight),
            "b" => Ok(Piece::Bishop),
            "r" => Ok(Piece::Rook),
            "q" => Ok(Piece::Queen),
            "k" => Ok(Piece::King),
            _ => Err("invalid piece string".to_string()),
        }
    }
}
