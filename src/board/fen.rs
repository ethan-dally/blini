use crate::{
    board::board::Board,
    common::{colour::Colour, file::File, piece::Piece, rank::Rank, square::Square},
};

impl Board {
    pub fn parse_fen(fen: &str) -> Result<Board, String> {
        let mut board = Board::default();
        let mut parts = fen.split_whitespace();
        let pieces = parts.next().ok_or("invalid whitespace")?;
        let stm = parts.next().ok_or("invalid whitespace")?;
        let castling = parts.next().ok_or("invalid whitespace")?;
        let en_passant = parts.next().ok_or("invalid whitespace")?;
        let hmc = parts.next().ok_or("invalid whitespace")?;
        let fmc = parts.next().ok_or("invalid whitespace")?;
        if parts.next().is_some() {
            return Err("fen has too much whitespace".to_string());
        }

        /*
        pieces
        */
        for (rank, row) in pieces.split('/').enumerate() {
            let Some(rank) = Rank::try_index(7 - rank as u8) else {
                return Err("fen has invalid ranks".to_string());
            };
            let mut file_count: u8 = 0;
            for c in row.chars() {
                if c.is_ascii_digit() {
                    file_count += (c.to_digit(9).ok_or("invalid digit")?) as u8;
                } else {
                    let file = File::try_index(file_count).ok_or("invalid fen sum")?;
                    let sqr = Square::new(rank, file);
                    let piece = Piece::try_from(c).map_err(|_| "invalid char")?;
                    let colour = Colour::from(c.is_ascii_uppercase());
                    board.set_square(sqr, colour, piece);
                    file_count += 1;
                }
            }
        }

        /*
        stm
        */
        if stm.len() != 1 {
            return Err("fen stm too long".to_string());
        }
        let stm: char = stm.chars().next().ok_or("unreachable")?;
        let stm = Colour::try_from_char(stm).ok_or("invalid stm char")?;
        board.set_stm(stm);

        /*
        castling
        */
        if castling.len() > 4 {
            return Err("fen castling string length too big".to_string());
        }
        if castling != "-" {
            for c in castling.chars() {
                match c {
                    'K' => board.set_castling(Colour::White, true, true),
                    'Q' => board.set_castling(Colour::White, false, true),
                    'k' => board.set_castling(Colour::Black, true, true),
                    'q' => board.set_castling(Colour::Black, false, true),
                    _ => return Err(format!("fen has incorrect char: {} in castling", c)),
                }
            }
        }

        /*
        en passant
        */
        if en_passant != "-" {
            let en_passant = Square::try_from_str(en_passant)
                .ok_or(format!("invalid en_passant str {en_passant}"))?;
            board.set_en_passant(Some(en_passant));
        }

        /*
        half move clock
        */
        let hmc = hmc.parse::<u8>().or(Err("hmc not a valid number"))?;
        if hmc > 50 {
            return Err("fen hmc above 50".to_string());
        }
        board.set_hmc(hmc);

        /*
        full move number
        */
        let fmc = fmc.parse::<u32>().or(Err("fmc not a valid number"))?;
        board.set_fmn(fmc);

        Ok(board)
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
    assert_eq!(board.en_passant(), Some(Square::D6));
    assert_eq!(board.hmc(), 17);
    assert_eq!(board.fmn(), 42);
}
