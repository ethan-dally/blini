use crate::common::magics::{FindMagic, magic_table};

pub mod board;
pub mod common;

fn main() {
    let x = magic_table();
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
