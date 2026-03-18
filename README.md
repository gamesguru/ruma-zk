# ZK-Matrix-Join: Trustless Light Clients for the Matrix Protocol

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)]()
[![Halo2](https://img.shields.io/badge/halo2-proofs-blue.svg)]()
[![Status](https://img.shields.io/badge/status-experimental_AF-red.svg)]()

This repository is a conceptual implementation of a Layer-2 Zero-Knowledge scaling solution for the Matrix decentralized communication protocol.

Specifically, we are replacing the concept of **Partial Joins** with **ZK-Joins**.

## The Problem: Trust vs. Time

When a Matrix homeserver joins a federated room today, it faces a dilemma:

1. **Full Join:** Download the entire multi-gigabyte historical Directed Acyclic Graph (DAG) of the room and compute the state from the genesis event. This takes forever.
2. **Partial Join:** Ask a peer server for the current state and trust that they aren't lying. This is fast, but breaks the "don't trust, verify" ethos of decentralization.

## The Solution: Math over Computation

`zk-matrix-join` introduces a ZK-Rollup architecture to Matrix.

Instead of every homeserver re-calculating the DAG merges and running State Resolution v2, heavily provisioned "Sequencer" nodes handle the heavy lifting. They compute the state resolution and generate a Zero-Knowledge recursive SNARK proving that the resulting state is mathematically correct according to the protocol rules.

Standard homeservers can then perform a **ZK-Join**: they download the latest state and a tiny cryptographic proof. They verify the proof in O(1) time (milliseconds) and instantly participate in the room with absolute mathematical certainty that the state is valid.

## Architecture

This crate is split into two primary components:

- `src/circuit/`: The heavy Prover logic. This contains the Halo2 circuits required to mathematically constrain Matrix's State Resolution v2 algorithm. (Yes, this means we are sorting SHA-256 hashes inside a finite-field arithmetic circuit. Send help).
- `src/verifier/`: The lightweight Verifier logic. This is the code that standard, low-resource homeservers will actually run to check the Sequencer's work.

## Status

Highly experimental. We are currently mapping the rules of Matrix DAG resolution to Plonkish arithmetization. If you are a cryptographer who enjoys inflicting pain upon yourself with accumulation schemes and hash-sorting constraints, PRs are welcome.

## License

Dual-licensed under MIT or Apache 2.0.
