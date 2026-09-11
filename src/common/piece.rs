use crate::common::colour::Colour;
use enum_map::Enum;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Enum)]
pub enum Piece {
    Pawn,
    Castle,
    Knight,
    Bishop,
    Queen,
    King,
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Pawn => 'p',
            Piece::Castle => 'c',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }
}

impl Piece {
    fn display(&self, colour: Colour) -> char {
        let c = match self {
            Piece::Pawn => 'p',
            Piece::Castle => 'c',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Queen => 'q',
            Piece::King => 'k',
        };
        match colour {
            Colour::White => c.to_ascii_uppercase(),
            Colour::Black => c,
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'p' => Ok(Piece::Pawn),
            'c' => Ok(Piece::Castle),
            'n' => Ok(Piece::Knight),
            'b' => Ok(Piece::Bishop),
            'q' => Ok(Piece::Queen),
            'k' => Ok(Piece::King),
            _ => Err("invalid char".to_string()),
        }
    }
}
