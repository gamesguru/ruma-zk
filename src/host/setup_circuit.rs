// Copyright 2026 Shane Jaroch
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sp1_sdk::blocking::{Prover, ProverClient};
use sp1_sdk::{HashableKey, ProvingKey};

pub const ZK_MATRIX_GUEST_ELF: &[u8] = include_bytes!(env!("SP1_ELF_zk-matrix-join-guest"));

fn main() {
    println!("> [Setup Tool] Initializing SP1 Circuit Compilation...");
    println!("> This will generate the Proving Key (PK) for the Guest ELF and VM Configuration (DIM=10).");
    println!("> Once cached in ~/.sp1/circuits/, this setup works for ANY number of events.");
    println!("> For a complex program like Ruma State Res, this one-time math takes 15-30 mins on a CPU.");

    let prover_client = ProverClient::from_env();

    println!("> Compiling guest ELF and building universal constraints...");
    let pk = prover_client
        .setup(sp1_sdk::Elf::Static(ZK_MATRIX_GUEST_ELF))
        .unwrap();

    println!("> Serializing and saving Proving Key to res/pk.bin...");
    let pk_bytes = bincode::serialize(&pk).expect("Failed to serialize PK");
    std::fs::write("res/pk.bin", pk_bytes).expect("Failed to write res/pk.bin");

    println!("✓ Setup Complete!");
    println!("✓ Verifying Key Hash: {}", pk.verifying_key().bytes32());
    println!("✓ Your main application will now start instantly.");
}
