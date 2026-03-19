# ZK-Matrix-Join: Trustless Light Clients for the Matrix Protocol

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)]()
[![Halo2](https://img.shields.io/badge/halo2-proofs-blue.svg)]()
[![Status](https://img.shields.io/badge/status-experimental_AF-red.svg)]()

This repository is a conceptual implementation of a Layer-2 Zero-Knowledge scaling solution for the Matrix decentralized communication protocol.

Specifically, we are replacing the concept of **Partial Joins** with **ZK-Joins**.

## The Problem: Trust vs. Time

When a Matrix homeserver joins a federated room today, it faces a dilemma:

1. **Status Quo (Full Join):** To **trustlessly** join a room, a server must download the room's massive historical "Auth Chain" DAG and locally execute the complex State Resolution v2 algorithm from genesis. For large rooms like `#matrix:matrix.org`, this contains hundreds of thousands of events, taking an enormous amount of RAM and CPU. This takes forever.
2. **Faster Joins (MSC3902):** A temporary fix where a server *blindly trusts* the remote server's assertion of the "current state" so the user can chat immediately, while it secretly downloads the gigabytes of history and verifies it in the background. This is ultimately a compromise on decentralization.

## The Solution: Math over Computation

`zk-matrix-join` introduces a Zero-Knowledge architecture to Matrix state resolution.

By providing a succinct STARK proof alongside the current state, the joining server can verify that the state was calculated correctly from genesis in *milliseconds*.

Instead of every individual homeserver downloading 50MB of Auth Chain and running Kahn's topological sort and Ed25519 signature verification on 500,000 events, a prover node handles the heavy lifting inside a Gen-Purpose **zkVM** (like SP1). This node computes the state resolution and generates a Zero-Knowledge recursive STARK proving that the resulting state conforms exactly to Matrix protocol rules.

Standard residential servers (or even browser light clients via WebAssembly) can perform a **Trustless ZK-Join**: they download just the latest state (2MB) and a tiny STARK proof (250KB). They verify the proof instantly and participate with 100% cryptographically guaranteed trustlessness.

## Architecture

This crate is split into two primary components:

- `src/circuit/`: The heavy Prover logic. This contains the Halo2 circuits required to mathematically constrain Matrix's State Resolution v2 algorithm. (Yes, this means we are sorting SHA-256 hashes inside a finite-field arithmetic circuit. Send help).
- `src/verifier/`: The lightweight Verifier logic. This is the code that standard, low-resource homeservers will actually run to check the Sequencer's work.

## Status

Highly experimental. We are currently mapping the rules of Matrix DAG resolution to Plonkish arithmetization. If you are a cryptographer who enjoys inflicting pain upon yourself with accumulation schemes and hash-sorting constraints, PRs are welcome.

## License

Dual-licensed under MIT or Apache 2.0.
