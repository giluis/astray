# Project structure

Astray has a front facing [crate](crates.io/crates/astray) which combines its two main crates

1. [Astray Macro](https://github.com/giluis/astray_macro): provides a proc-macro that auto generates parsing functions from Rust type definitions.
2. [Astray Core](https://github.com/giluis/astray_core): holds all other functionality besides the proc-macro itself.

This division happen because a proc-macro crate may only export proc-macros, and Astray requires additional resources besides the proc-macro itself in order to work.

Currently each of the sub crates does not have its own documentation, but I'm currently working on that. 
