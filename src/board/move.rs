use crate::{
    board::board::Board,
    common::{
        colour::Colour,
        r#move::{Move, MoveFlag},
        piece::Piece,
        rank::Rank,
        square::Square,
    },
};

impl Board {
    #[inline]
    fn remove_castling_rights(&mut self, mv: Move, moved_piece: Piece) {
        if moved_piece == Piece::King {
            self.set_castling(self.stm, true, false);
            self.set_castling(self.stm, false, false);
        } else if moved_piece == Piece::Rook {
            if mv.src() == Square::A1.relative(self.stm) {
                self.set_castling(self.stm, false, false);
            }

            if mv.src() == Square::H1.relative(self.stm) {
                self.set_castling(self.stm, true, false);
            }
        }

        if mv.dst() == Square::A8.relative(self.stm) {
            self.set_castling(!self.stm, false, false);
        }

        if mv.dst() == Square::H8.relative(self.stm) {
            self.set_castling(!self.stm, true, false);
        }
    }

    pub fn do_move(&mut self, mv: Move) {
        self.hmc += 1;
        self.fmn += u16::from(self.stm == Colour::Black);
        let moved_piece = self.remove_piece(mv.src());
        self.set_square(mv.dst(), self.stm(), moved_piece);
        self.set_en_passant(None);
        self.remove_castling_rights(mv, moved_piece);

        if moved_piece == Piece::Pawn {
            self.hmc = 0;
        }

        match mv.flag() {
            MoveFlag::NonCapture => {}
            MoveFlag::Capture => {
                self.hmc = 0;
            }
            MoveFlag::PawnDouble => {
                self.set_en_passant(Some(mv.dst()));
            }
            MoveFlag::CastleLong => {
                let rook_src = Square::A1.relative(self.stm);
                let rook_dst = Square::D1.relative(self.stm);
                self.remove_piece(rook_src);
                self.set_square(rook_dst, self.stm, Piece::Rook);
            }
            MoveFlag::CastleShort => {
                let rook_src = Square::H1.relative(self.stm);
                let rook_dst = Square::F1.relative(self.stm);
                self.remove_piece(rook_src);
                self.set_square(rook_dst, self.stm, Piece::Rook);
            }
            MoveFlag::EnPassant => {
                // self.hmc = 0;
                let file = mv.dst().file();
                let rank = Rank::Five.relative_to(self.stm);
                self.remove_piece(Square::new(rank, file));
            }
            MoveFlag::CapturePromotionKnight
            | MoveFlag::CapturePromotionBishop
            | MoveFlag::CapturePromotionQueen
            | MoveFlag::CapturePromotionRook => {
                self.hmc = 0;
                self.set_square(mv.dst(), self.stm, mv.flag().piece());
            }
            MoveFlag::PromotionKnight
            | MoveFlag::PromotionBishop
            | MoveFlag::PromotionQueen
            | MoveFlag::PromotionRook => {
                self.set_square(mv.dst(), self.stm, mv.flag().piece());
            }
        }
        self.stm = !self.stm;
    }

    #[inline]
    pub fn apply_uci_moves(&mut self, uci_mv: Vec<&str>) -> Result<(), String> {
        for mv in uci_mv {
            let Some((raw_src, raw_dst)) = mv.split_at_checked(2) else {
                return Err(format!("couldnt split move '{mv}'"));
            };
            let Some(src) = Square::parse(raw_src) else {
                return Err(format!("couldnt parse src '{raw_src}' in '{mv}'"));
            };
            let Some(dst) = Square::parse(raw_dst) else {
                return Err(format!("couldnt parse src '{raw_dst}' in '{mv}'"));
            };
            let Some(verified_move) = self.get_moves().find(src, dst) else {
                return Err(format!("move '{mv}' is not a legal move"));
            };
            self.do_move(verified_move);
        }
        Ok(())
    }
}
