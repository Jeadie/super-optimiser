use crate::instruction::Instruction;


pub struct CPU {
    max_mem_cells: usize,
    state: Vec<u32>,
}

impl CPU {
    pub fn new(max_mem_cells: usize) -> Self {
        CPU {
            max_mem_cells,
            state: vec![0; max_mem_cells],
        }
    }

    pub fn execute(&mut self, program: &[Instruction]) -> Vec<u32> {
        for instruction in program {
            match instruction {
                Instruction::Load(val) => self.load(*val),
                Instruction::Swap(mem1, mem2) => self.swap(*mem1, *mem2),
                Instruction::Xor(mem1, mem2) => self.xor(*mem1, *mem2),
                Instruction::Inc(mem) => self.inc(*mem),
            }
        }
        return self.state.clone();
    }

    pub fn load(&mut self, val: u32) {
        self.state[0] = val;
    }

    pub fn swap(&mut self, mem1: usize, mem2: usize) {
        self.state.swap(mem1, mem2);
    }

    pub fn xor(&mut self, mem1: usize, mem2: usize) {
        self.state[mem1] ^= self.state[mem2];
    }

    pub fn inc(&mut self, mem: usize) {
        self.state[mem] += 1;
    }   
}
