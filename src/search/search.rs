use std::cmp::max;

use crate::{board::board::Board, common::r#move::Move};
use rand::{Rng, RngExt, rngs::ThreadRng};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Score(i16);

impl Score {
    fn new() -> Score {
        Score(i16::MIN)
    }
}

#[derive(Debug)]
struct Search{
    board: Board
}

impl Search {
    fn new(board: Board) -> Search {
        Search { board }
    }
}

impl Search {

    pub fn negamax(self, depth: u8) -> Option<Move> {
        let mut rng = rand::rng();
        let moves = self.board.get_moves();
        let mut best_move: Option<Move> = None;
        let mut best_score: Score = Score::new();
        for mv in moves {
            let mut board = self.board.clone();
            board.do_move(mv);
            let score = self.negamax_recursion(board, depth, &mut rng);
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }
        best_move
    }

    fn negamax_recursion(&self, board: Board, depth: u8, rng: &mut ThreadRng) -> Score {
        if depth == 0 {
            return random_eval(board, rng);
        }
        let moves = board.get_moves();
        let mut best_score = Score::new();
        for mv in moves {
            let mut new_board = board.clone();
            new_board.do_move(mv);
            let score = random_eval(new_board, rng);
            best_score = max(best_score, score);
        }
        best_score
    }
}

fn random_eval(_board: Board, rng: &mut ThreadRng) -> Score {
    Score(rng.random_range(-10_000..=10_000))
}

#[test]
fn test_negamax() {
    let search = Search::new(Board::startpos());
    let best_mv = search.negamax(3);
    println!("best_move is {:?}", best_mv);
    assert!(false)
}
