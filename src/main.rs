use color_eyre::Result;

use crate::{
    common::magics::magic_table,
    uci::Engine,
};

mod bench;
mod board;
mod common;
mod search;
mod uci;

fn main() -> Result<()> {
    color_eyre::install()?;
    let _magic_table = magic_table();
    Engine::default().run()?;
    Ok(())
}
