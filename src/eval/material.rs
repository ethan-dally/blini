use crate::{board::board::Board, common::{colour::Colour, piece::Piece}, search::search::Score};

const PIECES: [u16; 6] = [100, 300, 325, 500, 900, 0];

// static psq tables taken from https://chessprogramming.org/Simplified_Evaluation_Function
// king in a seperate table

#[rustfmt::skip]
const PSQT: [[i16; 64]; 6] = [
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

const WHITE_PSQT: [[i16; 64]; 6] = {
    let mut psqt: [[i16; 64]; 6] = [[0; 64]; 6];
    let mut piece = 0;
    while piece < 6 {
        let piece_val = PIECES[piece].cast_signed();
        let mut sqr = 0;
        while sqr < 64 {
            psqt[piece][sqr] = PSQT[piece][sqr] + piece_val;
            sqr += 1;
        }
        piece += 1;
    }
    psqt
};

const BLACK_PSQT: [[i16; 64]; 6] = {
    let mut psqt: [[i16; 64]; 6] = [[0; 64]; 6];
    let mut piece = 0;
    while piece < 6 {
        let piece_val = PIECES[piece].cast_signed();
        let mut sqr = 0;
        while sqr < 64 {
            psqt[piece][sqr] = -PSQT[piece][sqr ^ 0b111000] - piece_val;
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
    for piece in Piece::ALL {
        for sqr in (board.pieces(piece) & board.colours(Colour::White)).iter() {
            score += WHITE_PSQT[piece as usize][sqr as usize];
        }
        for sqr in (board.pieces(piece) & board.colours(Colour::Black)).iter() {
            score += BLACK_PSQT[piece as usize][sqr as usize];
        }
    }
    Score::from_i16(score)
}
