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
    Optimal(Optimal),
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

#[derive(Debug, StructOpt)]
struct Optimal {
    /// Maximum number of memory cells
    #[structopt(short, long)]
    max_mem_cells: usize,

    /// Maximum length
    #[structopt(short = "l", long = "max-length")]
    max_length: usize,

    /// Maximum value
    #[structopt(short = "v", long = "max-val")]
    max_val: u32,

    /// Enable or disable debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Loads instructions from a file, or from stdin if `-` is provided
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: Option<PathBuf>,

    /// Loads program state vector
    #[structopt(short = "s", long = "state")]
    state: Option<Vec<u32>>,
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
        Command::Optimal(optimal) => {
            // Check that exactly one of file or state is provided
            match (&optimal.file, &optimal.state) {
                // Invalid cases
                (None, None) | (Some(_), Some(_)) => {
                    eprintln!("Error: Please provide exactly one of --file or --state");
                    std::process::exit(1);
                },
                // Program file specified
                (Some(f), None) => {
                    let mut instructions_str = String::new();

                    if f.to_string_lossy() == "-" {
                        io::stdin().read_to_string(&mut instructions_str).unwrap();
                    } else {
                        instructions_str = fs::read_to_string(&f).expect("Something went wrong reading the file");
                    }
                    match optimise::optimal_from_code(&instructions_str, optimal.max_length, optimal.max_mem_cells, optimal.max_val, optimal.debug)  {
                        Ok(result) => {
                            match result {
                                Some(prog) => {
                                    println!("***Optimal***\n{}\n{}\n", assemble::output(&prog), "=".repeat(20));
                                },
                                _ => {
                                    println!("No optimal solution found.")
                                }
                            
                            }
                        },
                        Err(e) => {
                            println!("{:?}", e)
                        }
        
                    }
                },
                // State Array specified
                (None, Some(s)) => {
                    match optimise::optimal_from_state(s.to_vec(), optimal.max_length, optimal.max_val, optimal.debug) {
                        Some(prog) => {
                            println!("***Optimal***\n{}\n{}\n", assemble::output(&prog), "=".repeat(20));
                        },
                        _ => {
                            println!("No optimal solution found.")
                        }
                    
                    }
                }
            }
        }
    }
}