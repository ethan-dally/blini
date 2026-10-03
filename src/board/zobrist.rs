use crate::{
    board::board::{Board, Castling},
    common::{colour::Colour, file::File, piece::Piece, square::Square},
};

const ZOBRIST_SIZE: usize = 793;
static ZOBRIST_TABLE: [u64; ZOBRIST_SIZE] = build_zobrist();

const fn splitmix64(prev: u64) -> u64 {
    let mut z = prev.wrapping_add(0x9e3779b97f4a7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

const fn build_zobrist() -> [u64; ZOBRIST_SIZE] {
    let mut arr: [u64; ZOBRIST_SIZE] = [0; ZOBRIST_SIZE];
    let mut index = 0;
    let mut prev = 1;
    while index < ZOBRIST_SIZE {
        prev = splitmix64(prev);
        arr[index] = prev;
        index += 1;
    }
    arr
}

#[derive(Debug)]
pub enum Zobrist {
    Move,
    Piece {
        piece: Piece,
        sqr: Square,
        colour: Colour,
    },
    EnPassant(File),
    Castling(Castling),
}

impl Zobrist {
    #[inline]
    fn get_index(self) -> usize {
        match self {
            Zobrist::Move => 0,
            Zobrist::Piece { piece, sqr, colour } => {
                (colour as usize | (sqr as usize) << 1 | (piece as usize) << 7) + 1
            }
            Zobrist::EnPassant(file) => 769 + file as usize,
            Zobrist::Castling(castling) => {
                debug_assert!(16 > castling.value());
                777 + usize::from(castling.value())
            }
        }
    }

    #[inline]
    pub fn get(self) -> u64 {
        ZOBRIST_TABLE[self.get_index()]
    }

    #[inline]
    pub fn calculate(board: &Board) -> u64 {
        // TODO: perhaps move into Board?
        let mut zobrist: u64 = 0;
        if board.stm() == Colour::Black {
            zobrist ^= Zobrist::Move.get();
        }
        for (sqr, entry) in board.mailbox {
            let Some((piece, colour)) = entry else {
                continue;
            };
            zobrist ^= Zobrist::Piece { piece, sqr, colour }.get();
        }
        if let Some(sqr) = board.en_passant() {
            zobrist ^= Zobrist::EnPassant(sqr.file()).get();
        }
        zobrist ^= Zobrist::Castling(board.castling).get();
        zobrist
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{
            board::{Board, Castling},
            zobrist::{ZOBRIST_TABLE, Zobrist},
        },
        common::{colour::Colour, file::File, piece::Piece, square::Square},
    };

    #[test]
    fn zobrist_index() {
        assert_eq!(Zobrist::get_index(Zobrist::Move), 0);

        let mut index = 1;

        for piece in Piece::ALL {
            for sqr in Square::ALL {
                for colour in Colour::ALL {
                    let calc_index = Zobrist::Piece { piece, sqr, colour }.get_index();
                    assert_eq!(index, calc_index);
                    ZOBRIST_TABLE
                        .get(calc_index)
                        .expect(&*format!("zobrist oob at index {}", calc_index));
                    index += 1;
                }
            }
        }

        for file in File::ALL {
            let calc_index = Zobrist::EnPassant(file).get_index();
            assert_eq!(index, calc_index);
            ZOBRIST_TABLE
                .get(calc_index)
                .expect(&*format!("zobrist oob at index {}", calc_index));
            index += 1;
        }

        for raw_u8 in 0..16 {
            let calc_index = Zobrist::Castling(Castling::from_u8(raw_u8)).get_index();
            assert_eq!(index, calc_index);
            index += 1;
        }

        assert!(
            ZOBRIST_TABLE.get(index).is_none(),
            "zobrist table {} is too big",
            index
        );
    }

    macro_rules! test_zob {
        ($name:ident, $moves:expr, $fen:expr) => {
            #[test]
            fn $name() {
                let board = Board::parse_fen($fen)
                    .unwrap()
                    .apply_uci_moves($moves)
                    .unwrap();
                assert_eq!(board.zobrist, Zobrist::calculate(&board));
            }
        };
    }

    test_zob!(
        empty,
        vec![],
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
    test_zob!(
        stm_black,
        vec!["e7e5"],
        "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
    );
    test_zob!(
        en_passant,
        vec!["f5e6"],
        "rnbqkbnr/pppp1ppp/8/4pP2/8/8/PPPPP1PP/RNBQKBNR w KQkq e6 0 1"
    );
    test_zob!(
        castling_rights,
        vec!["h1h2"],
        "rnbqkbnr/ppppppp1/7p/8/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 1"
    );
    test_zob!(
        castling,
        vec!["e1g1"],
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1"
    );
    test_zob!(
        startpos,
        vec![
            "e2e4", "e7e5", "g1f3", "b8c6", "f1b5", "a7a6", "b5a4", "g8f6", "e1g1", "f8e7", "f1e1",
            "b7b5", "a4b3", "d7d6", "c2c3", "e8g8"
        ],
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );

    const KIWI_PETE: &'static str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    test_zob!(
        kiwi_pete1,
        vec![
            "g2g3", "c7c6", "a1c1", "e8g8", "e2c4", "f8d8", "a2a3", "f6d5"
        ],
        KIWI_PETE
    );
    test_zob!(
        kiwi_pete2,
        vec![
            "e5d3", "a6b7", "f3g3", "h8h4", "g3e3", "b7c8", "g2h3", "f6g4"
        ],
        KIWI_PETE
    );
    test_zob!(
        kiwi_pete3,
        vec![
            "a1b1", "e7d6", "d2e3", "h8h5", "a2a3", "e8f8", "e5f7", "f6g4"
        ],
        KIWI_PETE
    );
    test_zob!(
        kiwi_pete4,
        vec![
            "h1f1", "a8c8", "e1c1", "c8a8", "c3b5", "h8g8", "f3f5", "d7d6"
        ],
        KIWI_PETE
    );
    test_zob!(
        kiwi_pete5,
        vec![
            "e5g6", "a6c4", "f3h5", "e8d8", "h5h6", "f6g4", "e1d1", "b4b3"
        ],
        KIWI_PETE
    );
}
