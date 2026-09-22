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

#[test]
fn perft_1() {
    let board = Board::parse_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    ).expect("fen incorrect");
    let perft = board.perft(0);
    perft.1.display();
    assert_eq!(perft.0, 20);
}

#[test]
fn perft_2() {
    let board = Board::parse_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    ).expect("fen incorrect");
    let perft = board.perft(1);
    perft.1.display();
    assert_eq!(perft.0, 400);
}

#[test]
fn perft_3() {
    let board = Board::parse_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    ).expect("fen incorrect");
    let perft = board.perft(2);
    perft.1.display();
    assert_eq!(perft.0, 8902);
}

// #[test]
// fn perft_4() {
//     let board = Board::parse_fen(
//         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
//     ).expect("fen incorrect");
//     let perft = board.perft(3);
//     perft.1.display();
//     assert_eq!(perft.0, 197281);
// }

#[test]
fn kiwipete_1() {
    let board = Board::parse_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
    ).expect("fen incorrect");
    let perft = board.perft(0);
    perft.1.display();
    assert_eq!(perft.0, 48);
}

#[test]
fn kiwipete_2() {
    let board = Board::parse_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
    ).expect("fen incorrect");
    let perft = board.perft(1);
    perft.1.display();
    assert_eq!(perft.0, 2039);
}

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
macro_rules! epd_tests {
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

//tests from https://github.com/ChrisWhittington/Chess-EPDs
epd_tests!{
    (epd_001,"3k1b2/p4r2/1P6/P1p1pnpP/2P5/R4B1R/6N1/4K3 b - - 0 1", 27),
    (epd_002,"4r3/5b2/p3Pn2/P1kP4/2p4p/7N/2BK2P1/3N2R1 w - - 0 1", 28),
    (epd_003,"3N3k/B4P2/n6P/1b6/3K2p1/6P1/8/2r4R w - - 0 1", 24),
    (epd_004,"7k/7n/R3b2P/3B4/5K2/8/2b5/8 w - - 0 1", 26),
    (epd_005,"8/8/4P1k1/4K1B1/8/8/8/5q2 w - - 0 1", 14),
    (epd_006,"rn2k2r/4n2p/1p2p1p1/8/P2q2P1/4P2P/3b1KbR/3r1RN1 b - - 0 1", 61),
    (epd_007,"6r1/8/k7/8/3P4/P4K1R/8/3N4 b - - 0 1", 19),
    (epd_010,"b6b/1n6/P3kP2/2p5/1BP3P1/p1N3p1/4K3/1N1R4 b - - 0 1", 11),
    (epd_011,"rn2qb1r/N1pkp1pp/Qp6/3bnP2/2pP1P2/4P2P/PP1NB3/R1B1K1R1 w Q - 0 1", 45),
    (epd_012,"1k1n4/5b2/8/3p2p1/3K4/8/3N4/8 b - - 0 1", 14),
    (epd_013,"3rk2r/p4pQ1/3p2pb/1pp3P1/P2p4/2n2R1b/1PKN3P/2B4R w k - 0 1", 38),
    (epd_014,"8/4R3/6R1/2p2K2/2Pq4/8/3N4/3k4 b - - 0 1", 24),
    (epd_015,"6nr/2p1p2B/nr2b2k/1p2P3/1bq3pP/6P1/P4P1K/1R3BNR b - - 0 1", 39),
    (epd_016,"k7/7r/8/8/2pK4/2P5/4N3/8 b - - 0 1", 17),
    (epd_017,"6bB/2n2k2/7p/2r5/6K1/1P1pp2R/8/R6B b - - 0 1", 27),
    (epd_020,"rnb2qnr/pppPbk1p/4p1p1/5p2/8/1P2PP2/PR1P2PP/2BQKBNR b K - 0 1", 31),
    (epd_021,"3b2k1/8/8/8/8/2K3p1/3p2P1/2B2b2 w - - 0 1", 9),
    (epd_022,"1Kr4k/8/5N2/1P6/8/5n2/8/8 w - - 0 1", 3),
    (epd_023,"r2B1qr1/p2bbk1p/np2p1pn/2p2p2/6PP/PP2PP2/1R1PQ3/2B1KBNR b K - 0 1", 31),
    (epd_024,"rk3b1N/q5p1/1p1p4/1b2p2p/1pnPPpn1/R1B2PPB/3NK3/7R w - - 0 1", 35),
    (epd_025,"3b2k1/8/b7/8/8/B2p2p1/6P1/K7 b - - 0 1", 16),
    (epd_026,"8/k7/2B1p3/4P2p/K3P2P/8/8/5N2 w - - 0 1", 15),
    (epd_027,"rnbk1r2/3p3p/2p1Q3/p1b4n/PP6/2Np2qn/1R1P3R/2B4K w - - 0 1", 40),
    (epd_030,"rnb1kbnr/pppp1pp1/4pq1p/8/2P1P3/1P6/P2P1PPP/RNBQKBNR w KQkq - 0 1", 28),
    (epd_031,"5b2/3k4/6P1/8/5N2/1N6/2K2b2/5bR1 b - - 0 1", 30),
    (epd_032,"4k2b/1B5N/p2p4/P1r1p1P1/1Q5p/N1P4P/4K2R/1R6 w - - 0 1", 48),
    (epd_033,"8/1p1k3B/7n/3r2p1/1Kpp4/1R1b2PN/7P/5R2 b - - 0 1", 31),
    (epd_034,"6k1/8/3R4/4b3/3p4/B5p1/2b3P1/2K5 b - - 0 1", 20),
    (epd_035,"4q3/r3bk1b/1p2pnpr/p1pP3p/P2P1PPP/1QN3NR/1P1B1KB1/R7 b - - 0 1", 35),
    (epd_036,"1B6/B2n1k2/8/5P1P/2pK4/R2n1p2/8/7r w - - 0 1", 23),
    (epd_037,"1rb1k1r1/p2p2b1/B3p3/6pn/P2P2Pp/2p4P/R1P1P3/1N1QBKNR b - - 0 1", 28),
    (epd_040,"8/6b1/1kB1P3/8/8/QK3p2/6P1/8 b - - 0 1", 13),
    (epd_041,"3N4/8/3p1P1R/1b6/5kn1/6n1/1K5p/8 b - - 0 1", 31),
    (epd_042,"2r2nk1/7r/8/1R5p/p6p/3pPB2/1b3K2/R7 b - - 0 1", 38),
    (epd_043,"4kr1r/2pR3p/1p3p1n/Q4n2/PP1B1bP1/2p4q/4P2P/4KR2 w - - 0 1", 37),
    (epd_044,"1r3br1/1bN4p/p1p1B1k1/4pq2/1PP4R/3p4/PB2P3/3K1RN1 w - - 0 1", 44),
    (epd_045,"2k5/7b/8/p7/8/6K1/2r5/8 b - - 0 1", 24),
    (epd_046,"k2n4/3N4/1R6/6p1/1K6/2pp4/7P/8 b - - 0 1", 8), 
    (epd_047,"2k5/4Br1Q/p3b3/P7/1P6/3p4/8/R1K5 b - - 0 1", 23),
    (epd_050,"8/1b6/2k5/8/6pp/2Q5/5r2/1K6 b - - 0 1", 5),
    (epd_051,"5b1r/6k1/pp1p3p/PP2n1pP/2bP1Rp1/1R1BK3/2P4N/2Q5 b - - 0 1", 22),
    (epd_052,"6k1/7N/P5P1/3p4/3Q2B1/4p1pP/4K3/2R1N3 b - - 0 1", 1),
    (epd_053,"3kr3/6rp/p6P/npp1p3/2Pp4/P3bK2/RP6/N4BNQ w - - 0 1", 20),
    (epd_054,"4Nk2/7b/4R3/1B4P1/1P5R/p5B1/8/4K3 w - - 0 1", 48),
    (epd_055,"1n1q4/p1B4k/P4P2/Bp1p2p1/3P3p/7P/5NR1/4Q1K1 w - - 0 1", 40),
    (epd_056,"r1bqkb1r/pppppp1p/2n2np1/8/4P3/2P5/PP1PKPPP/RNBQ1BNR w kq - 0 1", 25),
    (epd_057,"8/p1bpkNr1/b2B3p/3p1PpP/1rB5/P1n3R1/Q1P3P1/1R2K1N1 b - - 0 1", 4),
    (epd_060,"B1k2bn1/p4r2/1P3p1B/P1p1p2P/2P5/2R2N2/4N3/4K2R b - - 0 1", 19),
    (epd_061,"1r1q1bn1/4pk1r/5p1p/pp1P3P/Pp2Q3/2N2PP1/2RP2B1/1NB2RK1 b - - 0 1", 22),
    (epd_062,"5k2/4R1q1/8/8/P3p2r/1p1P4/7K/1r6 w - - 0 1", 0),
    (epd_063,"1k4r1/3B3P/p2n4/Ppp4R/1P6/1P5P/1N3Kb1/1rBr4 w - - 0 1", 35),
    (epd_064,"3k3r/1r2b2p/p7/npp1p1PP/2PpQ1b1/PN3K2/RP2B3/6N1 w - - 0 1", 5),
    (epd_065,"6k1/8/1P2B3/p5P1/4r3/6R1/1K6/8 b - - 0 1", 5),
    (epd_066,"1R6/3k1p2/4r3/p5rp/1b2p3/p1PN3B/1R2KP1P/8 w - - 0 1", 37),
    (epd_067,"2r5/1p2pk1p/7n/3p1Pp1/2P3b1/2B1R1P1/5N1P/2K2B1R w - - 0 1", 38),
    (epd_070,"8/r7/8/1k2N3/p2P4/Pb4K1/8/R6r w - - 0 1", 22),
    (epd_071,"k2B4/Pq6/2n4p/3p4/b7/3p4/6K1/5nN1 w - - 0 1", 15),
    (epd_072,"2bq1b2/4k3/1p3ppr/2pp1Q1N/3p3P/2P5/PpN2PP1/1RB1KBR1 w - - 0 1", 48),
    (epd_073,"2kq4/4Q3/1n1p3b/r1NP1bpp/pPP2PP1/p3P2P/4K3/2R1NBR1 b - - 0 1", 33),
    (epd_074,"1r6/1k2b3/1p3R2/1P3p2/p3p3/6K1/PN2R3/1n1B4 b - - 0 1", 24),
    (epd_075,"8/8/2P1k3/5p1p/8/8/4N1K1/B7 b - - 0 1", 6),
    (epd_076,"8/8/1k1n4/1p5P/1p4K1/1P1R3R/8/r2N4 b - - 0 1", 24),
    (epd_077,"1k6/8/5pP1/p1p5/P1P5/1P3K2/Nr6/R1R5 w - - 0 1", 18),
    (epd_0100,"k5n1/b7/5P2/8/3p2Pp/R2B2bP/b3KN2/8 w - - 0 1", 26),
    (epd_0101,"1r6/8/r7/k7/3P4/P1N2K2/5R2/8 b - - 0 1", 24),
    (epd_0102,"5bk1/3n1b1r/1p1p2Bp/Pp4pP/3P2p1/1R1QKR2/2P4N/8 w - - 0 1", 38),
    (epd_0103,"rnbqkb1r/ppp1p1pp/3p1p1n/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 1", 20),
    (epd_0104,"r2k1bn1/pB5r/5p1p/1Pp1p2Q/P1P4P/2R2b2/3N4/2B1K1NR b - - 0 1", 30),
    (epd_0105,"7k/rp2B3/n7/2bpp3/P4r1N/1p4PK/7P/RN6 b - - 0 1", 35),
    (epd_0106,"r6r/pp3k2/3B1n2/q3p2p/Pb3pp1/1P1P3N/1R1QPPP1/1N2KB1R b K - 0 1", 46),
    (epd_0107,"r1b1kbr1/pp1ppppp/n1p4n/6Q1/3PPP2/2q5/P1PBK1PP/RN3BNR b q - 0 1", 35),
    (epd_0110,"6nr/k1r5/8/np1P2Pp/1p1P3P/RP2KN2/8/1qN3R1 b - - 0 1", 35),
    (epd_0111,"5knr/2p5/pr4pp/PP1p1p1q/1n1P1P1P/1P1bP3/1B1KR3/RB1N4 b - - 0 1", 32),
    (epd_0112,"5b2/6P1/p6p/P3k2P/6R1/1b1p2P1/3R4/4K3 w - - 0 1", 28),
    (epd_0113,"rk3b1N/q5p1/1p1p3n/1b2p2p/1pnPPp2/2B2PPB/R2NK3/3R4 b - - 0 1", 35),
    (epd_0114,"5k2/1n6/P1p2b2/6p1/1P6/N7/3B1r2/3K1N2 b - - 0 1", 28),
    (epd_0115,"R7/p7/1kBqP2Q/8/1P3bp1/3R4/2K3P1/8 w - - 0 1", 49),
    (epd_0116,"r1b1k1n1/3q1pr1/n1p1p2B/1pbp4/Pp2P2P/N1PP1P1B/8/R2QK2R w KQ - 0 1", 39),
    (epd_0117,"N4b2/1r6/5Pk1/8/qpP3P1/4B3/B1n5/2K2R2 w - - 0 1", 29),
    (epd_0120,"q3kb1r/3bp1p1/rp1p4/n1p3Np/1PPP1pn1/R3P1PB/3NKP2/B3R3 b k - 0 1", 43),
    (epd_0121,"b7/b1Pkr3/7N/7P/P3p3/K7/8/8 b - - 0 1", 23),
    (epd_0122,"8/5k2/2K5/8/1q6/6B1/8/6Q1 b - - 0 1", 31),
    (epd_0123,"rn1k3r/p7/b1p3pp/qp3p2/1P2P1P1/1PNK1pnP/P2P1R2/R1B2B2 b - - 0 1", 29),
    (epd_0124,"4k1Br/2p5/4pbp1/ppP1P1Bp/P2Q4/1PNR1P1P/5KP1/6R1 b - - 0 1", 14),
    (epd_0125,"k7/2r5/8/8/2p2K2/2PN4/8/8 b - - 0 1", 14),
    (epd_0126,"1n6/Bb3r1k/1P4p1/4p3/2pp1PpP/4Pp2/8/1n2R1RK w - - 0 1", 16),
    (epd_0127,"rnbqkb1r/2p1pppp/5n2/pp1p2B1/2PP4/7P/PP2PPP1/RN1QKBNR w KQk - 0 1", 33),
    (epd_0130,"rnbqkbnr/1ppppppp/p7/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 1", 19),
    (epd_0131,"5r2/1p6/p1kR1Q2/2p1Pbrp/R1K2pnP/1PP2P2/4N3/1N6 b - - 0 1", 1),
    (epd_0132,"8/8/k7/2R5/N2p2PP/3K2b1/8/5b2 w - - 0 1", 4),
    (epd_0133,"rnbqkbnr/ppppppp1/8/7p/8/3PB3/PPP1PPPP/RN1QKBNR b KQkq - 0 1", 21),
    (epd_0134,"3K1kn1/8/8/5P2/1P4p1/pNr5/8/5N2 b - - 0 1", 20),
    (epd_0135,"3r3r/8/2k5/8/3K2p1/8/8/8 w - - 0 1", 5),
    (epd_0136,"rn4r1/p2k4/b1p3pp/qp3p2/1P2PpP1/1PN3nP/P1KP1R2/R1B2B2 b - - 0 1", 33),
    (epd_0137,"8/4kP2/8/1P6/7P/p3b1n1/4R3/K7 b - - 0 1", 14),
    (epd_0140,"rnbqkb1r/p1pp1ppp/1p3n2/4p3/P5P1/7B/1PPPPP1P/RNBQK1NR w KQkq - 0 1", 20),
    (epd_0141,"2kr1b1r/1b1pn1pp/2p1p3/P4p2/p2BP1P1/N5P1/2PP4/RN1Q1BKR b - - 0 1", 20),
    (epd_0142,"k7/8/p1b5/4r3/8/8/5K2/8 w - - 0 1", 3),
    (epd_0143,"r7/1pk2p1p/3ppn1b/6BP/P1pQ2B1/R2P1n2/2K1P3/6qR w - - 0 1", 42),
    (epd_0144,"8/8/1kb4r/8/6p1/6Kp/8/8 b - - 0 1", 27),
}