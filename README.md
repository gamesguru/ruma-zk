# ZK-Matrix-Join: Trustless Light Clients for the Matrix Protocol

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](#)
[![SP1 zkVM](https://img.shields.io/badge/sp1-zkVM-blue.svg)](#)
[![Status](https://img.shields.io/badge/status-experimental_AF-red.svg)](#)

This repository is a conceptual implementation of a Layer-2 Zero-Knowledge scaling solution for the Matrix decentralized communication protocol.

Specifically, we are replacing the concept of **Partial Joins** with **ZK-Joins**.

## The Problem: Trust vs. Time

When a Matrix homeserver joins a federated room today, it faces a dilemma:

1. **Status Quo (Full Join):** To **trustlessly** join a room, a server must download the room's massive historical "Auth Chain" DAG and locally execute the complex State Resolution v2 algorithm from genesis. For large rooms like `#matrix:matrix.org`, this contains hundreds of thousands of events, taking an enormous amount of RAM and CPU. This takes forever.
2. **Faster Joins (MSC3902):** A temporary fix where a server _blindly trusts_ the remote server's assertion of the "current state" so the user can chat immediately, while it secretly downloads the gigabytes of history and verifies it in the background. This is ultimately a compromise on decentralization.

## The Solution: Math over Computation

`zk-matrix-join` introduces a Zero-Knowledge architecture to Matrix state resolution.

By providing a succinct STARK proof alongside the current state, the joining server can verify that the state was calculated correctly from genesis in _milliseconds_.

Instead of every individual homeserver downloading 50MB of Auth Chain and running Kahn's topological sort and Ed25519 signature verification on 500,000 events, a prover node handles the heavy lifting inside a Gen-Purpose **zkVM** (like SP1). This node computes the state resolution and generates a Zero-Knowledge recursive STARK proving that the resulting state conforms exactly to Matrix protocol rules.

Standard residential servers (or even browser light clients via WebAssembly) can perform a **Trustless ZK-Join**: they download just the latest state (2MB) and a tiny STARK proof (250KB). They verify the proof instantly and participate with 100% cryptographically guaranteed trustlessness.

## Architecture

This project strictly utilizes the **SP1 RISC-V zkVM framework**, not raw circuit arithmetization, which permits native Rust libraries like `ruma-state-res` to function effectively in ZK. It is split into three components:

- `src/host/`: The Prover orchestration. It natively resolves State Res v2, pre-sorts Matrix DAG branches, and constructs linear "Hints" containing compressed byte arrays for the guest. It then compresses the resulting SP1 STARK into a 300-byte Growth16 SNARK.
- `src/guest/`: The RISC-V zkVM program. Because $O(N \log N)$ sorting burns immense CPU cycles in a VM, the guest linearly verifies the Host's Hint arrays in $O(N)$ execution time, using embedded target ASM features like `core::arch::asm!` for optimized memory hashing.
- `src/wasm-client/`: A `cdylib` crate exposing the SNARK verification to pure JavaScript via `wasm-bindgen`, utilizing `ark-bn254` for <15ms verification times.

## Language Considerations and Overhead

Implementing complex cryptographic operations, such as Zero-Knowledge recursive STARK proving and State Resolution v2, introduces significant computational overhead. While this repository is written in Rust for optimal performance and safety, it's important to consider the implications of portability to other languages:

- **Python & JavaScript/TypeScript:** These languages are ubiquitous but notoriously slow for heavy, repeated mathematical iterations (like finite-field arithmetic and hash-sorting constraints). Implementing the Prover in interpreted or JIT-compiled languages would lead to unacceptable latency for real-time communication. Native implementations of State Resolution v2 in Python (e.g., Synapse) already face performance bottlenecks; adding ZK constraints natively would geometrically compound this issue.
- **The Optimistic Path:** Fortunately, there is a clear path forward for both ecosystems:
  - **Python Servers:** Homeservers like Synapse can easily be salvaged by leveraging native C/Rust extensions (like PyO3), offloading only the heavy cryptography to this compiled core while keeping their higher-level networking logic untouched in Python.
  - **JavaScript Clients:** The Matrix SDK primarily functions as a client, so it only ever needs to run the lightweight Verifier. While running verification in _pure_ JS/TS would be too slow (due to a lack of native big-integer acceleration), compiling the verification logic to WebAssembly completely solves this. This allows standard browser clients to verify a ZK proof incredibly fast directly in the browser using Wasm bindings.

## Status

Highly experimental. We use the SP1 Prover paired with Verifiable Computation (The Hinting Paradigm) to scale Matrix topology resolution to 1,000,000+ events. Check out the MSC Proposal or run the WebAssembly demo.

## License

Dual-licensed under MIT or Apache 2.0.
