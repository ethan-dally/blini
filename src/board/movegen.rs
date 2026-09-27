use crate::{
    board::board::Board,
    common::{
        bitboard::Bitboard,
        colour::Colour,
        direction::{
            East,
            North,
            NorthEast,
            NorthWest,
            South,
            SouthEast,
            SouthWest,
            West,
        },
        magics::magic_table,
        masks::{
            between_mask,
            king_mask,
            knight_mask,
        },
        r#move::{
            Move,
            MoveFlag,
            MoveList,
        },
        piece::Piece::{
            self,
        },
        rank::Rank,
        square::Square,
    },
};

impl Board {
    pub fn get_moves(&self) -> MoveList {
        let banned = self.calc_banned();
        let (checkmask, orth_pin_mask, diag_pin_mask) = self.check_and_pin_masks();

        let mut move_list = MoveList::default();
        self.pawns(&mut move_list, checkmask, orth_pin_mask, diag_pin_mask);
        self.pinned_pawns(&mut move_list, checkmask, orth_pin_mask, diag_pin_mask);
        self.knights(&mut move_list, checkmask, orth_pin_mask, diag_pin_mask);
        self.sliders(&mut move_list, checkmask, orth_pin_mask, diag_pin_mask);

        self.king(&mut move_list, banned);
        self.castling(&mut move_list, banned);
        move_list
    }

    #[inline]
    fn calc_banned(&self) -> Bitboard {
        let mut banned = Bitboard::EMPTY;
        let magics = magic_table();
        let stm_pieces = self.all_pieces() & !(self.pieces(Piece::King) & self.us_pieces());
        // remove the king since we want to catch if its in check when it moves
        // TODO: Consider if it should be calculated and
        // saved earlier into the board on make_move for eval

        let pawns = self.pieces(Piece::Pawn) & self.them_pieces();

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

        let us_king: Square = (self.pieces(Piece::King) & self.us_pieces())
            .iter()
            .next()
            .expect("there should be exactly 1 king");

        let them_orth = (self.pieces(Piece::Rook) | self.pieces(Piece::Queen)) & self.them_pieces();
        let them_diag =
            (self.pieces(Piece::Bishop) | self.pieces(Piece::Queen)) & self.them_pieces();

        //FULL since we want double checks to 'bubble up'
        let mut check_mask = Bitboard::FULL;
        let mut orth_pin_mask = Bitboard::EMPTY;
        let mut diag_pin_mask = Bitboard::EMPTY;

        let orth_candidates = magics.get_orth(self.them_pieces(), us_king) & them_orth;

        let diag_candidates = magics.get_diag(self.them_pieces(), us_king) & them_diag;

        for sqr in orth_candidates.iter() {
            let ray = between_mask(us_king, sqr);
            match (ray & self.all_pieces()).count() {
                1 => check_mask &= ray,    // in check
                2 => orth_pin_mask |= ray, //pin
                _ => {}
            }
        }

        for sqr in diag_candidates.iter() {
            let ray = between_mask(us_king, sqr);
            match (ray & self.all_pieces()).count() {
                1 => check_mask &= ray,    // in check
                2 => diag_pin_mask |= ray, //pin
                _ => {}
            }
        }

        let checkmask_knights =
            knight_mask(us_king) & self.pieces(Piece::Knight) & self.them_pieces();

        let checkmask_pawns = (us_king
            .relative_shift::<NorthEast>(self.stm, 1)
            .map_or_default(|s| s.to_bb())
            | us_king
                .relative_shift::<NorthWest>(self.stm, 1)
                .map_or_default(|s| s.to_bb()))
            & self.pieces(Piece::Pawn)
            & self.them_pieces();

        for sqr in (checkmask_knights | checkmask_pawns).iter() {
            check_mask &= sqr.to_bb();
        }

        (check_mask, orth_pin_mask, diag_pin_mask)
    }

