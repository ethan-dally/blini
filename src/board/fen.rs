use crate::{board::board::Board, common::{colour::Colour, file::File, piece::Piece, rank::Rank, square::Square}};

impl Board {
    fn parse_fen(fen: &str) -> Result<Board, &str> {
        let mut board = Board::default();
        let mut parts = fen.split_whitespace();
        let pieces = parts.next().ok_or("invalid whitespace")?;
        let stm = parts.next().ok_or("invalid whitespace")?;
        let castling = parts.next().ok_or("invalid whitespace")?;
        let en_passant = parts.next().ok_or("invalid whitespace")?;
        let hmc = parts.next().ok_or("invalid whitespace")?;
        let fmc = parts.next().ok_or("invalid whitespace")?;
        if parts.next().is_some() {
            return Err("fen has too much whitespace");
        }

        /*
        pieces
        */
        for (rank, row) in pieces.split('/').enumerate() {
            let Some(rank) = Rank::try_index(rank as u8) else {
                return Err("fen has invalid ranks");
            };
            let mut file_count: u8 = 0;

            for c in row.chars() {
                if c.is_ascii_digit() {
                    file_count += (c.to_digit(9).ok_or("invalid digit")?) as u8;
                } else {
                    let file = File::try_index(file_count).ok_or("invalid fen sum")?;
                    let sqr = Square::new(rank, file);
                    let piece = Piece::try_from(c).map_err(|_|{"invalid char"})?;
                    let colour = Colour::from(c.is_ascii_uppercase());
                    board.set_square(sqr, colour, piece);
                }
            }
        }

        /*
        stm
        */
        if stm.len() != 1 {return Err("fen stm too long");}
        let stm: char = stm.chars().next().ok_or("unreachable")?;
        let stm = Colour::try_from_char(stm).ok_or("invalid stm char")?;
        board.set_stm(stm);

        //todo complete

        Ok(board)
    }
}