use color_eyre::eyre::{OptionExt, Result, eyre};

use crate::{
    board::{
        board::{Board, Castling},
        zobrist::Zobrist,
    },
    common::{
        colour::Colour,
        direction::{North, South},
        file::File,
        piece::Piece,
        rank::Rank,
        square::Square,
    },
};

impl Board {
    pub fn parse_fen(fen: &str) -> Result<Board> {
        let mut board = Board::empty();
        let mut parts = fen.split_whitespace();

        let pieces = parts
            .next()
            .ok_or_else(|| eyre!("missing piece placement"))?;

        let stm = parts.next().ok_or_else(|| eyre!("missing side to move"))?;

        let castling = parts
            .next()
            .ok_or_else(|| eyre!("missing castling rights"))?;

        let en_passant = parts
            .next()
            .ok_or_else(|| eyre!("missing en passant square"))?;

        let hmc = parts
            .next()
            .ok_or_else(|| eyre!("missing halfmove clock"))?;

        let fmc = parts
            .next()
            .ok_or_else(|| eyre!("missing fullmove counter"))?;

        if parts.next().is_some() {
            return Err(eyre!("has too many fields"));
        }

        /*
        pieces
        */
        let mut white_king = 0;
        let mut black_king = 0;
        for (rank, row) in pieces.split('/').enumerate() {
            let rank_index = u8::try_from(7 - rank).map_err(|_| eyre!("invalid rank count"))?;
            let Some(rank) = Rank::try_index(rank_index) else {
                return Err(eyre!("has invalid rank"));
            };

            let mut file_count: u8 = 0;

            for c in row.chars() {
                if c.is_ascii_digit() {
                    file_count +=
                        u8::try_from(c.to_digit(9).ok_or_else(|| eyre!("invalid digit '{c}'"))?)?;
                } else {
                    let file = File::try_index(file_count)
                        .ok_or_else(|| eyre!("invalid file index {file_count}"))?;
                    let sqr = Square::new(rank, file);
                    let piece = Piece::try_from(c).map_err(|_| eyre!("invalid piece char {c}"))?;
                    let colour = Colour::from(c.is_ascii_uppercase());
                    if piece == Piece::King {
                        white_king += u8::from(colour == Colour::White);
                        black_king += u8::from(colour == Colour::Black);
                    }
                    board.set_square(sqr, colour, piece);
                    file_count += 1;
                }
            }
        }

        if white_king != 1 || black_king != 1 {
            return Err(eyre!(
                "should have one king on each side w: {} b: {}",
                white_king,
                black_king,
            ));
        }

        /*
        stm
        */
        if stm.len() != 1 {
            return Err(eyre!("stm string len is too long"));
        }

        let stm: char = stm.chars().next().ok_or_eyre("unreachable")?;

        let stm = Colour::try_from_char(stm).ok_or_eyre("invalid stm char")?;

        board.set_stm(stm);

        /*
        castling
        */
        if castling.len() > 4 {
            return Err(eyre!("castling string should be of size 4"));
        }

        if castling != "-" {
            for c in castling.chars() {
                match c {
                    'K' => board.set_castling(Colour::White, true, true),
                    'Q' => board.set_castling(Colour::White, false, true),
                    'k' => board.set_castling(Colour::Black, true, true),
                    'q' => board.set_castling(Colour::Black, false, true),
                    _ => return Err(eyre!("has incorrect char: {} in castling", c)),
                }
            }
        }

        /*
        en passant
        */
        if en_passant != "-" {
            let en_passant = Square::try_from_str(en_passant)
                .ok_or(eyre!("invalid en_passant str {en_passant}"))?;
            let dst = en_passant
                .relative_shift::<South>(stm, 1)
                .ok_or_eyre("invalid en passant sqr")?;
            board.set_en_passant(Some(dst));
        }

        /*
        half move clock
        */
        let hmc = hmc
            .parse::<u8>()
            .or(Err(eyre!("FEN hmc '{hmc}' not a valid number")))?;
        if hmc > 50 {
            return Err(eyre!("FEN hmc should be below 50"));
        }
        board.set_hmc(hmc);

        /*
        full move number
        */
        let fmc = fmc
            .parse::<u16>()
            .or(Err(eyre!("fmc not a valid number")))?;
        board.set_fmn(fmc);

        /*
        zobrist
        */
        board.zobrist = Zobrist::calculate(&board);
        Ok(board)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in Rank::ALL.iter().rev() {
            let mut counter = 0;
            for file in File::ALL {
                if let Some((piece, colour)) = self.mailbox(Square::new(*rank, file)) {
                    if counter != 0 {
                        fen.push(char::from(b'0' + counter));
                    }
                    fen.push(piece.display(colour));
                    counter = 0;
                } else {
                    counter += 1;
                }
            }

            if counter != 0 {
                fen.push(char::from(b'0' + counter));
            }

            if *rank != Rank::One {
                fen.push('/');
            }
        }

        let ep = match self.en_passant() {
            Some(sqr) => {
                let fen_sqr = sqr
                    .relative_shift::<North>(self.stm(), 1)
                    .expect("move gen code must have gone horribly wrong for this to fail");
                &fen_sqr.to_string()
            }
            None => "-",
        };

        fen.push(' ');
        fen.push(char::from(self.stm()));
        fen.push(' ');
        fen.push_str(&String::from(self.castling));
        fen.push(' ');
        fen.push_str(ep);
        fen.push(' ');
        fen.push_str(&self.hmc().to_string());
        fen.push(' ');
        fen.push_str(&self.fmn().to_string());
        fen
    }
}

