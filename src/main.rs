use crate::common::{magics::Magic, square::Square};

pub mod board;
pub mod common;

fn main() {
    let magic = Magic::dbg_find_magic(Square::A1, false);
    println!("magic: {:?}", magic.unwrap())
}

//MASK - produces keys
//  · · · · · · · ·
//  · · · · · · X ·
//  · X · · · X · ·
//  · · X · X · · ·
//  · · · B · · · ·
//  · · X · X · · ·
//  · X · · · X · ·
//  · · · · · · · ·

// KEYS
//  · · · · · · · ·
//  · · · · · · X ·
//  · · · · · · · ·
//  · · · · X · · ·
//  · · · B · · · ·
//  · · X · · · · ·
//  · X · · · X · ·
//  · · · · · · · ·

// VALUE - attack positions
//  · · · · · · · ·
//  X · · · · · · ·
//  · X · · · · · ·
//  · · X · X · · ·
//  · · · B · · · ·
//  · · X · X · · ·
//  · · · · · X · ·
//  · · · · · · · ·
