use std::{
    io::{self, Write},
    prelude::v1::Ok,
    str::SplitWhitespace,
};

use color_eyre::eyre::{Result, eyre};

use crate::{
    board::board::Board,
    common::colour::Colour,
    search::{
        time::TimeManager,
        worker::{Search, SearchStdOut},
    },
};

#[derive(Debug, PartialEq)]
enum ReceiveUci {
    Go {
        wtime: Option<u32>,
        btime: Option<u32>,
        winc: Option<u32>,
        binc: Option<u32>,
        depth: Option<u8>,
    },
    Stop,
    Position(Board),
    Quit,
    Uci,
    UciNewGame,
    IsReady,
    Bench,
    Perft(u8),
}

impl ReceiveUci {
    fn parse(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        let Some(uci_raw) = uci.next() else {
            println!("info type to give a command");
            return None;
        };
        match uci_raw.to_ascii_lowercase().as_str() {
            "go" => ReceiveUci::parse_go(uci),
            "position" => ReceiveUci::parse_pos(uci),
            "stop" => Some(ReceiveUci::Stop),
            "quit" => Some(ReceiveUci::Quit),
            "uci" => Some(ReceiveUci::Uci),
            "ucinewgame" => Some(ReceiveUci::UciNewGame),
            "isready" => Some(ReceiveUci::IsReady),
            "bench" => Some(ReceiveUci::Bench),
            _ => {
                println!("info unknown command '{uci_raw}'");
                None
            }
        }
    }

    fn parse_fen(uci: &mut SplitWhitespace<'_>) -> Option<Board> {
        let fen = (0..6)
            .map(|_| uci.next().unwrap_or_default())
            .collect::<Vec<_>>()
            .join(" ");

        let board = match Board::parse_fen(&fen) {
            Ok(board) => board,
            Err(report) => {
                println!("info fen {report}");
                return None;
            }
        };
        Some(board)
    }

    fn parse_go(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        //reasonable defaults
        let mut wtime: Option<u32> = None;
        let mut btime: Option<u32> = None;
        let mut winc: Option<u32> = None;
        let mut binc: Option<u32> = None;
        let mut depth: Option<u8> = None;
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

            if arg == "perft" {
                if uci.next().is_some() {
                    println!("info no text allowed after the command");
                    return None;
                }
                let Some(depth_str) = raw_val else {
                    println!("info numeric argument expected after 'perft'");
                    return None;
                };
                let Ok(depth) = depth_str.parse::<u8>() else {
                    println!("info '{depth_str}' not a valid depth");
                    return None;
                };
                return Some(ReceiveUci::Perft(depth));
            }

            let Some(raw_val) = raw_val else {
                println!("info second numeric argument expected after '{arg}'");
                return None;
            };

            let Ok(val) = raw_val.parse::<u32>() else {
                println!("info '{raw_val}' not a valid number");
                return None;
            };

            // .or() here to make sure the pair is filled for if a partial command is given
            // at least there exists a reasonable default rather than none
            match arg {
                "wtime" => {
                    wtime = Some(val);
                    winc = winc.or(Some(0u32));
                }
                "btime" => {
                    btime = Some(val);
                    binc = binc.or(Some(0u32));
                }
                "winc" => {
                    winc = Some(val);
                    wtime = wtime.or(Some(0u32));
                }
                "binc" => {
                    binc = Some(val);
                    btime = btime.or(Some(0u32));
                }
                "depth" => {
                    depth = Some(u8::try_from(val).ok()?);
                }
                _ => {
                    println!("info unknown argument '{arg}'");
                    return None;
                }
            }
        }

        Some(ReceiveUci::Go {
            wtime,
            btime,
            winc,
            binc,
            depth,
        })
    }

    fn parse_pos(mut uci: SplitWhitespace<'_>) -> Option<ReceiveUci> {
        let Some(pos) = uci.next() else {
            println!("info after pos try 'startpos' or 'fen ...'");
            return None;
        };

        let board: Board = match pos.to_ascii_lowercase().as_str() {
            "startpos" => Board::startpos(),
            "fen" => ReceiveUci::parse_fen(&mut uci)?,
            _ => {
                println!("info unknown argument '{pos}'");
                return None;
            }
        };

        let Some(mv_command) = uci.next() else {
            return Some(ReceiveUci::Position(board));
        };

        if mv_command != "moves" {
            println!("info leave blank or try 'moves <mv1> <mv2> ...'");
            return None;
        }

        let uci_mv = uci.collect();
        let board = match board.apply_uci_moves(uci_mv) {
            Err(message) => {
                println!("info {message}");
                return None;
            }
            Ok(board) => board,
        };

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
    pub position: Board,
    pub search: Search,
}

