use std::{
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

use color_eyre::eyre::Result;

use crate::{
    board::board::Board,
    search::{search::SearchStdOut, time::TimeManager},
    uci::Engine,
};

pub const DEPTH: u8 = 4;

const BENCH_FENS: &[&str] = &[
    "rnbq1k1r/ppp1bppp/4pn2/8/2B5/2NP1N2/PPP2PPP/R1BQR1K1 b - - 2 8",
    "rnbq1k1r/pp2bppp/4pn2/2p5/2B2B2/2NP1N2/PPP2PPP/R2QR1K1 b - - 1 9",
    "r1bq1k1r/pp2bppp/2n1pn2/2p5/2B1NB2/3P1N2/PPP2PPP/R2QR1K1 b - - 3 10",
    "r1bq1k1r/pp2bppp/2n1p3/2p5/2B1PB2/5N2/PPP2PPP/R2QR1K1 b - - 0 11",
    "r1b2k1r/pp2bppp/2n1p3/2p5/2B1PB2/5N2/PPP2PPP/3RR1K1 b - - 0 12",
    "r1b1k2r/pp2bppp/2n1p3/2p5/2B1PB2/2P2N2/PP3PPP/3RR1K1 b - - 0 13",
    "2rr2k1/1p3ppp/p2P1p2/2Q5/5P2/P1bb1NN1/R5PP/2R4K b - - 0 26",
    "3r2k1/1p3ppp/p2P1p2/2r5/5P2/P1bb1N2/R3N1PP/2R4K b - - 1 27",
    "8/8/2R2pk1/p6p/7P/b4K2/r1N5/8 b - - 3 51",
    "8/8/2R2pk1/p6p/7P/4NK2/rb6/8 b - - 5 52",
    "2R5/8/5pk1/7p/p6P/4NK2/rb6/8 b - - 1 53",
    "6R1/8/5pk1/7p/p6P/4NK2/1b6/r7 b - - 3 54",
];

impl Engine {
    // floor division is wanted here to display whole nums
    #[allow(clippy::integer_division)]
    pub fn run_bench(&mut self) -> Result<()> {
        let boards: Vec<_> = BENCH_FENS
            .iter()
            .filter_map(|f| Board::parse_fen(f).ok())
            .collect();

        let mut total_time = Duration::ZERO;
        let mut total_nodes = 0u128;

        for board in boards {
            let time_manager = TimeManager::new(Some(DEPTH), None, None);
            let time = Instant::now();
            let data = self
                .search
                .start_search(board, time_manager, SearchStdOut::None)?;
            self.search.wait()?;
            total_time += time.elapsed();
            total_nodes += u128::from(data.nodes.load(Ordering::Relaxed));
        }

        // +1 to avoid /0 error, too small to be noticable
        let total_nanos = total_time.as_nanos() + 1;
        let nps = total_nodes * 1_000_000_000u128 / total_nanos;
        println!("nodes {total_nodes} nps {nps}");
        Ok(())
    }
}
