use std::{
    sync::{
        Arc,
        atomic::{AtomicU8, AtomicU64},
        mpsc::{self, Receiver, Sender, channel},
    },
    thread::{self, JoinHandle},
};

use thiserror::Error;

use crate::{
    board::board::Board,
    search::{search::negamax, time::TimeManager},
};

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
    // TODO: Make an ALL
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
