use crate::opcode::Opcode;
use crate::state::*;
use crate::trap_vector::TrapVector;
use crate::utilities::sign_extend;

use lazy_static::lazy_static;
use regex::Regex;
use rustyline;

pub(crate) fn debug(mut state: State) -> State {
    let mut rl = rustyline::Editor::<()>::new();
    let readline = rl.readline(&format!("{:#04x}> ", state.pc));

    let instruction : u16 = state.read_memory(state.pc);
    let opcode = Opcode::from_instruction(instruction);

    lazy_static! {
        static ref READ_REGEX: Regex = Regex::new(r"^read 0x([a-f0-9]{1,4})$").unwrap();
        static ref BREAK_ADDRESS_REGEX: Regex = Regex::new(r"^break-address 0x([a-f0-9]{1,4})$").unwrap();
    }

    state.debug_continue = false;

    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_ref());

            match line.as_ref() {
                "c" | "continue" => {
                    state.debug_continue = true;
                }

                "i" | "inspect" => {
                    println!("{:?}, op_code: {:?}, instruction: {:#4x}, {:#016b}", state, opcode, instruction, instruction);
                }

                "d" | "disassemble" => {
                    disassemble(instruction, opcode);
                }

                line if READ_REGEX.is_match(line) => {
                    if let Some(address) = READ_REGEX.captures(line).unwrap().get(1) {
                        let address = u16::from_str_radix(address.as_str(), 16).unwrap();
                        let value = state.read_memory(address);
                        println!("{:#04x}, {:#016b}", value, value);
                    }
                }

                line if BREAK_ADDRESS_REGEX.is_match(line) => {
                    if let Some(address) = BREAK_ADDRESS_REGEX.captures(line).unwrap().get(1) {
                        let address = u16::from_str_radix(address.as_str(), 16).unwrap();
                        state.break_address = Some(address);
                        println!("Break address set to {:#04x}", address);
                    }
                }

                "exit" => {
                    state.running = false;
                }

                "" => {
                    // Allow hitting enter
                }

                _ => {
                    println!("Unknown command {:?}", line);
                }
            }
        },
        Err(rustyline::error::ReadlineError::Interrupted) => {
            state.running = false;
        },
        Err(rustyline::error::ReadlineError::Eof) => {
            state.running = false;
        },
        Err(err) => {
            println!("Error: {:?}", err);
            state.running = false;
        }
    }

    state
}

fn disassemble(instruction: u16, opcode: Opcode) {
    print!("{:?}, {:08b}_{:08b}, ", opcode, (instruction >> 8) & 0xff, instruction & 0xff);

    match opcode {
        Opcode::BR => {
            let n = (instruction >> 11) & 0x1;
            let z = (instruction >> 10) & 0x1;
            let p = (instruction >> 9) & 0x1;

            println!("n: {}, z: {}, p: {}", n, z, p);
        }

        Opcode::ADD => {
            let r0 = (instruction >> 9) & 0x7;
            let r1 = (instruction >> 6) & 0x7;
            let immediate_flag = ((instruction >> 5) & 0x1) == 0x1;

            println!("r0: {}, r1: {}, immediate_flag: {}", r0, r1, immediate_flag);
        }

        Opcode::LD => {
            let r0 = (instruction >> 9) & 0x7;
            let pc_offset = instruction & 0x1ff;

            println!("r0: {}, pc_offset: {}", r0, pc_offset);
        }

        Opcode::ST => {
            let r0 = (instruction >> 9) & 0x7;
            let pc_offset = instruction & 0x1ff;

            println!("r0: {}, pc_offset: {}", r0, pc_offset);
        }

        Opcode::JSR => {
            let use_pc_offset = (instruction >> 11) & 1;
            let pc_offset = instruction & 0x1ff;
            let r0 = (instruction >> 6) & 7;

            println!("use_pc_offset: {}, pc_offset: {}, r0: {}", use_pc_offset, pc_offset, r0);
        }

        Opcode::AND => {
            let immediate_flag = ((instruction >> 5) & 1) == 1;
            let immediate_value = sign_extend(instruction & 0x1f, 5);

            let r0 = (instruction >> 9) & 0x7;
            let r1 = (instruction >> 6) & 0x7;
            let r2 = (instruction) & 0x7;

            println!("immediate_flag: {}, immediate_value: {}, r0: {}, r1: {}, r2: {}",
                     immediate_flag, immediate_value, r0, r1, r2);
        }

        Opcode::LDR => {
            let r0 = (instruction >> 9) & 0x7;
            let r1 = (instruction >> 6) & 0x7;
            let offset = (instruction) & 0x3f;

            println!("r0: {}, r1: {}, offset: {}", r0, r1, offset);
        }

        Opcode::STR => {
            let r0 = (instruction >> 9) & 0x7;
            let r1 = (instruction >> 6) & 0x7;
            let offset = instruction & 0x3f;

            println!("r0: {}, r1: {}, offset: {}", r0, r1, offset);
        }

        Opcode::UNUSED => {
            panic!("unused");
        }

        Opcode::NOT => {
            let r0 = (instruction >> 9) & 0x7;
            let r1 = (instruction >> 6) & 0x7;

            println!("r0: {}, r1: {}", r0, r1);
        }

        Opcode::LDI => {
            let r0 = (instruction >> 9) & 0x7;
            let pc_offset = sign_extend(instruction & 0x1ff, 9);

            println!("r0: {}, pc_offset: {}", r0, pc_offset);
        }

        Opcode::STI => {
            let r0 = (instruction >> 9) & 0x7;
            let pc_offset = instruction & 0x1ff;

            println!("r0: {}, pc_offset: {}", r0, pc_offset);
        }

        Opcode::JMP => {
            let r0 = (instruction >> 6) & 0xa;

            println!("r0: {}", r0);
        }

        Opcode::RESERVED => {
            panic!("reserved");
        }

        Opcode::LEA => {
            let r0 = (instruction >> 9) & 0x7;
            let pc_offset = instruction & 0x1ff;

            println!("r0: {}, pc_offset: {}", r0, pc_offset);
        }

        Opcode::TRAP => {
            if let Ok(trap_vector) = TrapVector::from_instruction(instruction) {
                println!("trap_vector: {:?}", trap_vector);
            }
        }
    }
}
