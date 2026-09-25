use crate::{common::magics::magic_table, uci::Engine};
use color_eyre::Result;

mod board;
mod common;
mod uci;
mod bench;
mod search;

fn main() -> Result<()> {
    color_eyre::install()?;
    let _magic_table = magic_table();
    Engine::default().run()?;
    Ok(())
}