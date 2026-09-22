use crate::{
    board::{board::Board, perft::FlagsCount},
    common::{colour::Colour, file::File, r#move::MoveFlag, rank::Rank, square::Square},
};
use std::fmt::Display;

impl Board {
    pub fn display(&self) {
        println!("\n{self}")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   A     B     C     D     E     F     G     H   ")?;
        writeln!(f, "╔═════╤═════╤═════╤═════╤═════╤═════╤═════╤═════╗")?;
        for rank in Rank::ALL.iter().rev() {
            write!(f, "║  ")?;
            for file in File::ALL {
                let sqr = Square::new(*rank, file);
                let piece = self
                    .mailbox(sqr)
                    .map(|p| {
                        if p.1 == Colour::White {
                            char::from(p.0).to_ascii_uppercase()
                        } else {
                            char::from(p.0).to_ascii_lowercase()
                        }
                    })
                    .unwrap_or(' ');
                if file == File::H {
                    write!(f, "{piece}  ")?;
                } else {
                    write!(f, "{piece}  │  ",)?;
                }
            }
            writeln!(f, "║ {}", char::from(*rank))?;
            if *rank != Rank::One {
                writeln!(f, "╟─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────╢")?;
            }
        }
        writeln!(f, "╚═════╧═════╧═════╧═════╧═════╧═════╧═════╧═════╝")
    }
}

impl FlagsCount {
    pub fn display(&self) {
        println!(
            "NonCapture: {}, PawnDouble: {}, CastleShort: {}, CastleLong: {}, \
             Promotion: {}, Capture: {}, EnPassant: {}",
            self.0[MoveFlag::NonCapture],
            self.0[MoveFlag::PawnDouble],
            self.0[MoveFlag::CastleShort],
            self.0[MoveFlag::CastleLong],
            self.0[MoveFlag::PromotionQueen]
                + self.0[MoveFlag::PromotionRook]
                + self.0[MoveFlag::PromotionBishop]
                + self.0[MoveFlag::PromotionKnight],
            self.0[MoveFlag::Capture],
            self.0[MoveFlag::EnPassant],
        );
    }
}
