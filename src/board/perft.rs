use enum_map::{Enum, EnumMap};

use crate::board::{board::Board, r#move::{Move, MoveFlag}};

#[derive(Debug, Default)]
struct FlagsCount(EnumMap<MoveFlag, u32>);

impl FlagsCount {
    fn display(&self) {
        println!(
            "NonCapture: {}, PawnDouble: {}, CastleShort: {}, CastleLong: {}, \
             Promotion: {}, Capture: {}, EnPassant: {}",
            self.0[MoveFlag::NonCapture],
            self.0[MoveFlag::PawnDouble],
            self.0[MoveFlag::CastleShort],
            self.0[MoveFlag::CastleLong],

            self.0[MoveFlag::PromotionQueen]
                + self.0[MoveFlag::PromotionRook]
                + self.0[MoveFlag::PromotionBishop]
                + self.0[MoveFlag::PromotionKnight],

            self.0[MoveFlag::Capture],

            self.0[MoveFlag::EnPassant],
        );
    }
}

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
                move_flags.add(mv.flag);
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

// #[test]
// fn perft_1() {
//     let board = Board::parse_fen(
//         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(0);
//     perft.1.display();
//     assert_eq!(perft.0, 40);
// }

// #[test]
// fn perft_2() {
//     let board = Board::parse_fen(
//         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(1);
//     perft.1.display();
//     assert_eq!(perft.0, 400);
// }

// #[test]
// fn perft_3() {
//     let board = Board::parse_fen(
//         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(2);
//     perft.1.display();
//     assert_eq!(perft.0, 8902);
// }

// #[test]
// fn perft_4() {
//     let board = Board::parse_fen(
//         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(3);
//     perft.1.display();
//     assert_eq!(perft.0, 197281);
// }

// #[test]
// fn kiwipete_1() {
//     let board = Board::parse_fen(
//         "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(0);
//     perft.1.display();
//     assert_eq!(perft.0, 48);
//     assert!(false);
// }

// #[test]
// fn kiwipete_2() {
//     let board = Board::parse_fen(
//         "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(1);
//     perft.1.display();
//     assert_eq!(perft.0, 2039);
// }

// #[test]
// fn perft_test() {
//     let board = Board::parse_fen(
//         "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
//     ).expect("fen incorrect");
//     board.display();
//     board.get_moves().display();
//     assert!(false);
// }

//EDP test suite
macro_rules! edp_tests {
    ($(($name:ident, $fen:expr, $expected:expr)),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let board = Board::parse_fen($fen).expect("fen incorrect");
                let moves = board.get_moves();

                board.display();
                moves.display();

                println!("expected: {}, got: {}", $expected, moves.0.len());
                assert_eq!(moves.0.len(), $expected);
            }
        )*
    };
}

