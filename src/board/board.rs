use enum_map::EnumMap;
use crate::common::{bitboard::Bitboard, colour::{Colour}, piece::{Piece}, square::Square};

#[derive(Debug, Default)]
pub struct Board {
    pieces: EnumMap<Piece, Bitboard>,
    colours: EnumMap<Colour, Bitboard>,
    mailbox: EnumMap<Square, Option<(Piece, Colour)>>,
    stm: Colour,

}

impl Board {

    #[inline]
    pub fn set_stm(&mut self, colour: Colour) {
        self.stm = colour;
    }

    #[inline]
    pub fn set_square(&mut self, sqr: Square, colour: Colour, piece: Piece) {
        self.pieces[piece] ^= sqr.to_bb();
        self.colours[colour] ^= sqr.to_bb();
        self.mailbox[sqr] = Some((piece, colour));
    }

    #[inline]
    pub fn clear_square(&mut self, sqr: Square) {
        if let Some((piece, colour)) = self.mailbox[sqr] {
            self.pieces[piece] ^= sqr.to_bb();
            self.colours[colour] ^= sqr.to_bb();
            self.mailbox[sqr] = None;
        }
    }

    pub fn stm(&self) -> Colour {
        self.stm
    }

    #[inline]
    pub fn colours(&self, colour: Colour) -> Bitboard {
        self.colours[colour]
    }

    #[inline]
    pub fn pieces(&self, piece: Piece) -> Bitboard {
        self.pieces[piece]
    }

    #[inline]
    pub fn mailbox(&self, sqr: Square) -> Option<(Piece, Colour)> {
        self.mailbox[sqr]
    }
}