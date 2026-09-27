use std::fmt::Display;

use crate::{
    board::board::Board,
    common::{
        colour::Colour,
        file::File,
        rank::Rank,
        square::Square,
    },
};

impl Board {
    #[allow(dead_code)]
    pub fn display(&self) {
        println!("\n{self}");
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
