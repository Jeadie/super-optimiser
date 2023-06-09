# super-optimiser
Blazingly fast super-optimiser

## Overview
super-optimiser performs superoptimization on a basic, fictional assembly language. It is a Rust reimplementation of [Austin Henley's](https://austinhenley.com/blog/superoptimizer.html), with some performance/algorithm improvements (WIP) to make it blazingly fast.

## Assembly Level
The assembly language is simply: 
- `LOAD val` which loads the immediate value into memory location 0.
- `SWAP mem, mem` which swaps the values of the two memory locations.
- `XOR mem, mem` which performs a bitwise XOR operation on the values of the memory locations and stores the result in the first's location.
- `INC mem` which increments the value at the memory location.
For example:
```asm
LOAD 3
SWAP 0 1
XOR 1 2
INC 2
```

## Getting Started
### Installation
Clone the repository and build the project using `cargo`, the Rust package manager:

```sh
git clone https://github.com/Jeadie/super-optimiser.git
cd super-optimiser
cargo build --release
```

### Usage
After building the project, you can execute an assembly program:
```shell
cli execute --max-mem-cells 4 -f program.asm
```
Or, to run the super-optimiser, and find an optimal solution
```shell
cli optimal --max-length 5 --max-mem-cells 4 --max-val 5 -f program.asm
```
Since the result of the assembly program is solely described by the final state of memory, it's possible to find the optimal assembly program that results in a given output state:
```
cli optimal --max-length 5 --max-mem-cells 4 --max-val 5 --state 3 2 4 1
```

### Benchmarking
Benchmarking exists as a separate target. It is not CLI friendly yet, but can be found in [here](src/benchmark.rs)/

## Contribution Guidelines
All contributors are welcome. Whether you're fixing bugs, improving the documentation, or proposing and implementing new features, we appreciate your efforts. Please see the [CONTRIBUTING.md](./CONTRIBUTING.md) file for more detailed information.

