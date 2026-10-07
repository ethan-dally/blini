use std::fmt::Display;

use crate::{board::board::Board, common::piece::Piece};

const PIECES: [(Piece, u32); 5] = [
    (Piece::Pawn, 100),
    (Piece::Knight, 300),
    (Piece::Bishop, 325),
    (Piece::Rook, 500),
    (Piece::Queen, 900),
];

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Score(i16);

impl Score {
    pub const CHECKMATE: Score = Score(10_000);
    pub const MIN: Score = Score(-10_000);
    pub const MAX: Score = Score(10_000);
    pub const DRAW: Score = Score(0);
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Neg for Score {
    type Output = Score;
    fn neg(self) -> Self::Output {
        Score(-self.0)
    }
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
pub fn eval(board: &Board) -> Score {
    let mut score: i16 = 0;
    for (piece, piece_score) in PIECES {
        let us_count = (board.pieces(piece) & board.colours(board.stm())).count();
        let them_count = (board.pieces(piece) & board.colours(!board.stm())).count();
        score += (us_count * piece_score) as i16;
        score -= (them_count * piece_score) as i16;
    }
    Score(score)
}
