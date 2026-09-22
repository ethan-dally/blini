use enum_map::{Enum, EnumMap};

use crate::{board::board::Board, common::r#move::MoveFlag};

#[derive(Debug, Default)]
pub(super) struct FlagsCount(pub EnumMap<MoveFlag, u32>);

impl FlagsCount {
    #[inline]
    fn add(&mut self, flag: MoveFlag) {
        self.0[flag] += 1;
    }

    #[inline]
    fn add_assign(&mut self, other: Self) {
        for (flag, count) in other.0 {
            self.0[flag] += count;
        }
    }
}

impl Board {
    fn perft(self, depth: u8) -> (u64, FlagsCount) {
        let move_list = self.get_moves();

        if depth == 0 {
            let count = move_list.0.iter().len() as u64;
            let mut move_flags = FlagsCount::default();
            for mv in move_list {
                move_flags.add(mv.flag());
            }
            return (count, move_flags);
        }

        let mut count = 0;
        let mut flags_count = FlagsCount::default();

        for mv in move_list {
            let mut next = self.clone();
            next.do_move(mv);
            let perft = next.perft(depth - 1);
            flags_count.add_assign(perft.1);
            count += perft.0;
        }
        return (count, flags_count);
    }
}

#[test]
fn perft_1() {
    let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("fen incorrect");
    let perft = board.perft(0);
    perft.1.display();
    assert_eq!(perft.0, 20);
}

#[test]
fn perft_2() {
    let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("fen incorrect");
    let perft = board.perft(1);
    perft.1.display();
    assert_eq!(perft.0, 400);
}

#[test]
fn perft_3() {
    let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("fen incorrect");
    let perft = board.perft(2);
    perft.1.display();
    assert_eq!(perft.0, 8902);
}

#[test]
fn perft_4() {
    let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("fen incorrect");
    let perft = board.perft(3);
    perft.1.display();
    assert_eq!(perft.0, 197281);
}

// #[test]
// fn perft_5() {
//     let board = Board::parse_fen(
//         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(4);
//     perft.1.display();
//     assert_eq!(perft.0, 4865609);
// }

#[test]
fn kiwipete_1() {
    let board =
        Board::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .expect("fen incorrect");
    let perft = board.perft(0);
    perft.1.display();
    assert_eq!(perft.0, 48);
}

#[test]
fn kiwipete_2() {
    let board =
        Board::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .expect("fen incorrect");
    let perft = board.perft(1);
    perft.1.display();
    assert_eq!(perft.0, 2039);
}

#[test]
fn kiwipete_3() {
    let board =
        Board::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .expect("fen incorrect");
    let perft = board.perft(2);
    perft.1.display();
    assert_eq!(perft.0, 97862);
}

#[test]
fn kiwipete_4() {
    let board =
        Board::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .expect("fen incorrect");
    let perft = board.perft(3);
    perft.1.display();
    println!("total: {}", perft.0);
    assert_eq!(perft.0, 4085603);
}

//EDP tests
macro_rules! epd_tests {
    ($(($name:ident, $fen:expr, $expected:expr)),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let board = Board::parse_fen($fen).expect("fen incorrect");
                let moves = board.get_moves();

                board.display();
                moves.display();
                moves.display_raw();
                println!("expected: {}, got: {}", $expected, moves.0.len());
                assert_eq!(moves.0.len(), $expected);
            }
        )*
    };
}

//tests from https://github.com/ChrisWhittington/Chess-EPDs
epd_tests! {
    (epd_001, "r4nk1/p5bp/2p5/5p1P/6P1/4R1B1/K7/8 b - - 0 1", 23),
    (epd_002, "5k2/8/1P2B3/p5P1/7r/1R6/1K6/8 w - - 0 1", 27),
}
