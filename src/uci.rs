use std::{
    io::{self, Write}, str::SplitWhitespace,
};

use color_eyre::eyre::{Ok, OptionExt, Result, eyre};

use crate::{
    board::board::Board,
    common::{colour::Colour, square::Square},
    search::{
        search::{Search, SearchStdOut},
        time::TimeManager,
    },
};

#[derive(Debug)]
enum ReceiveUci {
    Go {
        wtime: Option<u32>,
        btime: Option<u32>,
        winc:  Option<u32>,
        binc:  Option<u32>,
        depth: Option<u8>,
    },
    Stop,
    Position(Board),
    Quit,
    Uci,
    UciNewGame,
    IsReady,
    Bench,
    Perft{depth: u8, fen: Option<String>},
}

impl ReceiveUci {

    fn parse(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        match uci.next()?.to_ascii_lowercase().as_str() {
            "go" => ReceiveUci::parse_go(uci),
            "stop" => Some(ReceiveUci::Stop),
            "position" => ReceiveUci::parse_pos(uci),
            "quit" => Some(ReceiveUci::Quit),
            "uci" => Some(ReceiveUci::Uci),
            "ucinewgame" => Some(ReceiveUci::UciNewGame),
            "isready" => Some(ReceiveUci::IsReady),
            "bench" => Some(ReceiveUci::Bench),
            "perft" => ReceiveUci::parse_perft(uci),
            _ => None,
        }
    }

    fn parse_perft(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        let (Some(depth), fen_opt, None) = (uci.next(), uci.next(), uci.next()) else {
            return None;
        };
        let depth = depth.parse::<u8>().ok()?;
        return Some(ReceiveUci::Perft{depth, fen: fen_opt.map(|s|{s.to_string()})});
    }

    fn parse_go(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        //reasonable defaults
        let mut wtime: Option<u32> = Some(1000);
        let mut btime: Option<u32> = Some(1000);
        let mut winc:  Option<u32> = Some(0);
        let mut binc:  Option<u32> = Some(0);
        let mut depth: Option<u8>  = None;
        while let (Some(arg), raw_val) = (uci.next(), uci.next()) {

            // TODO: fix that it works weird if you put "infinite" as like the 3rd argument

            if arg == "infinite" {
                wtime = None;
                btime = None;
                winc = None;
                binc = None;
                depth = None;
                break;
            }

            let Some(raw_val) = raw_val else {
                break;
            };

            let val = raw_val.parse::<u32>().ok()?;
            match arg {
                "wtime" => {
                    wtime = Some(val);
                }
                "btime" => {
                    btime = Some(val);
                }
                "winc" => {
                    winc = Some(val);
                }
                "binc" => {
                    binc = Some(val);
                }
                "depth" => {
                    depth = Some(u8::try_from(val).ok()?);
                }
                _ => {
                    return None;
                }
            }
        }

        Some(ReceiveUci::Go {
            wtime,
            btime,
            winc,
            binc,
            depth
        })
    }

    fn parse_pos(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        let pos = uci.next()?.to_ascii_lowercase();
        let mut board: Board = match pos.as_str() {
            "startpos" => Board::startpos(),
            "fen" => {
                let fen = uci.next()?;
                Board::parse_fen(fen).ok()?
            }
            _ => {
                return None;
            }
        };

        for mv in uci {
            let (raw_src, raw_dst) = mv.split_at_checked(2)?;
            let src = Square::parse(raw_src)?;
            let dst = Square::parse(raw_dst)?;
            let verified_move = board.get_moves().find(src, dst)?;
            board.do_move(verified_move);
        }
        Some(ReceiveUci::Position(board))
    }
}

#[derive(Debug, PartialEq)]
pub enum Abort {
    Yes,
    No,
}

#[derive(Debug)]
pub struct Engine {
    pub position: Option<Board>,
    pub search: Search,
}

impl Engine {
    pub fn default() -> Engine {
        Engine {
            position: None,
            search: Search::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let args = std::env::args().skip(1).collect::<Vec<String>>();
        if args == vec!["bench".to_string()] {
            self.run_bench()?;
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
            let Some(rec_uci) = ReceiveUci::parse(raw) else {
                continue;
            };
            let abort = self.do_uci_command(rec_uci)?;
            let _ = stdout.flush();
            if abort == Abort::Yes {
                break;
            }
        }
        Ok(())
    }

    fn do_uci_command(&mut self, uci: ReceiveUci) -> Result<Abort> {
        match uci {
            ReceiveUci::Quit => Ok(Abort::Yes),
            ReceiveUci::Uci => {
                println!("id name ???");
                println!("id author Drex");
                println!("uciok");
                Ok(Abort::No)
            }
            ReceiveUci::IsReady => {
                println!("readyok");
                Ok(Abort::No)
            }
            ReceiveUci::Position(board) => {
                self.position = Some(board);
                Ok(Abort::No)
            }
            ReceiveUci::UciNewGame => {
                *self = Engine::default();
                Ok(Abort::No)
            }
            ReceiveUci::Bench => {
                self.run_bench()?;
                Ok(Abort::No)
            }
            ReceiveUci::Go {
                wtime,
                btime,
                winc,
                binc,
                depth,
            } => {
                let board = self
                    .position
                    .clone()
                    .ok_or_eyre("use the 'position' command to set a position")?;
                let time_and_inc: Option<(u32, u32)> = match board.stm() {
                    Colour::White => wtime.zip(winc),
                    Colour::Black => btime.zip(binc),
                };
                let time_manager = TimeManager::new(depth, None, time_and_inc);
                let output = SearchStdOut::BestMove;
                self.search.start_search(board, time_manager, output)?;
                Ok(Abort::No)
            },
            ReceiveUci::Stop => {
                self.search.stop();
                Ok(Abort::No)
            },
            ReceiveUci::Perft { depth, fen } => {
                let board = match fen {
                    Some(fen) => Board::parse_fen(&fen)?,
                    None => self.position.clone().unwrap_or(Board::startpos()),
                };
                Engine::run_perft(board, depth);
                Ok(Abort::No)
            }
        }
    }

    pub fn shutdown(self) -> Result<()> {
        self.search.quit()?;
        Ok(())
    }
}
