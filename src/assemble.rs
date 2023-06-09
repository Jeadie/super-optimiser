use crate::instruction;

use crate::instruction::{Instruction, Program};


pub fn parse(assembly: &str) -> Result<Program, instruction::ParseInstructionError> {
    assembly
        .lines()
        .map(|x| x.parse::<Instruction>()) 
        .collect::<Result<Vec<_>, _>>()
}

pub fn output(program: &[Instruction]) -> String {
    program
        .iter()
        .map(Instruction::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}
