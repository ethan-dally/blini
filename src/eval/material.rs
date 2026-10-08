use std::fmt::Display;

use crate::{board::board::Board, common::{piece::Piece, square::Square}, search::search::Score};

const PIECES: [(Piece, u32); 5] = [
    (Piece::Pawn, 100),
    (Piece::Knight, 300),
    (Piece::Bishop, 325),
    (Piece::Rook, 500),
    (Piece::Queen, 900),
];

// static psq tables taken from https://chessprogramming.org/Simplified_Evaluation_Function
// king in a seperate table

#[rustfmt::skip]
const WHITE_PSQT: [[i8; 64]; 6] = [
    // Pawn
    [
         0,  0,  0,  0,  0,  0,  0,  0,
        50, 50, 50, 50, 50, 50, 50, 50,
        10, 10, 20, 30, 30, 20, 10, 10,
         5,  5, 10, 25, 25, 10,  5,  5,
         0,  0,  0, 20, 20,  0,  0,  0,
         5, -5,-10,  0,  0,-10, -5,  5,
         5, 10, 10,-20,-20, 10, 10,  5,
         0,  0,  0,  0,  0,  0,  0,  0
    ],

    // Knight
    [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ],

    // Bishop
    [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ],

    // Rook
    [
         0,  0,  0,  0,  0,  0,  0,  0,
         5, 10, 10, 10, 10, 10, 10,  5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
         0,  0,  0,  5,  5,  0,  0,  0
    ],

    // Queen
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
         -5,  0,  5,  5,  5,  5,  0, -5,
          0,  0,  5,  5,  5,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ],

    // Static King (using MG vals only for now)
    [
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
         20, 20,  0,  0,  0,  0, 20, 20,
         20, 30, 10,  0,  0, 10, 30, 20
    ]
];

const BLACK_PSQT: [[i8; 64]; 6] = {
    let mut psqt: [[i8; 64]; 6] = [[0; 64]; 6];
    let mut piece = 0;
    while piece < 6 {
        let mut sqr = 0;
        while sqr < 64 {
            psqt[piece][sqr] = WHITE_PSQT[piece][sqr ^ 0b111000];
            sqr += 1;
        }
        piece += 1;
    }
    psqt
};

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
    Score::from_i16(score)
}