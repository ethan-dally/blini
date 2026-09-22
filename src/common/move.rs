use arrayvec::ArrayVec;
use enum_map::Enum;
use crate::{common::{file::File, piece::Piece, rank::Rank, square::Square}};

const MAX_MOVES: usize = 218;

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
    pub fn piece(&self) -> Piece {
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

#[derive(Debug, Clone, Copy)]
pub struct Move {
    src: Square,
    dst: Square,
    flag: MoveFlag,
}

impl Move {
    #[inline]
    pub fn new(src: Square, dst: Square, flag: MoveFlag) -> Move {
        Move { src, dst, flag}
    }

    pub fn display(&self) {
        println!("s: {}, d: {}, f: {:?}", self.src, self.dst, self.flag);
    }

    #[inline]
    pub fn flag(&self) -> MoveFlag {
        self.flag
    }

    #[inline]
    pub fn src(&self) -> Square {
        self.src
    }

    #[inline]
    pub fn dst(&self) -> Square {
        self.dst
    }
}

#[derive(Debug, Clone, Default)]
pub struct MoveList(pub ArrayVec<Move, MAX_MOVES>);

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

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = arrayvec::IntoIter<Move, MAX_MOVES>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}