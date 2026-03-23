#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuestStateEvent {
    pub event_id_hash: [u8; 32],
    pub sender_pubkey: [u8; 32],
    pub power_level: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DAGMergeInput {
    pub room_version: u32,
    pub sorted_conflicts: Vec<GuestStateEvent>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DAGMergeOutput {
    pub resolved_state_hash: [u8; 32],
}

pub fn main() {
    // Read the input from the Host (the pre-sorted state events)
    let input: DAGMergeInput = sp1_zkvm::io::read();

    let mut hasher = Sha256::new();
    let mut prev_hash = [0u8; 32];

    for event in input.sorted_conflicts {
        // Enforce protocol rules: user must have power level >= 0
        if event.power_level < 0 {
            panic!("Invalid Power Level detected in Auth Chain! ZK Proof Generation Failed.");
        }

        // O(N) Hint Verification: Ensure the Host supplied a correctly sorted array.
        // Verifying a sort is vastly cheaper than performing one inside the VM!
        if event.event_id_hash < prev_hash {
            panic!("Host provided an unsorted DAG! Hint verification failed.");
        }
        prev_hash = event.event_id_hash;

        // Custom Inline ASM Optimization for critical loop bottlenecks
        // Matrix relies heavily on 256-bit (32-byte) arrays for hashes and Ed25519.
        // Here we demonstrate a highly optimized unrolled RISC-V 32-bit inline assembly block.
        // This is exactly how we squeeze cycles out of standard Rust code for ZK constraints.
        let mut optimized_state_hash = [0u8; 32];
        #[cfg(target_arch = "riscv32")]
        unsafe {
            let src = event.event_id_hash.as_ptr() as *const u32;
            let dst = optimized_state_hash.as_mut_ptr() as *mut u32;
            // Unroll 8 * 32-bit word copies (32 bytes = 256 bits)
            core::arch::asm!(
                "lw t0, 0({0})",
                "sw t0, 0({1})",
                "lw t1, 4({0})",
                "sw t1, 4({1})",
                "lw t2, 8({0})",
                "sw t2, 8({1})",
                "lw t3, 12({0})",
                "sw t3, 12({1})",
                "lw t4, 16({0})",
                "sw t4, 16({1})",
                "lw t5, 20({0})",
                "sw t5, 20({1})",
                "lw t6, 24({0})",
                "sw t6, 24({1})",
                "lw a0, 28({0})",
                "sw a0, 28({1})",
                in(reg) src,
                in(reg) dst,
                out("t0") _, out("t1") _, out("t2") _, out("t3") _, out("t4") _, out("t5") _, out("t6") _, out("a0") _
            );
        }
        #[cfg(not(target_arch = "riscv32"))]
        {
            optimized_state_hash.copy_from_slice(&event.event_id_hash);
        }

        // Only hash the minimal necessary binary data, never parse raw JSON
        hasher.update(&optimized_state_hash);
        hasher.update(&event.sender_pubkey);
        hasher.update(&event.power_level.to_le_bytes());
    }

    let expected_hash: [u8; 32] = hasher.finalize().into();

    let output = DAGMergeOutput {
        resolved_state_hash: expected_hash,
    };

    sp1_zkvm::io::commit(&output);
}
