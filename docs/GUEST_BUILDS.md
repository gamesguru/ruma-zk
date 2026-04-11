# Jolt Guest Build Documentation

This document explains how the Jolt Guest ELFs are managed and built in this project.

## Overview

Jolt is a zkVM that executes RISC-V guest code and generates zero-knowledge proofs. In this project, the guest code resides in the following crates:

- `ruma_zk_guest`: The optimized path using topological reducer logic.
- `demo_unoptimized_guest`: The unoptimized path running full Matrix State Resolution.

## Prerequisites

To build and run the guests, you need the following:

1.  **RISC-V Toolchain**: The `riscv64imac-unknown-none-elf` target must be added to your Rust toolchain.
2.  **Jolt CLI**: The `cargo-jolt` tool is useful for standalone guest management and robust builds.

## Setup

You can automate the setup by running:

```bash
make setup-jolt
```

This will run:

- `rustup target add riscv64imac-unknown-none-elf`
- `cargo install --git https://github.com/a16z/jolt.git cargo-jolt`

## Building Guests

While Jolt attempts to build guests automatically on the host side, it can sometimes fail in complex workspace environments or when using `--trace`. We recommend pre-building the guests via the Makefile:

```bash
make build-guest
```

This target ensures that both optimized and unoptimized guest ELFs are compiled and available in the target directory (or `/tmp/jolt-guest-targets` as managed by Jolt).

## Why is this needed?

- **Simulation**: Can run natively on the host (no ELF needed).
- **Tracing/Proving**: Requires the RISC-V ELF to execute the guest in the virtual machine. Without a pre-built ELF, you may encounter a `Built ELF not found` panic.

## Troubleshooting

If you see `Built ELF not found`:

1.  Run `make build-guest`.
2.  Ensure `Program::new` in `main.rs` uses the correct path (e.g., `./ruma_zk_guest`).
3.  Check for compilation errors in the guest crates by running `cargo check -p ruma_zk_guest --target riscv64imac-unknown-none-elf`.
