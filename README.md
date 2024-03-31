[![Compile, format, lint & test](https://github.com/timKraeuter/FEM-Last-Algorithm-Course-You-Need-Rust/actions/workflows/rust.yml/badge.svg)](https://github.com/timKraeuter/FEM-Last-Algorithm-Course-You-Need-Rust/actions/workflows/rust.yml)

# Overview

The last algorithm course you need from [FEM](https://frontendmasters.com/courses/algorithms/) but implemented using **Rust**.
Slides for the course are available [here](https://theprimeagen.github.io/fem-algos).

Currently, we have not implemented the course data structures since those are notoriously difficult, if not impossible, using safe Rust.

# Solutions & Problems

The solutions are in the [`solutions`](./src/solutions) dir, while the [`problems`](./src/problems) dir contains the problems without implementation.
Each problem file has a minimal set of test cases you can run after implementing the problem. The tests are usually taken from the [official repository](https://github.com/ThePrimeagen/kata-machine).

Rust allows for unique solutions for some problems due to its unique capabilities (see, for example, [comparing binary trees](./src/solutions/compare_binary_trees.rs)).

# Actions
The GitHub Actions automatically check formatting using the [rust-formatter](https://github.com/rust-lang/rustfmt) and lint using [rust-clippy](https://github.com/rust-lang/rust-clippy).
In addition, all tests in the solutions dir are run.
