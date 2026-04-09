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
pub const ZK_MATRIX_GUEST_UNOPTIMIZED_ELF: &[u8] =
    include_bytes!(env!("SP1_ELF_zk-matrix-join-guest-unoptimized"));

fn main() {
    let is_unoptimized = std::env::var("EXECUTE_UNOPTIMIZED").is_ok();
    let target_elf = if is_unoptimized {
        ZK_MATRIX_GUEST_UNOPTIMIZED_ELF
    } else {
        ZK_MATRIX_GUEST_ELF
    };

    println!("> [Setup Tool] Initializing SP1 Circuit Compilation...");
    if is_unoptimized {
        println!("> MODE: UNOPTIMIZED (Full Spec State Resolution)");
    } else {
        let dim = option_env!("SP1_TOPOLOGY_DIM").unwrap_or("10");
        println!("> MODE: OPTIMIZED (Topological Reducer, DIM={})", dim);
    }

    let prover_client = ProverClient::from_env();

    println!("> Compiling guest ELF and building universal constraints...");
    let pk = prover_client
        .setup(sp1_sdk::Elf::Static(target_elf))
        .unwrap();

    let dim = if is_unoptimized {
        "full_spec".to_string()
    } else {
        option_env!("SP1_TOPOLOGY_DIM").unwrap_or("10").to_string()
    };
    let pk_filename = format!("res/pk_{}.bin", dim);

    println!("> Serializing and saving Proving Key to {}...", pk_filename);
    let pk_bytes = bincode::serialize(&pk).expect("Failed to serialize PK");
    std::fs::write(&pk_filename, pk_bytes).expect("Failed to write PK bin");

    println!("✓ Setup Complete!");
    println!("✓ Verifying Key Hash: {}", pk.verifying_key().bytes32());
    println!("✓ Your main application will now start instantly.");
}
