use crate::common::magics::{FindMagic, magic_table};

pub mod board;
pub mod common;

fn main() {
    let x = magic_table();
    println!("size: {}", x.size_of());
    // let _ = FindMagic::find_all_magics();
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