impl Engine {
    pub fn default() -> Engine {
        Engine {
            position: Board::startpos(),
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
                println!("id name blini");
                println!("id author Drex");
                println!("uciok");
                Ok(Abort::No)
            }
            ReceiveUci::IsReady => {
                println!("readyok");
                Ok(Abort::No)
            }
            ReceiveUci::Position(board) => {
                self.position = board;
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
                let board = self.position.clone();
                let time_and_inc: Option<(u32, u32)> = match board.stm() {
                    Colour::White => wtime.zip(winc),
                    Colour::Black => btime.zip(binc),
                };
                let time_manager = TimeManager::new(depth, None, time_and_inc);
                let output = SearchStdOut::BestMove;
                self.search.start_search(board, time_manager, output)?;
                Ok(Abort::No)
            }
            ReceiveUci::Stop => {
                self.search.stop();
                Ok(Abort::No)
            }
            ReceiveUci::Perft(depth) => {
                Engine::run_perft(self.position.clone(), depth);
                Ok(Abort::No)
            }
        }
    }

    pub fn shutdown(self) -> Result<()> {
        self.search.quit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::board::Board, uci::ReceiveUci};

    #[test]
    fn test_receive_uci() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        // Basic commands
        assert_eq!(
            ReceiveUci::parse("uci".split_whitespace()),
            Some(ReceiveUci::Uci)
        );
        assert_eq!(
            ReceiveUci::parse("isready".split_whitespace()),
            Some(ReceiveUci::IsReady)
        );
        assert_eq!(
            ReceiveUci::parse("ucinewgame".split_whitespace()),
            Some(ReceiveUci::UciNewGame)
        );
        assert_eq!(
            ReceiveUci::parse("stop".split_whitespace()),
            Some(ReceiveUci::Stop)
        );
        assert_eq!(
            ReceiveUci::parse("quit".split_whitespace()),
            Some(ReceiveUci::Quit)
        );
        assert_eq!(
            ReceiveUci::parse("bench".split_whitespace()),
            Some(ReceiveUci::Bench)
        );

        // Position
        let board = Board::startpos()
            .apply_uci_moves(vec!["e2e4", "e7e5"])
            .unwrap();
        assert_eq!(
            ReceiveUci::parse("position startpos".split_whitespace()),
            Some(ReceiveUci::Position(Board::startpos()))
        );
        assert_eq!(
            ReceiveUci::parse("position startpos moves e2e4 e7e5".split_whitespace()),
            Some(ReceiveUci::Position(board))
        );
        assert_eq!(
            ReceiveUci::parse(format!("position fen {fen}").split_whitespace()),
            Some(ReceiveUci::Position(Board::startpos()))
        );

        // Go
        assert_eq!(
            ReceiveUci::parse("go".split_whitespace()),
            Some(ReceiveUci::Go {
                wtime: None,
                btime: None,
                winc: None,
                binc: None,
                depth: None,
            })
        );
        assert_eq!(
            ReceiveUci::parse("go wtime 1000".split_whitespace()),
            Some(ReceiveUci::Go {
                wtime: Some(1000),
                btime: None,
                winc: Some(0),
                binc: None,
                depth: None,
            })
        );
        assert_eq!(
            ReceiveUci::parse("go btime 2000".split_whitespace()),
            Some(ReceiveUci::Go {
                wtime: None,
                btime: Some(2000),
                winc: None,
                binc: Some(0),
                depth: None,
            })
        );
        assert_eq!(
            ReceiveUci::parse("go winc 100 binc 200".split_whitespace()),
            Some(ReceiveUci::Go {
                wtime: Some(0),
                btime: Some(0),
                winc: Some(100),
                binc: Some(200),
                depth: None,
            })
        );
        assert_eq!(
            ReceiveUci::parse("go depth 12".split_whitespace()),
            Some(ReceiveUci::Go {
                wtime: None,
                btime: None,
                winc: None,
                binc: None,
                depth: Some(12),
            })
        );
        assert_eq!(
            ReceiveUci::parse(
                "go wtime 10000 btime 8000 winc 100 binc 200 depth 10".split_whitespace()
            ),
            Some(ReceiveUci::Go {
                wtime: Some(10000),
                btime: Some(8000),
                winc: Some(100),
                binc: Some(200),
                depth: Some(10),
            })
        );

        // Invalid / unsupported commands
        assert_eq!(ReceiveUci::parse("".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("foo".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("uc".split_whitespace()), None);

        // Invalid position commands
        assert_eq!(ReceiveUci::parse("position".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("position foo".split_whitespace()), None);
        assert_eq!(
            ReceiveUci::parse("position startpos foo".split_whitespace()),
            None
        );
        assert_eq!(ReceiveUci::parse("position fen".split_whitespace()), None);

        // Invalid go commands
        assert_eq!(ReceiveUci::parse("go wtime".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("go wtime foo".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("go depth foo".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("go depth 256".split_whitespace()), None);

        // Invalid perft commands
        assert_eq!(ReceiveUci::parse("perft".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("perft foo".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("perft 0".split_whitespace()), None);
        assert_eq!(ReceiveUci::parse("perft 256".split_whitespace()), None);
    }
}
