use crate::{board::board::Board, common::r#move::Move};

#[derive(Debug, Clone)]
pub struct Position {
    board: Board,
    prev: Vec<u64>,
}

impl Position {
    #[inline]
    pub fn new(board: Board, depth_hint: usize) -> Position {
        Position {
            board,
            prev: Vec::with_capacity(depth_hint),
        }
    }

    #[inline]
    pub fn do_move(&mut self, mv: Move) {
        self.prev.push(self.board.zobrist());
        self.board.do_move(mv);
    }

    #[inline]
    pub fn board(&self) -> &Board {
        &self.board
    }

    #[inline]
    pub fn is_draw(&self) -> bool {
        if self.board.is_draw() {
            return true;
        }

        if self
            .prev
            .iter()
            .filter(|z| *z == &self.board.zobrist())
            .count()
            >= 2
        {
            return true;
        }

        false
    }
}
