use std::{
    cmp::max,
    sync::{
        Arc,
        atomic::{AtomicU8, AtomicU64, Ordering},
        mpsc::{self, Receiver, Sender, channel},
    },
    thread::{self, JoinHandle},
};

use rand::{RngExt, rngs::ThreadRng};
use thiserror::Error;

use crate::{board::board::Board, common::r#move::Move, search::time::TimeManager};

#[derive(Debug)]
pub struct Search {
    shared_data: Option<Arc<SharedData>>,
    worker_thread: JoinHandle<Result<(), SearchError>>,
    sender: Sender<WorkerCommand>,
}

impl Search {
    pub fn new() -> Search {
        let (sender, receiver) = channel::<WorkerCommand>();
        let worker_thread = thread::spawn(|| Search::worker_loop(receiver));
        Search {
            worker_thread,
            sender,
            shared_data: None,
        }
    }

    #[inline]
    pub fn stop(&self) {
        let Some(shared) = &self.shared_data else {
            return;
        };
        shared.time_manager.stop();
    }

    fn worker_loop(cmds: Receiver<WorkerCommand>) -> Result<(), SearchError> {
        for cmd in &cmds {
            match cmd {
                WorkerCommand::Search((shared, output)) => {
                    negamax(shared.clone(), output)?;
                }
                WorkerCommand::Stop => {
                    break;
                }
                WorkerCommand::CallerWait(sender) => {
                    sender.send(()).map_err(|_| SearchError::CallerWait)?;
                }
            }
        }
        Ok(())
    }

    pub fn start_search(
        &mut self,
        board: Board,
        time_manager: TimeManager,
        output: SearchStdOut,
    ) -> Result<Arc<SharedData>, SearchError> {
        /*
        later for multiple threads here would be the place to clone the shared
        data arc
        */
        let shared_data = SharedData::new(board, time_manager);
        self.sender
            .send(WorkerCommand::Search((shared_data.clone(), output)))
            .map_err(|_| SearchError::SendCommand)?;
        self.shared_data = Some(shared_data.clone());
        Ok(shared_data)
    }

    pub fn wait(&self) -> Result<(), SearchError> {
        let (sender, reciever) = mpsc::channel::<()>();
        self.sender
            .send(WorkerCommand::CallerWait(sender))
            .map_err(|_| SearchError::CallerWait)?;
        reciever.recv().map_err(|_| SearchError::CallerWait)?;
        Ok(())
    }

    pub fn quit(self) -> Result<(), SearchError> {
        self.sender
            .send(WorkerCommand::Stop)
            .map_err(|_| SearchError::Stop)?;
        self.worker_thread.join().map_err(|_| SearchError::Stop)?
    }
}

#[derive(Debug)]
pub struct SharedData {
    pub board: Board,
    pub time_manager: TimeManager,
    pub depth: AtomicU8,
    pub nodes: AtomicU64,
}

impl SharedData {
    pub fn new(board: Board, time_manager: TimeManager) -> Arc<SharedData> {
        Arc::new(SharedData {
            board,
            time_manager,
            depth: AtomicU8::new(0),
            nodes: AtomicU64::new(0),
        })
    }
}

#[derive(Debug)]
enum WorkerCommand {
    Search((Arc<SharedData>, SearchStdOut)),
    CallerWait(Sender<()>),
    Stop,
}

#[derive(Debug, PartialEq)]
pub enum SearchStdOut {
    BestMove,
    None,
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("no legal moves")]
    NoLegalMoves,
    #[error("failed to send command to worker")]
    SendCommand,
    #[error("failed to send stop command to worker")]
    Stop,
    #[error("Couldnt wait for the worker thread")]
    CallerWait,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Score(i16);

impl Score {
    #[inline]
    const fn new() -> Score {
        Score(-10_000)
    }
}

impl std::ops::Neg for Score {
    type Output = Score;
    fn neg(self) -> Self::Output {
        Score(-self.0)
    }
}

#[inline]
fn random_eval(_board: Board, rng: &mut ThreadRng) -> Score {
    Score(rng.random_range(-10_000..=10_000))
}

#[inline]
pub fn negamax(shared: Arc<SharedData>, output: SearchStdOut) -> Result<(), SearchError> {
    let mut rng = rand::rng();
    let moves = shared.board.get_moves();

    //checkmate check
    let length = moves.iter().len();
    if length == 0 {
        return Err(SearchError::NoLegalMoves);
    }

    //early return check
    let first_move = moves.clone().into_iter().next().unwrap();
    if length == 1 {
        shared.depth.store(0, Ordering::Relaxed);
        shared.nodes.store(1, Ordering::Relaxed);
        if output == SearchStdOut::BestMove {
            println!("{}", first_move.uci());
        }
        return Ok(());
    }

    //iterative deepening
    let mut ply = 0;
    let mut node_count: u64 = 0;
    let mut prev_best_move: Move = first_move;

    loop {
        let mut best_score = Score::new();
        let mut best_move = first_move;

        for mv in moves.clone() {
            let mut new_board = shared.board.clone();
            new_board.do_move(mv);

            let Some(score) =
                negamax_recursion(new_board, ply, &mut rng, &mut node_count, &shared).map(|s| -s)
            else {
                // recursion only returns none if hit hard limit
                shared.nodes.store(node_count, Ordering::Relaxed);
                if output == SearchStdOut::BestMove {
                    println!("{}", prev_best_move.uci());
                }
                return Ok(());
            };

            if score > best_score {
                best_score = score;
                best_move = mv;
            }
        }

        //update shared data
        shared.depth.store(ply, Ordering::Relaxed);
        shared.nodes.store(node_count, Ordering::Relaxed);

        // soft limit
        if shared.time_manager.soft_limit() {
            if output == SearchStdOut::BestMove {
                println!("{}", best_move.uci());
            }
            return Ok(());
        }

        // depth limit
        if shared.time_manager.depth_limit(ply) {
            if output == SearchStdOut::BestMove {
                println!("{}", best_move.uci());
            }
            return Ok(());
        }

        prev_best_move = best_move;
        ply += 1;
    }
}

#[inline]
fn negamax_recursion(
    board: Board,
    depth: u8,
    rng: &mut ThreadRng,
    node_count: &mut u64,
    shared: &Arc<SharedData>,
) -> Option<Score> {
    if depth == 0 {
        *node_count += 1;
        return Some(random_eval(board, rng));
    }

    let moves = board.get_moves();
    let mut best_score = Score::new();
    for mv in moves {
        if (*node_count).is_multiple_of(0x400)
            && (shared.time_manager.hard_limit() || shared.time_manager.node_limit(*node_count))
        {
            return None;
        }

        let mut new_board = board.clone();
        new_board.do_move(mv);
        let score = -negamax_recursion(new_board, depth - 1, rng, node_count, shared)?;
        best_score = max(best_score, score);
    }
    *node_count += 1;
    Some(best_score)
}
