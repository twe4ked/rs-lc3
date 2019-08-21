mod config;
mod cpu;
mod debugger;
mod file_loader;
mod sign_extend;
mod state;
mod trap_vector;

pub use crate::config::Config;
use crate::debugger::debug;
use crate::{file_loader::load_file, sign_extend::SignExtend, state::State};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut state = load_file(config.filename, State::new())?;

    if config.debug {
        debug(state)
    } else {
        while state.running {
            state = state.step()
        }
    }

    Ok(())
}
