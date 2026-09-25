use std::time::Instant;

use color_eyre::eyre::{Result, eyre};

use crate::{board::board::Board, uci::Engine};

pub const DEPTH: u8 = 4;

const BENCH_FENS: &[&str] = &[
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
];

impl Engine {
    pub fn run_bench() -> Result<()> {

        let boards: Vec<_> = BENCH_FENS.iter().filter_map(|f|{
            Board::parse_fen(*f).ok()
        }).collect();

        let timer = Instant::now();
        let mut count: u128 = 0;

        for board in boards {
            count += board.perft(DEPTH) as u128;
        }

        let time = timer.elapsed().as_millis();
        if time == 0u128 {
            return Err(eyre!("couldnt parse bench fens, or very low depth"));
        }

        let nps = count * 1000u128 / time;
        println!("nodes: {count}, time: {time}, nps: {nps}");
        Ok(())
    }
}