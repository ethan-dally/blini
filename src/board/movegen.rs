use crate::{
    board::{
        board::Board,
        r#move::{Move, MoveList},
    },
    common::{
        direction::{North, South},
        piece::Piece,
    },
};

impl Board {
    pub fn get_moves(&self) -> MoveList {
        let mut move_list = MoveList::default();
        /*
        pawns
        */
        self.pawns(&mut move_list);
        move_list
    }

    #[inline]
    fn pawns(&self, move_list: &mut MoveList) {
        let friendly_pawns = self.pieces(Piece::Pawn) & self.colours(self.stm());
        //forward 1
        let pawns_forward_1 =
            friendly_pawns.relative_shift::<North>(self.stm(), 1) & !self.all_pieces();
        for dst in pawns_forward_1.iter() {
            let Some(src) = dst.relative_shift::<South>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst));
        }

        //forward 2
        let pawns_forward_2 =
            pawns_forward_1.relative_shift::<North>(self.stm(), 1) & !self.all_pieces();
        for dst in pawns_forward_2.iter() {
            let Some(src) = dst.relative_shift::<South>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst));
        }

        //attack left
    }
}

#[test]
fn default_board() {
    let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    println!("moves count: {}", board.get_moves().0.len());
    // assert!(false);
}
