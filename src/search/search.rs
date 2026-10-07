use std::sync::{Arc, atomic::Ordering};

use crate::{
    common::r#move::Move,
    eval::material::{Score, eval},
    search::{
        position::Position,
        worker::{SearchError, SearchStdOut, SharedData},
    },
};

#[derive(Debug, Clone, Copy)]
struct AlphaBeta {
    alpha: Score,
    beta: Score,
}

impl AlphaBeta {
    #[inline]
    fn new() -> AlphaBeta {
        AlphaBeta {
            alpha: Score::MIN,
            beta: Score::MAX,
        }
    }

    #[inline]
    fn narrow(&mut self, score: Score) {
        self.alpha = Ord::max(score, self.alpha)
    }

    #[inline]
    fn switch(&self) -> AlphaBeta {
        AlphaBeta {
            alpha: -self.beta,
            beta: -self.alpha,
        }
    }

    #[inline]
    fn best_score(&self) -> Score {
        self.alpha
    }

    #[inline]
    fn prune(&self) -> bool {
        self.alpha >= self.beta
    }
}

#[inline]
pub fn negamax(shared: Arc<SharedData>, output: SearchStdOut) -> Result<(), SearchError> {
    let moves = shared.board.get_moves();

    //checkmate check
    let length = moves.iter().len();
    if length == 0 {
        return Err(SearchError::NoLegalMoves);
    }

    //early return check
    let first_move = moves.0[0];
    if length == 1 {
        shared.depth.store(0, Ordering::Relaxed);
        shared.nodes.store(1, Ordering::Relaxed);
        if output == SearchStdOut::BestMove {
            println!("bestmove {}", first_move.uci());
        }
        return Ok(());
    }

    //iterative deepening
    let mut ply: u8 = 1;
    let mut node_count: u64 = 0;
    let mut prev_best_move: Move = first_move;

    loop {
        let mut alpha_beta = AlphaBeta::new();
        let mut best_move = first_move;

        for mv in moves.clone() {
            let mut new_pos = Position::new(shared.board.clone(), usize::from(ply));
            new_pos.do_move(mv);

            let Some(score) = negamax_recursion(
                &mut new_pos,
                ply - 1,
                &mut node_count,
                alpha_beta.switch(),
                &shared,
            )
            .map(|s| -s) else {
                // recursion only returns none if hit hard limit
                shared.nodes.store(node_count, Ordering::Relaxed);
                if output == SearchStdOut::BestMove {
                    println!("bestmove {}", prev_best_move.uci());
                }
                return Ok(());
            };

            if score > alpha_beta.alpha {
                alpha_beta.alpha = score;
                best_move = mv;
            }
        }

        //update shared data
        shared.depth.store(ply, Ordering::Relaxed);
        shared.nodes.store(node_count, Ordering::Relaxed);

        //info output
        if output == SearchStdOut::BestMove {
            let nodes = shared.nodes.load(Ordering::Relaxed);
            let nps = shared.time_manager.calc_nps(nodes);
            let time = shared.time_manager.time();
            println!(
                "info depth {ply} seldepth {ply} score cp {} nodes {nodes} nps {nps} hashfull 0 pv {} time {time}",
                alpha_beta.alpha,
                best_move.uci()
            );
        }

        // soft limit
        if shared.time_manager.soft_limit() {
            if output == SearchStdOut::BestMove {
                println!("bestmove {}", best_move.uci());
            }
            return Ok(());
        }

        // depth limit
        if shared.time_manager.depth_limit(ply) {
            if output == SearchStdOut::BestMove {
                println!("bestmove {}", best_move.uci());
            }
            return Ok(());
        }

        prev_best_move = best_move;
        ply += 1;
    }
}

#[inline]
fn negamax_recursion(
    position: &mut Position,
    depth: u8,
    node_count: &mut u64,
    mut alpha_beta: AlphaBeta,
    shared: &Arc<SharedData>,
) -> Option<Score> {
    *node_count += 1;
    if (*node_count).is_multiple_of(0x1000)
        && (shared.time_manager.hard_limit() || shared.time_manager.node_limit(*node_count))
    {
        return None;
    }

    if position.is_draw() {
        return Some(Score::DRAW);
    }

    if depth == 0 {
        return Some(eval(position.board()));
    }

    let moves = position.board().get_moves();

    if moves.is_empty() {
        if position.board().in_check() {
            return Some(-Score::CHECKMATE);
        } else {
            return Some(Score::DRAW);
        }
    }

    for mv in moves {
        let prev_board = position.board().clone();
        position.do_move(mv);
        let score =
            -negamax_recursion(position, depth - 1, node_count, alpha_beta.switch(), shared)?;
        position.undo_move(prev_board);
        alpha_beta.narrow(score);
        if alpha_beta.prune() {
            break;
        }
    }

    Some(alpha_beta.best_score())
}
