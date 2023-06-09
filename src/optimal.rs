use structopt::StructOpt;
use std::path::PathBuf;

use std::fs;
use std::io::{self, Read};

use crate::assemble;
use crate::optimise;


#[derive(Debug, StructOpt)]
pub struct Optimal {
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

impl Optimal {
    pub fn run(&self) {
        // Check that exactly one of file or state is provided
        match (&self.file, &self.state) {
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
                match optimise::optimal_from_code(&instructions_str, self.max_length, self.max_mem_cells, self.max_val, self.debug)  {
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
                match optimise::optimal_from_state(s.to_vec(), self.max_length, self.max_val, self.debug) {
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
