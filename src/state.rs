pub mod memory;
pub mod registers;

use crate::cpu::execute;
use crate::instruction::{Instruction, Register};
use memory::Memory;
use registers::Registers;

pub struct State {
    pub memory: Memory,
    pub registers: Registers,
    pub pc: u16,
    pub condition: Condition,
    pub running: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
            pc: 0x0000,
            condition: Condition::P,
            running: true,
        }
    }

    pub fn update_flags(&mut self, r: Register) -> &Self {
        if self.registers.read(r) == 0 {
            self.condition = Condition::Z;
        } else if (self.registers.read(r) >> 15) == 1 {
            // NOTE: A 1 in the left-most bit indicates negative
            self.condition = Condition::N;
        } else {
            self.condition = Condition::P;
        }

        self
    }

    pub fn step(mut self) -> Self {
        let instruction = self.memory.read(self.pc);
        let instruction = Instruction::decode(instruction);
        execute(self, instruction)
    }

    pub fn registers(&self) -> [u16; 8] {
        self.registers.registers()
    }

    pub fn load_rom(&mut self, rom: &mut [u16]) -> Result<(), &str> {
        let mut rom = rom.iter();
        let address = match rom.next() {
            Some(a) => a,
            None => return Err("ROM must be at least 2 bytes."),
        };
        let mut address = *address;
        self.pc = address;

        for value in rom {
            self.memory.write(address, *value);
            address += 1;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    P,
    Z,
    N,
}
