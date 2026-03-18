//! ZK-Matrix-Join: A Zero-Knowledge scaling solution for the Matrix protocol.
//!
//! This crate provides the cryptographic circuits for proving Matrix State Resolution v2
//! over a DAG, as well as the lightweight verification logic for homeservers.

pub mod circuit;
pub mod verifier;
