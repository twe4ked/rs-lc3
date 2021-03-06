mod cpu;
mod debugger;
mod file;
mod instruction;
mod state;

use crate::debugger::Debugger;
use crate::state::State;
use std::error::Error;

pub fn run(filename: String, debug: bool) -> Result<(), Box<dyn Error>> {
    let mut rom = file::read_rom(filename)?;
    let mut state = State::new();
    state.load_rom(&mut rom)?;

    if debug {
        let mut debugger = Debugger::new();
        debugger.step(state)
    } else {
        while state.running {
            state = state.step()
        }
    }

    Ok(())
}
