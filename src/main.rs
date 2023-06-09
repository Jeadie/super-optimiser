mod assemble;
mod cpu;
mod instruction;
mod optimise;

fn main() {
    // Test 1
    let assembly = "LOAD 3
SWAP 0 1
LOAD 3
SWAP 0 2
LOAD 3
SWAP 0 3
LOAD 3
";

    // Execute a single Program on a CPU of 4 memory cells
    // let mut c = cpu::CPU::new(4);
    // let program  = &vec!(
    //     Instruction::Load(3),
    //     Instruction::Xor(1, 0),
    //     Instruction::Xor(2, 0),
    //     Instruction::Xor(3, 0)
    // );
    // c.execute(program, true);

    // ***Optimal***
    // LOAD 3
    // XOR 1, 0
    // XOR 2, 0
    // XOR 3, 0
    optimise::optimal_from_code(assembly, 4, 4, 5, false);

    // ***Optimal***
    // LOAD 2
    // SWAP 0, 1
    // INC 2
    optimise::optimal_from_state(Vec::from([0, 2, 1]), 3, 5, false)
}
