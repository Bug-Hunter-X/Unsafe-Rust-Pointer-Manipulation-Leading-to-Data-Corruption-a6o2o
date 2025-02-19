# Unsafe Rust Pointer Manipulation Bug

This repository demonstrates a common error in Rust:  unsafe pointer manipulation leading to potential data corruption or crashes. The example showcases how modifying a vector's data through a raw pointer without careful consideration of the vector's internal operations can lead to undefined behavior.

The `bug.rs` file contains the faulty code, while `bugSolution.rs` provides a safer alternative.