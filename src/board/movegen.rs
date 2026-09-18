use crate::{
    board::{
        board::Board,
        r#move::{Move, MoveList},
    }, common::{
        direction::{North, NorthEast, NorthWest, South, SouthEast, SouthWest}, magics::magic_table, masks::knight_mask, piece::Piece, rank::Rank,
    },
};

impl Board {
    pub fn get_moves(&self) -> MoveList {
        let mut move_list = MoveList::default();
        /*
        pawns
        */
        self.pawns(&mut move_list);
        self.knights(&mut move_list);
        move_list
    }

    #[inline]
    fn pawns(&self, move_list: &mut MoveList) {
        let us_pawns = self.pieces(Piece::Pawn) & self.colours(self.stm());
        //forward 1
        let pawns_forward_1 =
            us_pawns.relative_shift::<North>(self.stm(), 1) & !self.all_pieces();
        for dst in pawns_forward_1.iter() {
            let Some(src) = dst.relative_shift::<South>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst));
        }

        //forward 2
        let pawns_forward_2 = pawns_forward_1.relative_shift::<North>(self.stm(), 1)
            & !self.all_pieces()
            & Rank::Four.relative_to(self.stm()).to_bb();
        for dst in pawns_forward_2.iter() {
            let Some(src) = dst.relative_shift::<South>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst));
        }

        //attack left
        let pawns_attack_left =
            us_pawns.relative_shift::<NorthWest>(self.stm(), 1) & self.colours(!self.stm());
        for dst in pawns_attack_left.iter() {
            let Some(src) = dst.relative_shift::<SouthEast>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst));
        }

        //attack right
        let pawns_attack_right =
            us_pawns.relative_shift::<NorthEast>(self.stm(), 1) & self.colours(!self.stm());
        for dst in pawns_attack_right.iter() {
            let Some(src) = dst.relative_shift::<SouthWest>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst));
        }
    }

    #[inline]
    fn knights(&self, move_list: &mut MoveList) {
        let src_bb = self.pieces(Piece::Knight) & self.colours(self.stm());
        for src in src_bb.iter() {
            let valid_moves = knight_mask(src) & !self.colours(self.stm());
            for dst in valid_moves.iter() {
                move_list.add(Move::new(src, dst));
            }
        }
    }

    // #[inline]
    // fn rooks(&self, move_list: &mut MoveList) {
    //     let table = magic_table();
    // }
}

#[test]
fn default_board() {
    let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    board.display();
    println!("moves count: {}", board.get_moves().0.len());
    // assert!(false);
}

#[test]
fn pawn_attack() {
    let board = Board::parse_fen("4k3/8/8/2p1p1p1/3P1P2/8/8/4K3 w KQkq - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    board.display();
    println!("moves count: {}", board.get_moves().0.len());
    // assert!(false);
}

#[test]
fn knight_attack() {
    let board = Board::parse_fen("k7/8/8/3p1P2/8/4N3/8/K7 w - - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    board.display();
    println!("moves count: {}", board.get_moves().0.len());
    // assert!(false);
}