    #[inline]
    fn pinned_pawns(
        &self,
        move_list: &mut MoveList,
        checkmask: Bitboard,
        orth_pin_mask: Bitboard,
        diag_pin_mask: Bitboard,
    ) {
        /*
        this is gross as it redoes an amount of pawn_moves already, but it should be
        faster since were not doing the pin checks for the vast majority of pawns

        if theres a check in play, you cant move your pinned pawn to prevent it
        since the best the pinned pawn can do is take the piece thats pinning
        which cant 'by meaning of a pin' be the piece thats doing the check
        */
        if checkmask != Bitboard::FULL {
            return;
        }

        let promotions = Rank::Eight.relative_to(self.stm).to_bb();

        let diag_pawns = self.pieces(Piece::Pawn) & self.us_pieces() & diag_pin_mask;

        let orth_pawns = self.pieces(Piece::Pawn) & self.us_pieces() & orth_pin_mask;

        //since diag pinned pawns cant move forward
        let pawns_forward_1 = orth_pawns.relative_shift::<North>(self.stm, 1) & !self.all_pieces();

        //forward 1 (discounting promotions)
        for dst in pawns_forward_1.iter() {
            //also impossible to be pinned and promote here
            let src = dst
                .relative_shift::<South>(self.stm, 1)
                .expect("unreachable");
            if orth_pin_mask.has(dst) {
                move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
            }
        }

        //forward 2
        let pawns_forward_2 = pawns_forward_1.relative_shift::<North>(self.stm, 1)
            & Rank::Four.relative_to(self.stm).to_bb()
            & !self.all_pieces();

        for dst in pawns_forward_2.iter() {
            let src = dst
                .relative_shift::<South>(self.stm, 2)
                .expect("unreachable");
            if orth_pin_mask.has(dst) {
                move_list.add(Move::new(src, dst, MoveFlag::PawnDouble));
            }
        }

        //attack
        //if orth pinned, then cant take diagonally
        let attack_left = diag_pawns.relative_shift::<NorthEast>(self.stm, 1) & self.them_pieces();

        let attack_right = diag_pawns.relative_shift::<NorthWest>(self.stm, 1) & self.them_pieces();

        for dst in (attack_left & !promotions).iter() {
            let src = dst
                .relative_shift::<SouthWest>(self.stm, 1)
                .expect("unreachable");
            if diag_pin_mask.has(dst) {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }
        }

        for dst in (attack_left & promotions).iter() {
            let src = dst
                .relative_shift::<SouthWest>(self.stm, 1)
                .expect("unreachable");
            if diag_pin_mask.has(dst) {
                move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionQueen));
                move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionRook));
                move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionBishop));
                move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionKnight));
            }
        }

        for dst in (attack_right & !promotions).iter() {
            let src = dst
                .relative_shift::<SouthEast>(self.stm, 1)
                .expect("unreachable");
            if diag_pin_mask.has(dst) {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }
        }

        for dst in (attack_right & promotions).iter() {
            let src = dst
                .relative_shift::<SouthEast>(self.stm, 1)
                .expect("unreachable");
            if diag_pin_mask.has(dst) {
                move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionQueen));
                move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionRook));
                move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionBishop));
                move_list.add(Move::new(src, dst, MoveFlag::CapturePromotionKnight));
            }
        }

        //en passant
        //no discovered pin check needed since we are already in a pin
        if let Some(ep_sqr) = self.en_passant {
            let east = ep_sqr.shift::<East>(1).map_or_default(|s| s.to_bb());
            let west = ep_sqr.shift::<West>(1).map_or_default(|s| s.to_bb());
            let en_passant = diag_pawns & (east | west);
            for src in en_passant.iter() {
                let dst = ep_sqr
                    .relative_shift::<North>(self.stm, 1)
                    .expect("en passant dest square must exist");
                if diag_pin_mask.has(dst) {
                    move_list.add(Move::new(src, dst, MoveFlag::EnPassant));
                }
            }
        }
    }

    #[inline]
    fn pawns(
        &self,
        move_list: &mut MoveList,
        checkmask: Bitboard,
        orth_pin_mask: Bitboard,
        diag_pin_mask: Bitboard,
    ) {
        let pinned = orth_pin_mask | diag_pin_mask;
        let promotions = Rank::Eight.relative_to(self.stm).to_bb();

        let us_pawns = self.pieces(Piece::Pawn) & self.us_pieces() & !pinned;

        let pawns_forward_1 = us_pawns.relative_shift::<North>(self.stm, 1) & !self.all_pieces();

        //forward 1 (discounting promotions)
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
            us_pawns.relative_shift::<NorthWest>(self.stm, 1) & self.them_pieces();

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
            us_pawns.relative_shift::<NorthEast>(self.stm, 1) & self.them_pieces();

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
            let east = ep_sqr.shift::<East>(1).map_or_default(|s| s.to_bb());
            let west = ep_sqr.shift::<West>(1).map_or_default(|s| s.to_bb());
            let en_passant = us_pawns & (east | west) & checkmask;

            for src in en_passant.iter() {
                let relevant_rank = Rank::Five.relative_to(self.stm);
                let us_king = (self.pieces(Piece::King) & self.us_pieces())
                    .iter()
                    .next()
                    .expect("should be exactly 1 king");

                //TODO: un-grossify this slop
                //we mostly avoid doing the legality check by only doing it when were on rank 5
                match us_king.rank() == relevant_rank {
                    false => {
                        let dst = ep_sqr
                            .relative_shift::<North>(self.stm, 1)
                            .expect("en passant dest square must exist");
                        move_list.add(Move::new(src, dst, MoveFlag::EnPassant));
                    }

                    true => 'legality_check: {
                        let slider_pieces = (self.pieces(Piece::Bishop)
                            | self.pieces(Piece::Queen)
                            | self.pieces(Piece::Rook))
                            & relevant_rank.to_bb()
                            & self.them_pieces();

                        let relevant_board = self.all_pieces() ^ ep_sqr.to_bb() ^ src.to_bb();

                        let magics = magic_table();
                        for sqr in slider_pieces.iter() {
                            if magics.get_orth(relevant_board, sqr) & us_king.to_bb()
                                != Bitboard::EMPTY
                            {
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

    #[inline]
    fn knights(
        &self,
        move_list: &mut MoveList,
        checkmask: Bitboard,
        orth_pin_mask: Bitboard,
        diag_pin_mask: Bitboard,
    ) {
        let pinned = orth_pin_mask | diag_pin_mask;
        let src_bb = self.pieces(Piece::Knight) & self.us_pieces() & !pinned;
        for src in src_bb.iter() {
            let valid_moves = knight_mask(src) & !self.us_pieces();

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
        orth_pin_mask: Bitboard,
        diag_pin_mask: Bitboard,
    ) {
        let table = magic_table();

        /*
        Rooks
        */
        let us_rooks = self.pieces(Piece::Rook) & self.us_pieces() & !diag_pin_mask;

        for src in us_rooks.iter() {
            let mut rook_moves =
                table.get_orth(self.all_pieces(), src) & checkmask & !self.us_pieces();

            if orth_pin_mask & src.to_bb() != Bitboard::EMPTY {
                rook_moves &= orth_pin_mask;
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
        let us_bishops = self.pieces(Piece::Bishop) & self.us_pieces() & !orth_pin_mask;

        for src in us_bishops.iter() {
            let mut bishop_moves =
                table.get_diag(self.all_pieces(), src) & checkmask & !self.us_pieces();

            if diag_pin_mask & src.to_bb() != Bitboard::EMPTY {
                bishop_moves &= diag_pin_mask;
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
            let mut queen_moves = (table.get_orth(self.all_pieces(), src)
                | table.get_diag(self.all_pieces(), src))
                & checkmask
                & !self.us_pieces();

            if diag_pin_mask & src.to_bb() != Bitboard::EMPTY {
                queen_moves &= diag_pin_mask;
            }

            if orth_pin_mask & src.to_bb() != Bitboard::EMPTY {
                queen_moves &= orth_pin_mask;
            }

            for dst in (queen_moves & self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::Capture));
            }

            for dst in (queen_moves & !self.them_pieces()).iter() {
                move_list.add(Move::new(src, dst, MoveFlag::NonCapture));
            }
        }
    }

    #[inline]
    fn king(&self, move_list: &mut MoveList, banned: Bitboard) {
        let us_king = self.pieces(Piece::King) & self.us_pieces();

        debug_assert!(us_king.piece_count() == 1);

        let src = Square::from_trailing_zeros(us_king);

        let king_moves = king_mask(src) & !self.us_pieces() & !banned;

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
            Colour::Black => (0x6000000000000000u64, 0x0E00000000000000u64),
        };
        let (ks_threatened, qs_threatened) = match self.stm {
            Colour::White => (0x0000000000000070u64, 0x000000000000001Cu64),
            Colour::Black => (0x7000000000000000u64, 0x1C00000000000000u64),
        };

        if self.get_castling(self.stm, true)
            && (ks_mask & self.all_pieces().0 == 0)
            && (ks_threatened & banned.0) == 0
        {
            let src = Square::E1.relative(self.stm);
            let dst = Square::G1.relative(self.stm);
            move_list.add(Move::new(src, dst, MoveFlag::CastleShort));
        }

        if self.get_castling(self.stm, false)
            && (qs_mask & self.all_pieces().0 == 0)
            && (qs_threatened & banned.0) == 0
        {
            let src = Square::E1.relative(self.stm);
            let dst = Square::C1.relative(self.stm);
            move_list.add(Move::new(src, dst, MoveFlag::CastleLong));
        }
    }
}
