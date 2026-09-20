use arrayvec::ArrayVec;

use crate::{
    board::{
        board::Board, r#move::{Move, MoveFlag},
    }, common::{
        bitboard::Bitboard, colour::Colour, direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West}, file::File::F, magics::{self, magic_table}, masks::{between_mask, king_mask, knight_mask}, piece::{self, Piece::{self, Rook}}, rank::Rank, square::Square,
    },
};

const MAX_MOVES: usize = 218;

#[derive(Debug, Clone, Default)]
pub struct MoveList(pub ArrayVec<Move, MAX_MOVES>);

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = arrayvec::IntoIter<Move, MAX_MOVES>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Board {

    pub fn get_moves(&self) -> MoveList {
        let banned = self.calc_banned();
        let (
            checkmask, 
            orth_pin_mask, 
            diag_pin_mask
        ) = self.check_and_pin_masks();

        let mut move_list = MoveList::default();
        self.pawns(&mut move_list, checkmask);
        self.knights(&mut move_list, checkmask);
        self.sliders(&mut move_list, checkmask, 
            orth_pin_mask,
             diag_pin_mask
        );
        self.king(&mut move_list, banned);
        self.castling(&mut move_list, banned);
        move_list
    }

    #[inline]
    fn calc_banned(&self) -> Bitboard {
        let mut banned = Bitboard::EMPTY;
        let magics = magic_table();
        let stm_pieces = self.all_pieces() 
            & !(self.pieces(Piece::King) & self.us_pieces());
        //remove the king since we want to catch if its in check when it moves
        // TODO: Consider if it should be calculated and saved on make_move

        let pawns = 
            self.pieces(Piece::Pawn)
            & self.them_pieces();

        banned |= pawns.relative_shift::<NorthEast>(!self.stm, 1);
        banned |= pawns.relative_shift::<NorthWest>(!self.stm, 1);

        for sqr in (self.pieces(Piece::Knight) & self.them_pieces()).iter() {
            banned |= knight_mask(sqr);
        }

        for sqr in (self.pieces(Piece::Bishop) & self.them_pieces()).iter() {
            banned |= magics.get_diag(stm_pieces, sqr);
        }

        for sqr in (self.pieces(Piece::Rook) & self.them_pieces()).iter() {
            banned |= magics.get_orth(stm_pieces, sqr);
        }

        for sqr in (self.pieces(Piece::Queen) & self.them_pieces()).iter() {
            banned |= magics.get_diag(stm_pieces, sqr);
            banned |= magics.get_orth(stm_pieces, sqr);
        }

        for sqr in (self.pieces(Piece::King) & self.them_pieces()).iter() {
            banned |= king_mask(sqr);
        }

        banned
    }

    #[inline]
    fn check_and_pin_masks(&self) -> (Bitboard, Bitboard, Bitboard) {
        let magics = magic_table();

        let us_king: Square = 
            (self.pieces(Piece::King) & 
            self.us_pieces())
            .iter()
            .next()
            .expect("there should be exactly 1 king");

        let orth = (self.pieces(Piece::Rook) | self.pieces(Piece::Queen)) 
            & self.them_pieces();
        let diag = (self.pieces(Piece::Bishop) | self.pieces(Piece::Queen)) 
            & self.them_pieces();

        //FULL since we want double checks to 'bubble up'
        let mut check_mask = Bitboard::FULL; 
        let mut orth_pin_mask = Bitboard::EMPTY;
        let mut diag_pin_mask = Bitboard::EMPTY;

        let orth_candidates = magics
            .get_orth(self.them_pieces(), us_king) & orth;

        let diag_candidates = magics
            .get_diag(self.them_pieces(), us_king) & diag;

        for sqr in orth_candidates.iter() {
            let ray = between_mask(us_king, sqr);
            match (ray & self.all_pieces()).count() {
                1 => check_mask &= ray, // in check
                2 => orth_pin_mask |= ray, //pin
                _ => {}
            }
        }

        for sqr in diag_candidates.iter() {
            let ray = between_mask(us_king, sqr);
            match (ray & self.all_pieces()).count() {
                1 => check_mask &= ray, // in check
                2 => diag_pin_mask |= ray, //pin
                _ => {}
            }
        }

        (check_mask, orth_pin_mask, diag_pin_mask)
    }

    #[inline]
    fn pawns(&self, move_list: &mut MoveList, checkmask: Bitboard) {

        let promotions = Rank::Eight.relative_to(self.stm).to_bb();
        let us_pawns = 
            self.pieces(Piece::Pawn) 
            & self.us_pieces();

        //forward 1 (discounting promotions)
        let pawns_forward_1 = 
            us_pawns.relative_shift::<North>(self.stm, 1) 
            & !self.all_pieces();

        for dst in (pawns_forward_1 & !promotions & checkmask).iter() {
            let Some(src) = dst.relative_shift::<South>(self.stm, 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
        }

        //forward 1 promotions 
        for dst in (pawns_forward_1 & promotions & checkmask).iter() {
            let Some(src) = dst.relative_shift::<South>(self.stm, 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::PromotionQueen));
            move_list.add(Move::new(src, dst, MoveFlag::PromotionRook));
            move_list.add(Move::new(src, dst, MoveFlag::PromotionBishop));
            move_list.add(Move::new(src, dst, MoveFlag::PromotionKnight));
        }

        //forward 2
        let pawns_forward_2 = pawns_forward_1.relative_shift::<North>(self.stm, 1)
            & !self.all_pieces()
            & Rank::Four.relative_to(self.stm).to_bb();

        for dst in (pawns_forward_2 & checkmask).iter() {
            let Some(src) = dst.relative_shift::<South>(self.stm, 2) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::PawnDouble));
        }

        //attack left
        let pawns_attack_left =
            us_pawns.relative_shift::<NorthWest>(self.stm, 1)
            & self.them_pieces();

        for dst in (pawns_attack_left & !promotions & checkmask).iter() {
            let Some(src) = dst.relative_shift::<SouthEast>(self.stm, 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::Capture));
        }

        //attack left promotions
        for dst in (pawns_attack_left & promotions & checkmask).iter() {
            let Some(src) = dst.relative_shift::<SouthEast>(self.stm, 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionQueen));
            move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionRook));
            move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionBishop));
            move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionKnight));
        }

        //attack right
        let pawns_attack_right =
            us_pawns.relative_shift::<NorthEast>(self.stm, 1)
            & self.them_pieces();

        for dst in (pawns_attack_right & !promotions & checkmask).iter() {
            let Some(src) = dst.relative_shift::<SouthWest>(self.stm, 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::Capture));
        }

        //attack right promotions
        for dst in (pawns_attack_right & promotions & checkmask).iter() {
            let Some(src) = dst.relative_shift::<SouthWest>(self.stm, 1) else {
                debug_assert!(false, "should be unreachable");
                continue;
            };
            move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionQueen));
            move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionRook));
            move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionBishop));
            move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionKnight));
        }

        //en passant
        if let Some(ep_sqr) = self.en_passant {
            let east = ep_sqr.shift::<East>(1).map_or_default(|s|{s.to_bb()});
            let west = ep_sqr.shift::<West>(1).map_or_default(|s|{s.to_bb()});
            let en_passant = us_pawns & (east | west) & checkmask;

            for src in en_passant.iter() {
                let relevant_rank = Rank::Five.relative_to(self.stm);
                let us_king = (
                    self.pieces(Piece::King) & 
                    self.us_pieces()
                ).iter().next().expect("should be exactly 1 king");

                //TODO: un-grossify this slop
                //we mostly avoid doing the legality check by only doing it when were on rank 5
                match us_king.rank() == relevant_rank {
                    false => {
                        let dst = ep_sqr
                            .relative_shift::<North>(self.stm, 1)
                            .expect("en passant dest square must exist");
                        move_list.add(Move::new(src, dst, MoveFlag::EnPassant));
                    },

                    true => {
                        'legality_check: {
                            let slider_pieces = (
                                self.pieces(Piece::Bishop) |
                                self.pieces(Piece::Queen) |
                                self.pieces(Piece::Rook) )
                                & relevant_rank.to_bb()
                                & self.them_pieces();

                            let relevant_board = 
                                self.all_pieces() ^
                                ep_sqr.to_bb() ^
                                src.to_bb();

                            let magics = magic_table();
                            for sqr in slider_pieces.iter() {
                                if magics.get_orth(relevant_board, sqr) 
                                   & us_king.to_bb() != Bitboard::EMPTY {
                                    break 'legality_check;
                                }
                            }

                            let dst = ep_sqr
                                .relative_shift::<North>(self.stm, 1)
                                .expect("en passant dest square must exist");
                            move_list.add(Move::new(src, dst, MoveFlag::EnPassant));
                    }
                }
                }
            }
        }
    }

    #[inline]
    fn knights(&self, move_list: &mut MoveList, checkmask: Bitboard) {
        let src_bb = self.pieces(Piece::Knight) & self.us_pieces();
        for src in src_bb.iter() {

            let valid_moves = 
                knight_mask(src) & 
                !self.us_pieces();

            for dst in (valid_moves & self.them_pieces() & checkmask).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }

            for dst in (valid_moves & !self.them_pieces() & checkmask).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
            }
        }
    }

    #[inline]
    fn sliders(
        &self, 
        move_list: &mut MoveList, 
        checkmask: Bitboard,
        orth_pinmask: Bitboard,
        diag_pinmask: Bitboard,
    ) {

        let table = magic_table();

        /*
        Rooks
        */
        let us_rooks = self.pieces(Piece::Rook) & self.us_pieces();
        for src in us_rooks.iter() {

            let mut rook_moves = table
                .get_orth(self.all_pieces(), src)
                & checkmask
                & !self.us_pieces();

            if orth_pinmask & src.to_bb() != Bitboard::EMPTY {
                rook_moves &= orth_pinmask;
            }

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
        let us_bishops = self.pieces(Piece::Bishop) & self.us_pieces();
        for src in us_bishops.iter() {

            let mut bishop_moves = table
                .get_diag(self.all_pieces(), src) &
                checkmask &
                !self.us_pieces();

            if diag_pinmask & src.to_bb() != Bitboard::EMPTY {
                bishop_moves &= orth_pinmask;
            }

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
        let us_queens = self.pieces(Piece::Queen) & self.us_pieces();
        for src in us_queens.iter() {

            let mut queen_moves = (
                table.get_orth(self.all_pieces(), src) |
                table.get_diag(self.all_pieces(), src)) &
                checkmask &
                !self.us_pieces();

            if diag_pinmask & src.to_bb() != Bitboard::EMPTY {
                queen_moves &= orth_pinmask;
            }

            if orth_pinmask & src.to_bb() != Bitboard::EMPTY {
                queen_moves &= orth_pinmask;
            }

            for dst in (queen_moves & self.them_pieces() & checkmask).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }

            for dst in (queen_moves & !self.them_pieces() & checkmask).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
            }
        }
    }

    #[inline]
    fn king(&self, move_list: &mut MoveList, banned: Bitboard) {

        let us_king = 
            self.pieces(Piece::King) & 
            self.us_pieces();

        debug_assert!(us_king.piece_count() == 1);
        // SAFETY: Bitboard is u64 hence trailing_zeros max is 64
        let src = Square::unchecked_index(us_king.0.trailing_zeros() as u8);

        let king_moves = king_mask(src) & 
            !self.us_pieces()
            & !banned;

        for dst in (king_moves & self.them_pieces()).iter() {
            move_list.add(Move::new(src, dst, MoveFlag::Capture));
        }

        for dst in (king_moves & !self.them_pieces()).iter() {
            move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
        }
    }

    #[inline]
    fn castling(&self, move_list: &mut MoveList, banned: Bitboard) {
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
            if (ks_threatened & banned.0) == 0
            {
                let src = Square::E1.relative(self.stm);
                let dst = Square::G1.relative(self.stm);
                move_list.add(Move::new(src, dst, MoveFlag::CastleShort));
            }
        }

        if self.get_castling(self.stm, false) 
            && (qs_mask & self.all_pieces().0 == 0) 
        {
            if (qs_threatened & banned.0) == 0
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