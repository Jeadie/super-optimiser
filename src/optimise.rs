use std::collections::HashMap;

use crate::cpu::CPU;
use crate::assemble;
use crate::instruction::{Instruction, Program};


pub fn optimal_from_code(assembly: &str, max_length: usize, max_mem: usize, max_val: u32, debug: bool) {
    let mut cpu = CPU::new(max_mem);
    match assemble::parse(assembly) {
        Ok(program) => {
            let state = cpu.execute(&program);
            println!("***Source***\n{}", assembly);
            optimal_from_state(state, max_length, max_val, debug);
        },
        Err(err) => println!("Error parsing program: {:?}", err),
    }
}

pub fn optimal_from_state(state: Vec<u32>, max_length: usize, max_val: u32, debug: bool) {
    let max_mem = state.len();
    println!("***State***\n{:?}\n", state);
    let mut opt = Superoptimizer::new();
    match opt.search(max_length, max_mem, max_val, state, debug) {
        Some(shortest_program) => {
            let disassembly = assemble::output(&shortest_program);
            println!("***Optimal***\n{}\n{}\n", disassembly, "=".repeat(20));
        },
        None => println!("No optimal solution found."),
    }
}

pub fn multi_cartesian_product<T: Clone>(sets: Vec<Vec<T>>) -> Vec<Vec<T>> {
    match sets.split_first() {
        Some((first, rest)) => {
            let rest_product = multi_cartesian_product(rest.to_vec());

            first.iter().flat_map(|f| {
                rest_product.iter().map(move |r| {
                    let mut combination = vec![f.clone()];
                    combination.extend(r.clone());
                    combination
                })
            }).collect()
        },
        None => vec![vec![]],
    }
}


pub struct Superoptimizer {
    program_cache: HashMap<Vec<u32>, Program>,
}

impl Superoptimizer {
    pub fn new() -> Self {
        Superoptimizer {
            program_cache: HashMap::new(),
        }
    }

    pub fn generate_programs(&self, max_program_length: usize, max_mem: usize, max_val: u32) -> Vec<Program> {
        let mut programs: Vec<Program> = vec![];
        for length in 1..=max_program_length {

            // values used in `Instruction` not important
            let permutations: Vec<Program> = Self::permute(vec![Instruction::Load(0), Instruction::Swap(0, 0), Instruction::Xor(0, 0), Instruction::Inc(0)], length);

            for prog in permutations {
                let mut arg_sets: Vec<Vec<Instruction>> = vec![];
                for ins in prog {
                    match ins {
                        Instruction::Load(_) => {
                            arg_sets.push(Vec::from_iter((0..=max_val).map(|v|  Instruction::Load(v.try_into().unwrap()))))
                        }
                        Instruction::Swap(_, _) => {
                            arg_sets.push(Vec::from_iter((0..=max_mem - 1).flat_map(|a| {
                                (0..=max_mem - 1).map(move |b| Instruction::Swap(a.try_into().unwrap(), b.try_into().unwrap()))
                            }).collect::<Vec<Instruction>>()));
                        }
                        Instruction::Xor(_, _) => {
                            arg_sets.push(Vec::from_iter((0..=max_mem - 1).flat_map(|a| {
                                (0..=max_mem - 1).map(move |b| Instruction::Xor(a.try_into().unwrap(), b.try_into().unwrap()))
                            }).collect::<Vec<Instruction>>()));
                        }
                        Instruction::Inc(_) => {
                            arg_sets.push(Vec::from_iter((0..=max_mem -1 ).map(|v|  Instruction::Inc(v.try_into().unwrap()))))
                        }
                    }
                }
                for arg_set in multi_cartesian_product(arg_sets) {
                    programs.push(arg_set);
                }
            }
        }
    programs
}
    // Helper function to generate permutations of instructions
    fn permute(instructions: Vec<Instruction>, length: usize) -> Vec<Program> {
        if length == 0 {
            return vec![vec![]];
        }
        
        let mut result = vec![];

        for ins in &instructions {
            for mut perm in Self::permute(instructions.clone(), length - 1) {
                perm.insert(0, ins.clone());
                result.push(perm);
            }
        }

        result
    }

    pub fn search(&mut self, max_length: usize, max_mem: usize, max_val: u32, target_state: Vec<u32>, debug: bool) -> Option<Program> {
        let mut count = 0;
        for program in self.generate_programs(max_length, max_mem, max_val) {
            let mut cpu = CPU::new(max_mem);
            let state = cpu.execute(&program);
            if state == target_state {
                let state = state.clone();
                if !self.program_cache.contains_key(&state) || program.len() < self.program_cache[&state].len() {
                    self.program_cache.insert(state, program);
                }
            }
            if debug {
                count += 1;
                if count % 1_00_000 == 0 { println!("Programs searched: {}", count); }
                if count % 10_000_000 == 0 {
                    match self.program_cache.get(&target_state) {
                        Some(solution) => println!("Best solution: {:?}", solution),
                        None => println!("No solution found yet"),
                    }
                }
            }
        }
        self.program_cache.get(&target_state).cloned()
    }
}
