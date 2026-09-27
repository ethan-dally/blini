use std::{
    cmp::max,
    sync::{
        Arc,
        mpsc::{
            Receiver,
            Sender,
            channel,
        },
    },
    thread::{
        self,
        JoinHandle,
    },
};

use rand::{
    RngExt,
    rngs::ThreadRng,
};
use thiserror::Error;

use crate::{
    board::board::Board,
    common::r#move::Move,
    search::time::{
        TimeManager,
        TimeRemaining,
    },
};

#[derive(Debug)]
pub struct Search {
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
        }
    }

    fn worker_loop(cmds: Receiver<WorkerCommand>) -> Result<(), SearchError> {
        for cmd in &cmds {
            match cmd {
                WorkerCommand::Search(shared) => {
                    let mv = negamax(shared.clone())?;
                    println!("{}", mv.uci());
                }
                WorkerCommand::Stop => {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn start_search(&self, board: Board, time_manager: TimeManager) -> Result<(), SearchError> {
        /*
        later for multiple threads here would be the place to clone the shared
        data arc
        */
        let shared_data = SharedData::new(board, time_manager);
        self.sender
            .send(WorkerCommand::Search(shared_data))
            .map_err(|_| SearchError::Start)?;
        Ok(())
    }

    pub fn stop(self) -> Result<(), SearchError> {
        self.sender
            .send(WorkerCommand::Stop)
            .map_err(|_| SearchError::Stop)?;
        self.worker_thread.join().map_err(|_| SearchError::Stop)?
    }
}

#[derive(Debug)]
struct SharedData {
    board: Board,
    time_manager: TimeManager,
}

impl SharedData {
    fn new(board: Board, time_manager: TimeManager) -> Arc<SharedData> {
        Arc::new(SharedData {
            board,
            time_manager,
        })
    }
}

#[derive(Debug)]
enum WorkerCommand {
    Search(Arc<SharedData>),
    Stop,
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("no legal moves")]
    NoLegalMoves,
    #[error("failed to start search worker")]
    Start,
    #[error("failed to send stop command to worker")]
    Stop,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Score(i16);

impl Score {
    fn new() -> Score {
        Score(-10_000)
    }
}

fn random_eval(_board: Board, rng: &mut ThreadRng) -> Score {
    Score(rng.random_range(-10_000..=10_000))
}

fn negamax(shared: Arc<SharedData>) -> Result<Move, SearchError> {
    let mut rng = rand::rng();
    let moves = shared.board.get_moves();

    //early return check
    let length = moves.iter().len();
    if length == 0 {
        return Err(SearchError::NoLegalMoves);
    }
    let first_move = moves.clone().into_iter().next().unwrap();
    if length == 1 {
        return Ok(first_move);
    }

    //iterative deepening
    let mut ply = 0;
    let mut prev_ply_best_move = first_move;

    loop {
        let mut ply_best_score = Score::new();
        let mut ply_best_move = first_move;

        for mv in moves.clone() {
            let mut new_board = shared.board.clone();
            new_board.do_move(mv);
            let Some(score) = negamax_recursion(new_board, ply, &mut rng, shared.clone()) else {
                //hit the hard limit
                return Ok(prev_ply_best_move);
            };

            if score > ply_best_score {
                ply_best_score = score;
                ply_best_move = mv;
            }
        }

        // soft limit
        if shared.time_manager.poll() == TimeRemaining::SoftLimit {
            return Ok(ply_best_move);
        }
        prev_ply_best_move = ply_best_move;
        ply += 1;
    }
}

fn negamax_recursion(
    board: Board,
    depth: u8,
    rng: &mut ThreadRng,
    shared: Arc<SharedData>,
) -> Option<Score> {
    // TODO: add nodes count to not poll all the time???
    if depth == 0 {
        return Some(random_eval(board, rng));
    }

    let moves = board.get_moves();
    let mut best_score = Score::new();

    for mv in moves {
        if shared.time_manager.poll() == TimeRemaining::HardLimit {
            return None;
        }
        let mut new_board = board.clone();
        new_board.do_move(mv);
        let score = negamax_recursion(new_board, depth - 1, rng, shared.clone())?;
        best_score = max(best_score, score);
    }
    Some(best_score)
}
