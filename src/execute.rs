use structopt::StructOpt;
use std::path::PathBuf;
use crate::cpu::{CPU};
use crate::assemble;

use std::fs;
use std::io::{self, Read};


impl Execute {
    pub fn run(&self) {
        let mut instructions_str = String::new();

        if self.file.to_string_lossy() == "-" {
            io::stdin().read_to_string(&mut instructions_str).unwrap();
        } else {
            instructions_str = fs::read_to_string(&self.file).expect("Something went wrong reading the file");
        }

        match assemble::parse(&instructions_str) {
            Ok(program) => {
                let mut cpu = CPU::new(self.max_mem_cells);
                let output = cpu.execute(&program);
                println!("{:?}", output);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Execute {
    /// Maximum number of memory cells
    #[structopt(short, long)]
    max_mem_cells: usize,

    /// Loads instructions from a file, or from stdin if `-` is provided
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: PathBuf,
}