use color_eyre::eyre::{OptionExt, Result, eyre};
use std::{io::{self, Read, Write}, str::SplitWhitespace};
use crate::{board::{self, board::Board}, common::{r#move::Move, square::Square}};

#[derive(Debug)]
enum ReceiveUci {
    Go {wtime: u32, btime: u32, winc: u32, binc: u32},
    Position(Board),
    Quit,
    Uci,
    UciNewGame,
    IsReady,
}

impl ReceiveUci {
    fn parse(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        match uci.next()?.to_ascii_lowercase().as_str() {
            "go" => ReceiveUci::parse_go(uci),
            "position" => ReceiveUci::parse_pos(uci),
            "quit" => Some(ReceiveUci::Quit),
            "uci" => Some(ReceiveUci::Uci),
            "ucinewgame" => Some(ReceiveUci::UciNewGame),
            "isready" => Some(ReceiveUci::IsReady),
            _ => None
        }
    }

    fn parse_go(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        let mut wtime: u32 = 0;
        let mut btime: u32 = 0;
        let mut winc: u32 = 0;
        let mut binc: u32 = 0;
        while let (Some(arg), Some(raw_val)) = (uci.next(), uci.next()) {
            let val = raw_val.parse::<u32>().ok()?;
            match arg {
                "wtime" => {wtime = val;},
                "btime" => {btime = val;},
                "winc" => {winc = val;},
                "binc" => {binc = val;},
                _ => {return None;}
            }
        };
        Some(ReceiveUci::Go { wtime, btime, winc, binc})
    }

    fn parse_pos(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        let pos = uci.next()?.to_ascii_lowercase();
        println!("got to startpos: {pos}");
        let mut board: Board = match pos.as_str() {
            "startpos" => Board::startpos(),
            "fen" => {
                let fen = uci.next()?;
                Board::parse_fen(fen).ok()?
            },
            _ => {return None;}
        };
        while let Some(mv) = uci.next() {
            let (raw_src , raw_dst ) = mv.split_at_checked(2)?;
            let src = Square::parse(raw_src)?;
            let dst = Square::parse(raw_dst)?;
            let verified_move = board.get_moves().find(src, dst)?;
            board.do_move(verified_move);
        }
        Some(ReceiveUci::Position(board))
    }
}

#[derive(Debug)]
enum SendUci {
    BestMove(Move),
    UciOk,
    Id{name: String, author: String},
}

#[derive(Debug)]
pub struct Engine {}

impl Engine {
    pub fn default() -> Engine {
        Engine {}
    }

    pub fn run(&mut self) -> Result<()> {
        let args = std::env::args().skip(1).collect::<Vec<String>>();
        println!("args: {:?}", args);
        if args == vec!["bench".to_string()] {
            Engine::run_bench()?;
        } else if args.is_empty() {
            self.run_uci()?;
        } else {
            return Err(eyre!("invalid args: {:?}", args));
        }
        Ok(())
    }

    fn run_uci(&mut self) -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut buffer = String::new();
        loop {
            buffer.clear();
            stdin.read_line(&mut buffer)?;
            let raw: std::str::SplitWhitespace<'_> = buffer.split_whitespace();
            let Some(rec_uci) = ReceiveUci::parse(raw) else {continue;};
            println!("rec_uci {:?}", rec_uci);
            let _ = stdout.flush();
        }
    }
}