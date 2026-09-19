use arrayvec::ArrayVec;

use crate::{
    board::{
        board::Board, r#move::{Move, MoveFlag},
    }, common::{
        bitboard::Bitboard, colour::Colour, direction::{North, NorthEast, NorthWest, South, SouthEast, SouthWest}, file::File::F, magics::magic_table, masks::{king_mask, knight_mask}, piece::Piece, rank::Rank, square::Square,
    },
};

const MAX_MOVES: usize = 218;

#[derive(Debug, Default)]
pub struct MoveList(pub ArrayVec<Move, MAX_MOVES>);

impl MoveList {
    pub fn add(&mut self, mv: Move) {
        self.0.push(mv);
    }
}

impl Board {

    pub fn get_moves(&self) -> MoveList {
        let mut move_list = MoveList::default();
        /*
        pawns
        */
        self.pawns(&mut move_list);
        self.knights(&mut move_list);
        self.sliders(&mut move_list);
        self.king(&mut move_list);
        self.castling(&mut move_list);
        move_list
    }

    #[inline]
    fn pawns(&self, move_list: &mut MoveList) {
        let us_pawns = 
            self.pieces(Piece::Pawn) 
            & self.us_pieces();

        //forward 1
        let pawns_forward_1 = 
            us_pawns.relative_shift::<North>(self.stm(), 1) 
            & !self.all_pieces();
        for dst in pawns_forward_1.iter() {
            let Some(src) = dst.relative_shift::<South>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
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
            move_list.add(Move::new(src, dst, MoveFlag::PawnDouble));
        }

        //attack left
        let pawns_attack_left =
            us_pawns.relative_shift::<NorthWest>(self.stm(), 1)
            & self.colours(!self.stm());

        for dst in pawns_attack_left.iter() {
            let Some(src) = dst.relative_shift::<SouthEast>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::Capture));
        }

        //attack right
        let pawns_attack_right =
            us_pawns.relative_shift::<NorthEast>(self.stm(), 1)
            & self.them_pieces();

        for dst in pawns_attack_right.iter() {
            let Some(src) = dst.relative_shift::<SouthWest>(self.stm(), 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::Capture));
        }
    }

    #[inline]
    fn knights(&self, move_list: &mut MoveList) {
        let src_bb = self.pieces(Piece::Knight) & self.us_pieces();
        for src in src_bb.iter() {

            let valid_moves = 
                knight_mask(src) & 
                !self.us_pieces();

            for dst in (valid_moves & self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }

            for dst in (valid_moves & !self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
            }
        }
    }

    #[inline]
    fn sliders(&self, move_list: &mut MoveList) {

        let table = magic_table();

        /*
        Rooks
        */
        let us_rooks = self.pieces(Piece::Rook) & self.us_pieces();
        for src in us_rooks.iter() {

            let rook_moves = table
                .get_orth(self.all_pieces(), src)
                & !self.us_pieces();

            for dst in (rook_moves & self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }

            for dst in (rook_moves & !self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
            }
        }

        /*
        Bishops
        */
        let us_bishops = self.pieces(Piece::Bishop) & self.colours(self.stm());
        for src in us_bishops.iter() {

            let bishop_moves = table
                .get_diag(self.all_pieces(), src) &
                !self.colours(self.stm());

            for dst in (bishop_moves & self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }

            for dst in (bishop_moves & !self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
            }
        }

        /*
        Queens
        */
        let us_queens = self.pieces(Piece::Queen) & self.colours(self.stm());
        for src in us_queens.iter() {

            let queen_moves = (
                table.get_orth(self.all_pieces(), src) |
                table.get_diag(self.all_pieces(), src)) &
                !self.colours(self.stm());

            for dst in (queen_moves & self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }

            for dst in (queen_moves & !self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
            }
        }
    }

    #[inline]
    fn king(&self, move_list: &mut MoveList) {
        let us_king = 
            self.pieces(Piece::King) & 
            self.us_pieces();

        debug_assert!(us_king.piece_count() == 1);
        // SAFETY: Bitboard is u64 hence trailing_zeros max is 64
        let src = Square::unchecked_index(us_king.0.trailing_zeros() as u8);

        let king_moves = king_mask(src) & 
            !self.colours(self.stm());

        for dst in (king_moves & self.them_pieces()).iter() {
            move_list.add(Move::new(src, dst, MoveFlag::Capture));
        }

        for dst in (king_moves & !self.them_pieces()).iter() {
            move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
        }
    }

    #[inline]
    fn castling(&self, move_list: &mut MoveList) {
        let (ks_mask, qs_mask) = match self.stm {
            Colour::White => (0x0000000000000060u64, 0x000000000000000Eu64),
            Colour::Black => (0x6000000000000000u64, 0x0E00000000000000u64)
        };
        let (ks_threatened, qs_threatened) = match self.stm {
            Colour::White => (0x0000000000000070u64, 0x000000000000001Cu64),
            Colour::Black => (0x7000000000000000u64, 0x1C00000000000000u64)
        };

        if self.get_castling(self.stm, true) 
            && (ks_mask & self.all_pieces().0 == 0) 
        {
            if Bitboard(ks_threatened)
                .iter()
                .all(|sqr|{self.attackers(self.stm, sqr) == Bitboard::EMPTY}) 
            {
                let src = Square::E1.relative(self.stm);
                let dst = Square::G1.relative(self.stm);
                move_list.add(Move::new(src, dst, MoveFlag::CastleShort));
            }
        }

        if self.get_castling(self.stm, false) 
            && (qs_mask & self.all_pieces().0 == 0) 
        {
            if Bitboard(qs_threatened)
                .iter()
                .all(|sqr|{self.attackers(self.stm, sqr) == Bitboard::EMPTY}) 
            {
                let src = Square::E1.relative(self.stm);
                let dst = Square::C1.relative(self.stm);
                move_list.add(Move::new(src, dst, MoveFlag::CastleLong));
            }
        }
    }

}

#[test]
fn default_board() {
    let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    board.display();
    println!("moves count: {}", board.get_moves().0.len());
    assert!(false);
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

#[test]
fn rook_attack() {
    let board = Board::parse_fen("3k4/8/8/4P3/2p1R3/8/8/3K4 w - - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    board.display();
    println!("moves count: {}", board.get_moves().0.len());
    // assert!(false);
}

#[test]
fn bishop_attack() {
    let board = Board::parse_fen("3k4/8/2p5/5P2/4B3/8/8/3K4 w - - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    board.display();
    println!("moves count: {}", board.get_moves().0.len());
    // assert!(false);
}

#[test]
fn king_attack() {
    let board = Board::parse_fen("8/2k5/8/8/8/4r3/2PK4/8 w - - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    board.display();
    println!("moves count: {}", board.get_moves().0.len());
    // assert!(false);
}

#[test]
fn castling() {
    let board = Board::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
        .map_err(|s| panic!("invalid fen as {s}"))
        .unwrap();
    board.display();
    println!("moves count: {}", board.get_moves().0.len());
    assert_eq!(board.get_moves().0.len(), 48);
}