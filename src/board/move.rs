use enum_map::Enum;

use crate::{board::{board::Board, movegen::MoveList}, common::{bitboard::Bitboard, colour::Colour, direction::{NorthEast, NorthWest, South}, file::File, magics::magic_table, masks::{king_mask, knight_mask}, piece::Piece, rank::Rank, square::Square}};

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub(super) src: Square,
    pub(super) dst: Square,
    pub(super) flag: MoveFlag,
}

impl Move {
    #[inline]
    pub fn new(src: Square, dst: Square, flag: MoveFlag) -> Move {
        Move { src, dst, flag}
    }

    pub fn display(&self) {
        println!("s: {}, d: {}, f: {:?}", self.src, self.dst, self.flag);
    }
}

impl MoveList {

    pub fn add(&mut self, mv: Move) {
        self.0.push(mv);
    }

    pub fn display_raw(&self) {
        for mv in self.0.iter() {
            println!("{}{}", mv.src, mv.dst)
        }
    }

    pub fn display(&self) {
        let mut src_piece_array = [0u8; 64];
        let mut dst_piece_array = [0u8; 64];

        for mv in self.0.iter() {
            src_piece_array[mv.src as usize] += 1;
            dst_piece_array[mv.dst as usize] += 1;
        }

        println!("src: \n");
        for rank in Rank::ALL.iter().rev() {
            for file in File::ALL {
                let sqr =  Square::new(*rank, file);
                let amt = src_piece_array[sqr as usize];
                match amt {
                    0 => print!("· "),
                    _ => print!("{amt} "),
                }
            }
            println!("");
        }

        println!("\ndst: \n");
        for rank in Rank::ALL.iter().rev() {
            for file in File::ALL {
                let sqr =  Square::new(*rank, file);
                let amt = dst_piece_array[sqr as usize];
                match amt {
                    0 => print!("· "),
                    _ => print!("{amt} "),
                }
            }
            println!("");
        }

        println!("count: {}", self.0.len())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Enum)]
pub enum MoveFlag {
    NonCapture,
    PawnDouble,
    CastleShort,
    CastleLong,
    PromotionQueen,
    PromotionRook,
    PromotionBishop,
    PromotionKnight,
    //capture flags
    Capture,
    EnPassant,
    CapturePromotionQueen,
    CapturePromotionRook,
    CapturePromotionBishop,
    CapturePromotionKnight,
}

impl MoveFlag {

    #[inline]
    fn is_capture(self) -> bool {
        (self as u8) > 8
    }

    #[inline]
    fn piece(&self) -> Piece {
        match self {
            MoveFlag::CapturePromotionQueen => Piece::Queen,
            MoveFlag::CapturePromotionRook => Piece::Rook,
            MoveFlag::CapturePromotionBishop => Piece::Bishop,
            MoveFlag::CapturePromotionKnight => Piece::Knight,
            MoveFlag::PromotionQueen => Piece::Queen,
            MoveFlag::PromotionRook => Piece::Rook,
            MoveFlag::PromotionBishop => Piece::Bishop,
            MoveFlag::PromotionKnight => Piece::Knight,
            _ => unreachable!("shouldnt be called of flag {:?}", self)
        }
    }
}

impl Board {
    #[inline]
    pub fn attackers(&self, colour: Colour, sqr: Square) -> Bitboard {
        let magic = magic_table();

        let diag = magic.get_diag(self.all_pieces(), sqr) 
            & (self.pieces(Piece::Bishop) | self.pieces(Piece::Queen))
            & self.colours(!colour);

        let orth = magic.get_orth(self.all_pieces(), sqr)
            & (self.pieces(Piece::Rook) | self.pieces(Piece::Queen))
            & self.colours(!colour);

        let knights = knight_mask(sqr) 
            & self.colours(!colour) 
            & self.pieces(Piece::Knight);

        let pawns = (sqr.relative_shift::<NorthEast>(colour, 1).map_or_default(|a|{a.to_bb()}) 
            | sqr.relative_shift::<NorthWest>(colour, 1).map_or_default(|a|{a.to_bb()}))
            & self.colours(!colour)
            & self.pieces(Piece::Pawn);

        let king = king_mask(sqr) 
            & self.colours(!colour) 
            & self.pieces(Piece::King);

        diag | orth | knights | pawns | king
    }

    #[inline]
    fn remove_castling_rights(&mut self, mv: Move, moved_piece: Piece) {
        // TODO: some guard clause to return early when not needed to check
        // i.e have the src and dst as a bb and relevant squares as a mask

        if moved_piece == Piece::King {
            self.set_castling(self.stm, true, false);
            self.set_castling(self.stm, false, false);
        } else if moved_piece == Piece::Rook {
            if mv.src == Square::A1.relative(self.stm) {
                self.set_castling(self.stm, false, false);
            }

            if mv.src == Square::H1.relative(self.stm) {
                self.set_castling(self.stm, true, false);
            }
        }

        if mv.dst == Square::A8.relative(self.stm) {
            self.set_castling(!self.stm, false, false);
        }

        if mv.dst == Square::H8.relative(self.stm) {
            self.set_castling(!self.stm, true, false);
        }
    }

    pub fn do_move(&mut self, mv: Move) {
        self.hmc += 1;
        self.fmn += (self.stm == Colour::Black) as u32;
        let moved_piece = self.remove_piece(mv.src);
        self.set_square(mv.dst, self.stm(), moved_piece);
        self.set_en_passant(None);
        self.remove_castling_rights(mv, moved_piece);

        if moved_piece == Piece::Pawn {
            self.hmc = 0;
        }

        match mv.flag {
            MoveFlag::NonCapture => {

            },
            MoveFlag::Capture => {
                self.hmc = 0;
            },
            MoveFlag::PawnDouble => {
                self.set_en_passant(Some(mv.dst));
            }
            MoveFlag::CastleLong => {
                let rook_src = Square::A1.relative(self.stm);
                let rook_dst = Square::D1.relative(self.stm);
                self.remove_piece(rook_src);
                self.set_square(rook_dst, self.stm, Piece::Rook);
            },
            MoveFlag::CastleShort => {
                let rook_src = Square::H1.relative(self.stm);
                let rook_dst = Square::D1.relative(self.stm);
                self.remove_piece(rook_src);
                self.set_square(rook_dst, self.stm, Piece::Rook);
            },
            MoveFlag::EnPassant => {
                // self.hmc = 0;
                let file = mv.dst.file();
                let rank = Rank::Five.relative_to(self.stm);
                self.remove_piece(Square::new(rank, file));
            },
            MoveFlag::CapturePromotionKnight |
            MoveFlag::CapturePromotionBishop |
            MoveFlag::CapturePromotionQueen |
            MoveFlag::CapturePromotionRook => {
                self.hmc = 0;
                self.set_square(mv.dst, self.stm, mv.flag.piece());
            },
            MoveFlag::PromotionKnight |
            MoveFlag::PromotionBishop |
            MoveFlag::PromotionQueen |
            MoveFlag::PromotionRook => {
                self.set_square(mv.dst, self.stm, mv.flag.piece());
            }
        }
        self.stm = !self.stm;
    }
}