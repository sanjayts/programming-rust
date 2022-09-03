# Programming Rust Follow-Along

This repository contains code snippets which were written when trying to work through the "Programming Rust" book.

# Debugging Macros

Unfortunately I wasn't able to get the `trace_macros` to work for me. What I do use for debugging macros is a crate called "cargo-expand". Let's say for example I want to debug macros written as part of a module called `chap_21`, used within unit tests. The command `cargo fmt && cargo expand --lib --tests chap_21` seems to do the job pretty well!