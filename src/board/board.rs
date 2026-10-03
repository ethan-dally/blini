use enum_map::EnumMap;

use crate::{board::zobrist::Zobrist, common::{bitboard::Bitboard, colour::Colour, piece::Piece, square::Square}};

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub(super) pieces: EnumMap<Piece, Bitboard>,
    pub(super) colours: EnumMap<Colour, Bitboard>,
    pub(super) mailbox: EnumMap<Square, Option<(Piece, Colour)>>,
    pub(super) stm: Colour,
    pub(super) castling: Castling,
    pub(super) en_passant: Option<Square>,
    pub(super) hmc: u8,
    pub(super) fmn: u16,
    pub(super) zobrist: u64,
}

#[allow(dead_code)]
impl Board {

    #[inline]
    pub fn empty() -> Board {
        Board { 
            pieces: EnumMap::default(),
            colours: EnumMap::default(), 
            mailbox: EnumMap::default(),
            stm: Colour::White,
            castling: Castling::EMPTY,
            en_passant: None,
            hmc: 0,
            fmn: 0, 
            zobrist: Zobrist::Castling(Castling::EMPTY).get()
        }
    }

    #[inline]
    pub fn startpos() -> Board {
        Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("hardcoded startpos fen failed to parse")
    }

    #[inline]
    pub fn set_stm(&mut self, colour: Colour) {
        self.stm = colour;
    }

    #[inline]
    pub fn set_hmc(&mut self, count: u8) {
        self.hmc = count;
    }

    #[inline]
    pub fn set_fmn(&mut self, count: u16) {
        self.fmn = count;
    }

    #[inline]
    pub fn set_square(&mut self, sqr: Square, colour: Colour, piece: Piece) {
        if let Some((old_piece, old_colour)) = self.mailbox[sqr].take() {
            let bb = sqr.to_bb();
            self.pieces[old_piece] ^= bb;
            self.colours[old_colour] ^= bb;
        }
        let bb = sqr.to_bb();
        self.pieces[piece] |= bb;
        self.colours[colour] |= bb;
        self.mailbox[sqr] = Some((piece, colour));
    }

    /// assumption of piece existing, since used in `make_move` code
    #[inline]
    pub fn remove_piece(&mut self, sqr: Square) -> Piece {
        let (piece, colour) = self.mailbox[sqr]
            .take()
            .expect("remove_piece called on empty square");
        let bb = sqr.to_bb();
        self.pieces[piece] ^= bb;
        self.colours[colour] ^= bb;
        piece
    }

    #[inline]
    pub fn try_remove_piece(&mut self, sqr: Square) -> Option<Piece> {
        if self.mailbox[sqr].is_some() {
            return Some(self.remove_piece(sqr));
        } 
        None
    }

    #[inline]
    /// returns old value
    pub fn set_en_passant(&mut self, en_passant: Option<Square>) -> Option<Square> {
        let old = self.en_passant;
        self.en_passant = en_passant;
        old
    }

    #[inline]
    pub fn set_castling(&mut self, colour: Colour, is_kingside: bool, change_to: bool) {
        self.castling.set_castling(colour, is_kingside, change_to);
    }

    #[inline]
    pub fn get_castling(&self, colour: Colour, is_kingside: bool) -> bool {
        self.castling.get_castling(colour, is_kingside)
    }

    #[inline]
    pub fn fmn(&self) -> u16 {
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
    pub fn us_pieces(&self) -> Bitboard {
        self.colours[self.stm]
    }

    #[inline]
    pub fn them_pieces(&self) -> Bitboard {
        self.colours[!self.stm]
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
        self.colours[Colour::White] | self.colours[Colour::Black]
    }

    #[inline]
    pub fn mailbox(&self, sqr: Square) -> Option<(Piece, Colour)> {
        self.mailbox[sqr]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Castling(u8);

impl Castling {

    #[inline]
    pub fn from_u8(raw_u8: u8) -> Castling {
        Castling(raw_u8)
    }

    #[inline]
    pub fn get_castling(&self, colour: Colour, is_kingside: bool) -> bool {
        let index = colour as u8 | (is_kingside as u8) << 1;
        let mask = 0b1 << index;
        self.0 & mask != 0
    }

    #[inline]
    pub fn set_castling(&mut self, colour: Colour, is_kingside: bool, change_to: bool) {
        let index = colour as u8 | (is_kingside as u8) << 1;
        let mask = 0b1 << index;
        if change_to {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }

    #[inline]
    pub fn value(&self) -> u8 {
        self.0
    }

    const EMPTY: Castling = {
        Castling(0)
    };

    const FULL: Castling = {
        Castling(0xF)
    };
}