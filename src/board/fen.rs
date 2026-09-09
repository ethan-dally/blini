use crate::{board::board::Board, common::{colour::Colour, piece::Piece, rank::Rank}};

impl Board {
    fn parse_fen(fen: &str) -> Result<Board, &str> {
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

        for (rank, row) in pieces.split('/').enumerate() {
            let Some(rank) = Rank::try_index(rank as u8) else {
                return Err("fen has invalid ranks");
            };
            let mut file = 0;

            for c in row.chars() {
                if c.is_ascii_digit() {
                    file += c.to_digit(9).ok_or("invalid digit")?;
                } else {
                    let piece = Piece::try_from(c).map_err(|_|{"invalid char"})?;
                    let colour = Colour::from(c.is_ascii_uppercase());

                }
            }
        }
        todo!()
    }
}