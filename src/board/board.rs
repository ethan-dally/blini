use crate::common::{bitboard::Bitboard, colour::Colour, piece::Piece, square::Square};
use enum_map::EnumMap;

#[derive(Debug, Default)]
pub struct Board {
    pieces: EnumMap<Piece, Bitboard>,
    colours: EnumMap<Colour, Bitboard>,
    mailbox: EnumMap<Square, Option<(Piece, Colour)>>,
    stm: Colour,
    castling: [bool; 4], //layout KQkq
    en_passant: Option<Square>,
    hmc: u8,
    fmn: u32,
}

impl Board {
    #[inline]
    pub fn set_stm(&mut self, colour: Colour) {
        self.stm = colour;
    }

    #[inline]
    pub fn set_hmc(&mut self, count: u8) {
        self.hmc = count;
    }

    #[inline]
    pub fn set_fmn(&mut self, count: u32) {
        self.fmn = count;
    }

    #[inline]
    pub fn set_square(&mut self, sqr: Square, colour: Colour, piece: Piece) {
        self.pieces[piece] ^= sqr.to_bb();
        self.colours[colour] ^= sqr.to_bb();
        self.mailbox[sqr] = Some((piece, colour));
    }

    #[inline]
    pub fn set_en_passant(&mut self, en_passant: Option<Square>) {
        self.en_passant = en_passant;
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
        let index = (!colour.to_bool() as usize) << 1 | !is_kingside as usize;
        self.castling[index] = change_to;
    }

    #[inline]
    pub fn get_castling(&self, colour: Colour, is_kingside: bool) -> bool {
        let index = (!colour.to_bool() as usize) << 1 | !is_kingside as usize;
        self.castling[index]
    }

    #[inline]
    pub fn fmn(&self) -> u32 {
        self.fmn
    }

    #[inline]
    pub fn hmc(&self) -> u8 {
        self.hmc
    }

    pub fn stm(&self) -> Colour {
        self.stm
    }

    #[inline]
    pub fn colours(&self, colour: Colour) -> Bitboard {
        self.colours[colour]
    }

    #[inline]
    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    #[inline]
    pub fn pieces(&self, piece: Piece) -> Bitboard {
        self.pieces[piece]
    }

    #[inline]
    pub fn all_pieces(&self) -> Bitboard {
        self.colours[Colour::White] & self.colours[Colour::Black]
    }

    #[inline]
    pub fn mailbox(&self, sqr: Square) -> Option<(Piece, Colour)> {
        self.mailbox[sqr]
    }
}

#[test]
fn castling() {
    let mut board = Board::default();
    assert!(!board.get_castling(Colour::White, true));
    assert!(!board.get_castling(Colour::Black, true));
    assert!(!board.get_castling(Colour::White, false));
    assert!(!board.get_castling(Colour::Black, false));
    board.set_castling(Colour::White, true, true);
    board.set_castling(Colour::Black, true, true);
    board.set_castling(Colour::White, false, true);
    board.set_castling(Colour::Black, false, true);
    assert!(board.get_castling(Colour::White, true));
    assert!(board.get_castling(Colour::Black, true));
    assert!(board.get_castling(Colour::White, false));
    assert!(board.get_castling(Colour::Black, false));
}
