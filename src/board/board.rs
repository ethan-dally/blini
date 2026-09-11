use crate::common::{bitboard::Bitboard, colour::Colour, piece::Piece, square::Square};
use enum_map::EnumMap;

#[derive(Debug, Default)]
pub struct Board {
    pieces: EnumMap<Piece, Bitboard>,
    colours: EnumMap<Colour, Bitboard>,
    mailbox: EnumMap<Square, Option<(Piece, Colour)>>,
    stm: Colour,
    castling: [bool; 4],
    en_passant: Option<Square>,
    hmc: u8,
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

    #[inline]
    pub fn set_castling(&mut self, colour: Colour, is_kingside: bool, change_to: bool) {
        let index = (!colour.to_bool() as usize) << 1 & is_kingside as usize;
        self.castling[index] = change_to;
    }

    #[inline]
    pub fn get_castling(&self, colour: Colour, is_kingside: bool) -> bool {
        let index = (!colour.to_bool() as usize) << 1 & is_kingside as usize;
        self.castling[index]
    }

    pub fn set_en_passant(&mut self, en_passant: Option<Square>) {
        self.en_passant = en_passant;
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

#[test]
fn castling() {
    let mut board = Board::default();
    assert_eq!(board.get_castling(Colour::White, true), false);
    assert_eq!(board.get_castling(Colour::Black, true), false);
    assert_eq!(board.get_castling(Colour::White, false), false);
    assert_eq!(board.get_castling(Colour::Black, false), false);
    board.set_castling(Colour::White, true, true);
    board.set_castling(Colour::Black, true, true);
    board.set_castling(Colour::White, false, true);
    board.set_castling(Colour::Black, false, true);
    assert_eq!(board.get_castling(Colour::White, true), true);
    assert_eq!(board.get_castling(Colour::Black, true), true);
    assert_eq!(board.get_castling(Colour::White, false),true);
    assert_eq!(board.get_castling(Colour::Black, false),true);
}