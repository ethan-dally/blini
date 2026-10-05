use std::time::Instant;

use enum_map::EnumMap;

use crate::{board::board::Board, common::r#move::MoveFlag, uci::Engine};

#[derive(Debug, Default)]
struct FlagsCount(pub EnumMap<MoveFlag, u32>);

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

    #[allow(dead_code)]
    pub fn display(&self) {
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

impl Engine {
    pub fn run_perft(board: Board, depth: u8) {
        if depth == 0 {
            println!("1 nodes");
            return;
        }
        let time = Instant::now();
        let mut total_nodes = 0;
        let root_move_list = board.get_moves();
        for mv in root_move_list {
            let mut next = board.clone();
            next.do_move(mv);
            let count = next.perft(depth - 1);
            println!("{}: {count}", mv.uci());
            total_nodes += count;
        }
        let total_time = time.elapsed().as_nanos() + 1;
        let nps = (u128::from(total_nodes) * 1_000_000_000u128).div_ceil(total_time);
        println!("{total_nodes} nodes {total_time} time {nps} nps");
    }
}

impl Board {
    #[inline]
    fn perft(self, depth: u8) -> u64 {
        let move_list = self.get_moves();

        if depth == 0 {
            return 1;
        }

        if depth == 1 {
            let count = move_list.0.iter().len() as u64;
            return count;
        }

        let mut count = 0;
        for mv in move_list {
            let mut next = self.clone();
            next.do_move(mv);
            let perft = next.perft(depth - 1);
            count += perft;
        }
        count
    }

    #[allow(dead_code)]
    #[inline]
    fn perft_flags(self, depth: u8) -> (u64, FlagsCount) {
        let move_list = self.get_moves();

        if depth == 0 {
            return (0, FlagsCount::default());
        }

        if depth == 1 {
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
            let perft = next.perft_flags(depth - 1);
            flags_count.add_assign(perft.1);
            count += perft.0;
        }
        (count, flags_count)
    }
}

#[cfg(test)]
mod test {
    use crate::board::board::Board;
    const KIWIPETE_FEN: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

    macro_rules! perft {
        ($name:ident, $depth:expr, $perft_val:expr, $fen:expr $(, #[$attr:meta])?) => {
            #[test]
            $(#[$attr])?
            fn $name() {
                let board = Board::parse_fen($fen).expect("fen incorrect");
                assert_eq!(board.perft($depth), $perft_val, "failed at depth {}", $depth)
            }
        };
    }

    perft!(
        custom_1,
        3,
        12737,
        "rnb2k1r/pp2b1pp/4p1n1/qBPpPp2/5P2/4BNP1/PPP4P/RN1QK2R w KQ - 7 10"
    );
    perft!(
        custom_2,
        1,
        37,
        "rnb2k1r/pp4pp/4p1n1/qBPpPp2/5P1b/4BNP1/PPPQ3P/RN2K2R w KQ - 0 1"
    );
    perft!(kiwipete_1, 1, 48, KIWIPETE_FEN);
    perft!(kiwipete_2, 2, 2039, KIWIPETE_FEN);
    perft!(kiwipete_3, 3, 97_862, KIWIPETE_FEN);
    perft!(kiwipete_4, 4, 4_085_603, KIWIPETE_FEN);
    //use --ignore to test higher depths
    perft!(kiwipete_5, 5, 193_690_690, KIWIPETE_FEN,  #[ignore]);
    perft!(kiwipete_6, 6, 8_031_647_685, KIWIPETE_FEN, #[ignore]);
}
