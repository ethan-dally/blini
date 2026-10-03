use crate::{
    board::{board::Board, zobrist::Zobrist}, common::{
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

        let captured_piece = self.try_remove_piece(mv.dst());
        let moved_piece = self.remove_piece(mv.src());

        self.set_square(mv.dst(), self.stm(), moved_piece);

        //remove old en passant
        if let Some(sqr) = self.set_en_passant(None) {
            self.zobrist ^= Zobrist::EnPassant(sqr.file()).get();
        }

        // TODO: make this double xor conditional on castling rights changing
        self.zobrist ^= Zobrist::Castling(self.castling).get();
        self.remove_castling_rights(mv, moved_piece);
        self.zobrist ^= Zobrist::Castling(self.castling).get();

        self.zobrist ^= Zobrist::Move.get();
        self.zobrist ^= Zobrist::Piece { piece: moved_piece, sqr: mv.src(), colour: self.stm()}.get();
        self.zobrist ^= Zobrist::Piece { piece: moved_piece, sqr: mv.dst(), colour: self.stm()}.get();


        if moved_piece == Piece::Pawn {
            self.hmc = 0;
        }

        match mv.flag() {
            MoveFlag::NonCapture => {
            }

            MoveFlag::Capture => {
                let captured_piece = captured_piece.expect("move with capture flag didnt capture");
                self.zobrist ^= Zobrist::Piece { piece: captured_piece, sqr: mv.dst(), colour: !self.stm() }.get();
                self.hmc = 0;
            }

            MoveFlag::PawnDouble => {
                self.set_en_passant(Some(mv.dst()));
                self.zobrist ^= Zobrist::EnPassant(mv.dst().file()).get();
            }

            MoveFlag::CastleLong => {
                let rook_src = Square::A1.relative(self.stm);
                let rook_dst = Square::D1.relative(self.stm);
                self.remove_piece(rook_src);
                self.set_square(rook_dst, self.stm, Piece::Rook);
                self.zobrist ^= Zobrist::Piece { piece: Piece::Rook, sqr: rook_src, colour: self.stm()}.get();
                self.zobrist ^= Zobrist::Piece { piece: Piece::Rook, sqr: rook_dst, colour: self.stm()}.get();
            }

            MoveFlag::CastleShort => {
                let rook_src = Square::H1.relative(self.stm);
                let rook_dst = Square::F1.relative(self.stm);
                self.remove_piece(rook_src);
                self.set_square(rook_dst, self.stm, Piece::Rook);
                self.zobrist ^= Zobrist::Piece { piece: Piece::Rook, sqr: rook_src, colour: self.stm()}.get();
                self.zobrist ^= Zobrist::Piece { piece: Piece::Rook, sqr: rook_dst, colour: self.stm()}.get();
            }

            MoveFlag::EnPassant => {
                // self.hmc = 0;
                let sqr = Square::new(
                    Rank::Five.relative_to(self.stm),
                    mv.dst().file()
                );
                //rm piece
                self.remove_piece(sqr);
                self.zobrist ^= Zobrist::Piece { piece: Piece::Pawn, sqr, colour: !self.stm()}.get();
            }

            MoveFlag::CapturePromotionKnight
            | MoveFlag::CapturePromotionBishop
            | MoveFlag::CapturePromotionQueen
            | MoveFlag::CapturePromotionRook => {
                self.hmc = 0;
                self.set_square(mv.dst(), self.stm, mv.flag().piece());
                //for capture
                let captured_piece = captured_piece.expect("move with capture flag didnt capture");
                self.zobrist ^= Zobrist::Piece { piece: captured_piece, sqr: mv.dst(), colour: !self.stm() }.get();
                //for promotion (undoing what we eagerly did earlier) its rare so probs worth to do this way
                //rather than check each time
                self.zobrist ^= Zobrist::Piece { piece: moved_piece, sqr: mv.dst(), colour: self.stm()}.get();
                self.zobrist ^= Zobrist::Piece { piece: mv.flag().piece(), sqr: mv.dst(), colour: self.stm()}.get();
            }

            MoveFlag::PromotionKnight
            | MoveFlag::PromotionBishop
            | MoveFlag::PromotionQueen
            | MoveFlag::PromotionRook => {
                self.set_square(mv.dst(), self.stm, mv.flag().piece());
                //this undoes what we eagerly did earlier its rare so probs worth to do this way
                //rather than check each time
                self.zobrist ^= Zobrist::Piece { piece: moved_piece, sqr: mv.dst(), colour: self.stm()}.get();
                self.zobrist ^= Zobrist::Piece { piece: mv.flag().piece(), sqr: mv.dst(), colour: self.stm()}.get();
            }

        }
        self.stm = !self.stm;
    }

    #[inline]
    pub fn apply_uci_moves(mut self, uci_mv: Vec<&str>) -> Result<Self, String> {
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
        Ok(self)
    }
}