impl From<Castling> for String {
    fn from(castling: Castling) -> Self {
        if castling.value() == 0 {
            "-".to_string()
        } else {
            let mut out: String = "".to_string();
            if castling.get_castling(Colour::White, true) {
                out.push('K');
            }
            if castling.get_castling(Colour::White, false) {
                out.push('Q');
            }
            if castling.get_castling(Colour::Black, true) {
                out.push('k');
            }
            if castling.get_castling(Colour::Black, false) {
                out.push('q');
            }
            out
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        board::board::{Board, Castling},
        common::{colour::Colour, piece::Piece, rank::Rank, square::Square},
    };

    #[test]
    fn castling_to_string() {
        for index in 0..16 {
            let castling1 = Castling::from_u8(index);
            let wk = castling1.get_castling(Colour::White, true);
            let bk = castling1.get_castling(Colour::Black, true);
            let wq = castling1.get_castling(Colour::White, false);
            let bq = castling1.get_castling(Colour::Black, false);
            let mut castling2 = Castling::EMPTY;
            castling2.set_castling(Colour::White, true, wk);
            castling2.set_castling(Colour::Black, true, bk);
            castling2.set_castling(Colour::White, false, wq);
            castling2.set_castling(Colour::Black, false, bq);
            assert_eq!(castling1, castling2);
        }
    }

    #[test]
    fn fen_default() {
        let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .map_err(|s| panic!("invalid fen as {s}"))
            .unwrap();
        assert_eq!(
            board.mailbox(Square::A1),
            Some((Piece::Rook, Colour::White))
        );
        assert_eq!(
            board.mailbox(Square::D8),
            Some((Piece::Queen, Colour::Black))
        );
        assert_eq!(
            board.pieces(Piece::Queen),
            Square::D1.to_bb() | Square::D8.to_bb()
        );
        assert_eq!(
            board.pieces(Piece::Pawn),
            Rank::Two.to_bb() | Rank::Seven.to_bb()
        );
        assert_eq!(board.stm(), Colour::White);
        assert!(board.get_castling(Colour::White, true));
        assert!(board.get_castling(Colour::Black, true));
        assert!(board.get_castling(Colour::White, false));
        assert!(board.get_castling(Colour::Black, false));
        assert_eq!(board.en_passant(), None);
        assert_eq!(board.hmc(), 0);
        assert_eq!(board.fmn(), 1);
    }

    #[test]
    fn fen_custom_position() {
        let board = Board::parse_fen("r3k2r/ppp2ppp/2n5/3pP3/8/2N5/PPP2PPP/R3K2R b Kq d6 17 42")
            .map_err(|s| panic!("invalid fen as {s}"))
            .unwrap();
        assert_eq!(
            board.mailbox(Square::A8),
            Some((Piece::Rook, Colour::Black))
        );
        assert_eq!(
            board.mailbox(Square::E8),
            Some((Piece::King, Colour::Black))
        );
        assert_eq!(
            board.mailbox(Square::E5),
            Some((Piece::Pawn, Colour::White))
        );
        assert_eq!(
            board.mailbox(Square::C3),
            Some((Piece::Knight, Colour::White))
        );
        assert_eq!(board.mailbox(Square::D8), None);
        assert_eq!(board.stm(), Colour::Black);
        assert!(board.get_castling(Colour::White, true));
        assert!(!board.get_castling(Colour::White, false));
        assert!(!board.get_castling(Colour::Black, true));
        assert!(board.get_castling(Colour::Black, false));
        assert_eq!(board.en_passant(), Some(Square::D7));
        assert_eq!(board.hmc(), 17);
        assert_eq!(board.fmn(), 42);
    }

    macro_rules! to_fen_test {
        ($name:ident, $fen:expr) => {
            #[test]
            fn $name() {
                let board = Board::parse_fen($fen).expect("fen incorrect");
                assert_eq!(&*board.to_fen(), $fen)
            }
        };
    }

    to_fen_test!(to_fen_1, "8/8/8/8/8/8/8/K6k w Qk - 0 1");
    to_fen_test!(to_fen_2, "k7/8/8/8/8/8/8/7K b Kk - 0 1");
    to_fen_test!(to_fen_3, "Q6k/8/8/8/8/8/8/K6q w Qq - 0 1");
    to_fen_test!(to_fen_4, "r3k2r/ppp2ppp/8/8/8/8/PPP2PPP/R3K2R w KQkq - 0 1");
    to_fen_test!(to_fen_5, "r6k/8/8/3pP3/8/8/8/K7 b - d6 0 25");
    to_fen_test!(to_fen_6, "8/8/8/8/8/8/8/K6k w KQkq - 0 1");
    to_fen_test!(to_fen_7, "8/8/8/8/8/8/8/K6k b - - 49 100");
    to_fen_test!(to_fen_8, "8/8/8/8/8/8/8/K6k w - - 0 9999");
    to_fen_test!(
        to_fen_9,
        "rnbqk2r/ppp1bppp/3ppn2/8/2B1P3/2N1BN2/PPP2PPP/R2Q1RK1 w kq - 5 12"
    );
    to_fen_test!(to_fen_10, "8/5pk1/3p2p1/1p1P4/1P2P3/5PK1/8/8 w - - 42 67");
    to_fen_test!(to_fen_11, "R3k2r/8/8/8/8/8/8/r3K2R b Kq e6 49 200");
}