edp_tests!{
    (edp_01, "k7/6p1/8/8/8/8/7P/K7 b - - 0 1", 5),
    (edp_02, "3k4/3pp3/8/8/8/8/3PP3/3K4 w - - 0 1", 7),
    (edp_03, "8/8/8/8/8/K7/P7/k7 w - - 0 1", 3),
    (edp_04, "8/8/8/8/8/7K/7P/7k w - - 0 1", 3),
    (edp_05, "K7/p7/k7/8/8/8/8/8 w - - 0 1", 1),
    (edp_06, "7K/7p/7k/8/8/8/8/8 w - - 0 1", 1),
    (edp_07, "8/2k1p3/3pP3/3P2K1/8/8/8/8 w - - 0 1", 7),
    (edp_10, "8/8/8/8/8/K7/P7/k7 b - - 0 1", 1),
    (edp_11, "8/8/8/8/8/7K/7P/7k b - - 0 1", 1),
    (edp_12, "K7/p7/k7/8/8/8/8/8 b - - 0 1", 3),
    (edp_13, "7K/7p/7k/8/8/8/8/8 b - - 0 1", 3),
    (edp_14, "8/2k1p3/3pP3/3P2K1/8/8/8/8 b - - 0 1", 5),
    (edp_15, "8/8/8/8/8/4k3/4P3/4K3 w - - 0 1", 2),
    (edp_16, "4k3/4p3/4K3/8/8/8/8/8 b - - 0 1", 2),
    (edp_17, "8/8/7k/7p/7P/7K/8/8 w - - 0 1", 3),
    (edp_20, "8/8/k7/p7/P7/K7/8/8 w - - 0 1", 3),
    (edp_21, "8/8/3k4/3p4/3P4/3K4/8/8 w - - 0 1", 5),
    (edp_22, "8/3k4/3p4/8/3P4/3K4/8/8 w - - 0 1", 8),
    (edp_23, "8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1", 8),
    (edp_24, "k7/8/3p4/8/3P4/8/8/7K w - - 0 1", 4),
    (edp_25, "8/8/7k/7p/7P/7K/8/8 b - - 0 1", 3),
    (edp_26, "8/8/k7/p7/P7/K7/8/8 b - - 0 1", 3),
    (edp_27, "8/8/3k4/3p4/3P4/3K4/8/8 b - - 0 1", 5),
    (edp_30, "8/3k4/3p4/8/3P4/3K4/8/8 b - - 0 1", 8),
    (edp_31, "8/8/3k4/3p4/8/3P4/3K4/8 b - - 0 1", 8),
    (edp_32, "k7/8/3p4/8/3P4/8/8/7K b - - 0 1", 4),
    (edp_33, "7k/3p4/8/8/3P4/8/8/K7 w - - 0 1", 4),
    (edp_34, "7k/8/8/3p4/8/8/3P4/K7 w - - 0 1", 5),
    (edp_35, "k7/8/8/7p/6P1/8/8/K7 w - - 0 1", 5),
    (edp_36, "k7/8/7p/8/8/6P1/8/K7 w - - 0 1", 4),
    (edp_37, "k7/8/8/6p1/7P/8/8/K7 w - - 0 1", 5),
    (edp_40, "k7/8/6p1/8/8/7P/8/K7 w - - 0 1", 4),
    (edp_41, "k7/8/8/3p4/4p3/8/8/7K w - - 0 1", 3),
    (edp_42, "k7/8/3p4/8/8/4P3/8/7K w - - 0 1", 4),
    (edp_43, "7k/3p4/8/8/3P4/8/8/K7 b - - 0 1", 5),
    (edp_44, "7k/8/8/3p4/8/8/3P4/K7 b - - 0 1", 4),
    (edp_45, "k7/8/8/7p/6P1/8/8/K7 b - - 0 1", 5),
    (edp_46, "k7/8/7p/8/8/6P1/8/K7 b - - 0 1", 4),
    (edp_47, "k7/8/8/6p1/7P/8/8/K7 b - - 0 1", 5),
    (edp_50, "k7/8/6p1/8/8/7P/8/K7 b - - 0 1", 4),
    (edp_51, "k7/8/8/3p4/4p3/8/8/7K b - - 0 1", 5),
    (edp_52, "k7/8/3p4/8/8/4P3/8/7K b - - 0 1", 4),
    (edp_53, "7k/8/8/p7/1P6/8/8/7K w - - 0 1", 5),
    (edp_54, "7k/8/p7/8/8/1P6/8/7K w - - 0 1", 4),
    (edp_55, "7k/8/8/1p6/P7/8/8/7K w - - 0 1", 5),
    (edp_56, "7k/8/1p6/8/8/P7/8/7K w - - 0 1", 4),
    (edp_57, "k7/7p/8/8/8/8/6P1/K7 w - - 0 1", 5),
    (edp_60, "k7/6p1/8/8/8/8/7P/K7 w - - 0 1", 5),
    (edp_61, "3k4/3pp3/8/8/8/8/3PP3/3K4 w - - 0 1", 7),
    (edp_62, "7k/8/8/p7/1P6/8/8/7K b - - 0 1", 5),
    (edp_63, "7k/8/p7/8/8/1P6/8/7K b - - 0 1", 4),
    (edp_64, "7k/8/8/1p6/P7/8/8/7K b - - 0 1", 5),
    (edp_65, "7k/8/1p6/8/8/P7/8/7K b - - 0 1", 4),
    (edp_66, "k7/7p/8/8/8/8/6P1/K7 b - - 0 1", 5),
    (edp_67, "k7/6p1/8/8/8/8/7P/K7 b - - 0 1", 5),
    (edp_70, "3k4/3pp3/8/8/8/8/3PP3/3K4 b - - 0 1", 7),
    (edp_71, "4k3/8/8/8/8/8/8/4K2R w K - 0 1", 15),
    (edp_72, "4k3/8/8/8/8/8/8/R3K3 w Q - 0 1", 16),
    (edp_73, "4k2r/8/8/8/8/8/8/4K3 w k - 0 1", 5),
    (edp_74, "r3k3/8/8/8/8/8/8/4K3 w q - 0 1", 5),
    (edp_75, "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1", 26),
    (edp_76, "r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1", 5),
    (edp_77, "8/8/8/8/8/8/6k1/4K2R w K - 0 1", 12),
    (edp_100, "8/8/8/8/8/8/1k6/R3K3 w Q - 0 1", 15),
    (edp_101, "4k2r/6K1/8/8/8/8/8/8 w k - 0 1", 3),
    (edp_102, "r3k3/1K6/8/8/8/8/8/8 w q - 0 1", 4),
    (edp_103, "4k3/8/8/8/8/8/8/4K2R b K - 0 1", 5),
    (edp_104, "4k3/8/8/8/8/8/8/R3K3 b Q - 0 1", 5),
    (edp_105, "4k2r/8/8/8/8/8/8/4K3 b k - 0 1", 15),
    (edp_106, "r3k3/8/8/8/8/8/8/4K3 b q - 0 1", 16),
    (edp_107, "4k3/8/8/8/8/8/8/R3K2R b KQ - 0 1", 5),
    (edp_110, "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1", 26),
    (edp_111, "8/8/8/8/8/8/6k1/4K2R b K - 0 1", 3),
    (edp_112, "8/8/8/8/8/8/1k6/R3K3 b Q - 0 1", 4),
    (edp_113, "4k2r/6K1/8/8/8/8/8/8 b k - 0 1", 12),
    (edp_114, "r3k3/1K6/8/8/8/8/8/8 b q - 0 1", 15),
    (edp_115, "8/1n4N1/2k5/8/8/5K2/1N4n1/8 w - - 0 1", 14),
    (edp_116, "8/1k6/8/5N2/8/4n3/8/2K5 w - - 0 1", 11),
    (edp_117, "8/8/4k3/3Nn3/3nN3/4K3/8/8 w - - 0 1", 19),
    (edp_120, "K7/8/2n5/1n6/8/8/8/k6N w - - 0 1", 3),
    (edp_121, "k7/8/2N5/1N6/8/8/8/K6n w - - 0 1", 17),
    (edp_122, "8/1n4N1/2k5/8/8/5K2/1N4n1/8 b - - 0 1", 15),
    (edp_123, "8/1k6/8/5N2/8/4n3/8/2K5 b - - 0 1", 16),
    (edp_124, "8/8/3K4/3Nn3/3nN3/4k3/8/8 b - - 0 1", 4),
    (edp_125, "K7/8/2n5/1n6/8/8/8/k6N b - - 0 1", 17),
    (edp_126, "k7/8/2N5/1N6/8/8/8/K6n b - - 0 1", 3),
    (edp_127, "B6b/8/8/8/2K5/4k3/8/b6B w - - 0 1", 17),
    (edp_130, "8/8/1B6/7b/7k/8/2B1b3/7K w - - 0 1", 21),
    (edp_131, "k7/B7/1B6/1B6/8/8/8/K6b w - - 0 1", 21),
    (edp_132, "K7/b7/1b6/1b6/8/8/8/k6B w - - 0 1", 7),
    (edp_133, "B6b/8/8/8/2K5/5k2/8/b6B b - - 0 1", 6),
    (edp_134, "8/8/1B6/7b/7k/8/2B1b3/7K b - - 0 1", 17),
    (edp_135, "k7/B7/1B6/1B6/8/8/8/K6b b - - 0 1", 7),
    (edp_136, "K7/b7/1b6/1b6/8/8/8/k6B b - - 0 1", 21),
    (edp_137, "7k/RR6/8/8/8/8/rr6/7K w - - 0 1", 19),
    (edp_140, "6kq/8/8/8/8/8/8/7K w - - 0 1", 2),
    (edp_141, "6KQ/8/8/8/8/8/8/7k b - - 0 1", 2),
    (edp_142, "K7/8/8/3Q4/4q3/8/8/7k w - - 0 1", 6),
    (edp_143, "6qk/8/8/8/8/8/8/7K b - - 0 1", 22),
    (edp_144, "6KQ/8/8/8/8/8/8/7k b - - 0 1", 2),
    (edp_145, "K7/8/8/3Q4/4q3/8/8/7k b - - 0 1", 6),
    (edp_146, "8/Pk6/8/8/8/8/6Kp/8 w - - 0 1", 11),
    (edp_147, "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N w - - 0 1", 24),
    (edp_150, "8/PPPk4/8/8/8/8/4Kppp/8 w - - 0 1", 18),
    (edp_151, "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - - 0 1", 24),
    (edp_152, "8/Pk6/8/8/8/8/6Kp/8 b - - 0 1", 11),
    (edp_153, "r3k2r/8/8/8/8/8/8/1R2K2R b Kkq - 0 1", 26),
    (edp_154, "r3k2r/8/8/8/8/8/8/2R1K2R b Kkq - 0 1", 25),
    (edp_155, "r3k2r/8/8/8/8/8/8/R3K1R1 b Qkq - 0 1", 25),
    (edp_156, "1r2k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1", 25),
    (edp_157, "2r1k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1", 25),
    (edp_160, "r3k1r1/8/8/8/8/8/8/R3K2R b KQq - 0 1", 25),
    (edp_161, "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", 26),
    (edp_162, "r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1", 25),
    (edp_163, "r3k2r/8/8/8/8/8/8/2R1K2R w Kkq - 0 1", 25),
    (edp_164, "r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1", 25),
    (edp_165, "1r2k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1", 26),
    (edp_166, "2r1k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1", 25),
    (edp_167, "r3k1r1/8/8/8/8/8/8/R3K2R w KQq - 0 1", 25),
    (edp_170, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 20),
    (edp_171, "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N b - - 0 1", 24),
    (edp_172, "8/PPPk4/8/8/8/8/4Kppp/8 b - - 0 1", 18),
    (edp_173, "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1", 24),
    (edp_174, "R6r/8/8/2K5/5k2/8/8/r6R w - - 0 1", 36),
    (edp_175, "7k/RR6/8/8/8/8/rr6/7K b - - 0 1", 19),
    (edp_176, "R6r/8/8/2K5/5k2/8/8/r6R b - - 0 1", 36),
}