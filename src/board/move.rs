use crate::common::square::Square;
use arrayvec::ArrayVec;

const MAX_MOVES: usize = 218;

#[derive(Debug)]
pub struct Move {
    src: Square,
    dst: Square,
}

impl Move {
    #[inline]
    pub fn new(src: Square, dst: Square) -> Move {
        Move { src, dst }
    }
}

#[derive(Debug, Default)]
pub struct MoveList(pub ArrayVec<Move, MAX_MOVES>);

impl MoveList {
    pub fn add(&mut self, mv: Move) {
        self.0.push(mv);
    }
}
