use crate::{
    board::board::Board,
    common::{colour::Colour, file::File, piece::Piece, rank::Rank, square::Square},
};
use color_eyre::eyre::{OptionExt, Result, eyre};

impl Board {
    pub fn parse_fen(fen: &str) -> Result<Board> {
        let mut board = Board::default();
        let mut parts = fen.split_whitespace();

        let pieces = parts
            .next()
            .ok_or_else(|| eyre!("missing piece placement"))?;

        let stm = parts
            .next()
            .ok_or_else(|| eyre!("missing side to move"))?;

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
            return Err(eyre!("FEN has too many fields"));
        }

        /*
        pieces
        */
        let mut white_king = 0;
        let mut black_king = 0;
        for (rank, row) in pieces.split('/').enumerate() {

            let Some(rank) = Rank::try_index(7 - rank as u8) else {
                return Err(eyre!("FEN has invalid rank"));
            };

            let mut file_count: u8 = 0;

            for c in row.chars() {

                if c.is_ascii_digit() {
                    file_count += (c
                        .to_digit(9)
                        .ok_or_else(||{eyre!("invalid digit '{c}'")})?
                    ) as u8;
                } else {
                    let file = File::try_index(file_count)
                        .ok_or_else(||{eyre!("invalid file index {file_count}")})?;
                    let sqr = Square::new(rank, file);
                    let piece = Piece::try_from(c)
                        .map_err(|_|{eyre!("invalid piece char {c}")})?;
                    let colour = Colour::from(c.is_ascii_uppercase());
                    if piece == Piece::King {
                        white_king += (colour == Colour::White) as u8;
                        black_king += (colour == Colour::Black) as u8;
                    }
                    board.set_square(sqr, colour, piece);
                    file_count += 1;
                }
            }
        }
        
        if white_king != 1 || black_king != 1 {
            return Err(eyre!(
                "there should be one king on each side w: {} b: {}",
                white_king,
                black_king,
            ));
        }

        /*
        stm
        */
        if stm.len() != 1 {
            return Err(eyre!("fen stm string leng is too long"));
        }

        let stm: char = stm
            .chars()
            .next()
            .ok_or_eyre("unreachable")?;

        let stm = Colour::try_from_char(stm)
                .ok_or_eyre("invalid stm char")?;

        board.set_stm(stm);

        /*
        castling
        */
        if castling.len() > 4 {
            return Err(eyre!("FEN castling string should be of size 4"));
        }

        if castling != "-" {
            for c in castling.chars() {
                match c {
                    'K' => board.set_castling(Colour::White, true, true),
                    'Q' => board.set_castling(Colour::White, false, true),
                    'k' => board.set_castling(Colour::Black, true, true),
                    'q' => board.set_castling(Colour::Black, false, true),
                    _ => return Err(eyre!("fen has incorrect char: {} in castling", c)),
                }
            }
        }

        /*
        en passant
        */
        if en_passant != "-" {
            let en_passant = Square::try_from_str(en_passant)
                .ok_or(eyre!("invalid en_passant str {en_passant}"))?;
            board.set_en_passant(Some(en_passant));
        }

        /*
        half move clock
        */
        let hmc = hmc.parse::<u8>()
            .or(Err(eyre!("FEN hmc '{hmc}' not a valid number")))?;
        if hmc > 50 {
            return Err(eyre!("FEN hmc should be below 50"));
        }
        board.set_hmc(hmc);

        /*
        full move number
        */
        let fmc = fmc.parse::<u32>()
            .or(Err(eyre!("fmc not a valid number")))?;
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
