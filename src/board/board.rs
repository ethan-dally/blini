use enum_map::EnumMap;
use crate::common::{bitboard::Bitboard, colour::{self, Colour}, piece::{self, Piece}, square::Square};

pub struct Board {
    pieces: EnumMap<Piece, Bitboard>,
    colours: EnumMap<Colour, Bitboard>
}

impl Board {
    #[inline]
    fn set_square(&mut self, sqr: Square, colour: Colour, piece: Piece) {
        // self.pieces[piece]
    }
}