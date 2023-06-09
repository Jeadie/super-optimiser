mod assemble;
mod cpu;
mod instruction;
mod optimise;

use crate::instruction::Instruction;

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
    // let mut c = cpu::CPU::new(4);
    // c.execute(&vec!(Instruction::Load(3), Instruction::Swap(2, 3), Instruction::Swap(1, 3)), true);
    // LOAD 3
    // XOR 1, 0
    // XOR 2, 0
    // XOR 3, 0
    optimise::optimal_from_code(assembly, 4, 4, 5, false);

//     match assemble::parse(assembly) {
//         Ok(program) => {
//             let mut cpu = cpu::CPU::new(4);
//             let state = cpu.execute(&program);
//             println!("***Source***\n{}\n***State***\n{:?}\n", assembly, state);
//             let mut opt = optimise::Superoptimizer::new();
//             match opt.search(4, 4, 5, state, true) {
//                 Some(shortest_program) => {
//                     let disassembly = assemble::output(&shortest_program);
//                     println!("***Optimal***\n{}\n{}", disassembly, "=".repeat(20));
//                 },
//                 None => println!("No optimal solution found."),
//             }
//         },
//         Err(err) => println!("Error parsing program: {:?}", err),
//     }
// 
//     let state = vec![0, 2, 1];
//     println!("***State***\n{:?}\n", state);
//     let mut opt = optimise::Superoptimizer::new();
//     match opt.search(3, 3, 5, state, true) {
//         Some(shortest_program) => {
//             let disassembly = assemble::output(&shortest_program);
//             println!("***Optimal***\n{}\n{}", disassembly, "=".repeat(20));
//         },
//         None => println!("No optimal solution found."),
//     }
}
