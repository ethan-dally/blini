use crate::common::magics::{magic_table};
use color_eyre::Result;

pub mod board;
pub mod common;

fn main() -> Result<()> {
    color_eyre::install()?;
    let _magic_table = magic_table();
    Ok(())
}