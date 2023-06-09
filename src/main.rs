mod assemble;
mod cpu;
mod instruction;
mod optimise;

use structopt::StructOpt;
use std::path::PathBuf;
use cpu::{CPU};  // Make sure to replace with your actual crate name


use std::fs;
use std::io::{self, Read};

#[derive(Debug, StructOpt)]
#[structopt(name = "My Super CLI", about = "Does awesome things.")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Execute(Execute),
}

#[derive(Debug, StructOpt)]
struct Execute {
    /// Maximum number of memory cells
    #[structopt(short, long)]
    max_mem_cells: usize,

    /// Loads instructions from a file, or from stdin if `-` is provided
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Execute(execute) => {
            let mut instructions_str = String::new();

            if execute.file.to_string_lossy() == "-" {
                io::stdin().read_to_string(&mut instructions_str).unwrap();
            } else {
                instructions_str = fs::read_to_string(&execute.file).expect("Something went wrong reading the file");
            }

            match assemble::parse(&instructions_str) {
                Ok(program) => {
                    let mut cpu = CPU::new(execute.max_mem_cells);
                    let output = cpu.execute(&program);
                    println!("{:?}", output);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        },
    }
}