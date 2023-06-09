# super-optimiser
Blazingly fast super-optimiser

## Overview
Reimplementing [My first superoptimizer](https://austinhenley.com/blog/superoptimizer.html) and making it blazingly fast.

## Assembly Level
- `LOAD val` which loads the immediate value into memory location 0.
- `SWAP mem, mem` which swaps the values of the two memory locations.
- `XOR mem, mem` which performs a bitwise XOR operation on the values of the memory locations and stores the result in the first's location.
- `INC mem` which increments the value at the memory location.