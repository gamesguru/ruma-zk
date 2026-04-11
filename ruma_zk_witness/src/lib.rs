#![cfg_attr(feature = "guest", no_std)]
#![forbid(unsafe_code)]
#![allow(unexpected_cfgs)]

#[cfg(feature = "guest")]
extern crate alloc;

#[cfg(feature = "guest")]
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[cfg(not(feature = "guest"))]
use std::{collections::BTreeMap, string::String, vec::Vec};

use jolt::provable;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HybridEventHint {
    pub event_id: String,
    pub event_type: String,
    pub state_key: String,
    pub prev_events: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DAGMergeInput {
    pub sorted_events: Vec<HybridEventHint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DAGMergeOutput {
    pub resolved_state_hash: [u8; 32],
    pub event_count: u32,
}

#[provable(max_input_size = 1048576, max_trace_length = 1048576)]
pub fn prove_hybrid_resolution(inputs: Vec<u8>) -> DAGMergeOutput {
    let input: DAGMergeInput =
        ciborium::from_reader(inputs.as_slice()).expect("Failed to deserialize STARK inputs");
    let event_count = input.sorted_events.len() as u32;

    // 1. Hint Verification (Topological Check)
    // The host claims `sorted_events` is grouped topologically. We check it in O(N log N).
    let mut positions = BTreeMap::new();
    for (i, ev) in input.sorted_events.iter().enumerate() {
        positions.insert(ev.event_id.clone(), i);
    }

    for (i, ev) in input.sorted_events.iter().enumerate() {
        for prev_id in &ev.prev_events {
            if let Some(&prev_pos) = positions.get(prev_id) {
                if prev_pos >= i {
                    panic!("STARK HINT INVALID: Dependency violates topological order!");
                }
            }
        }
    }

    // 2. Unspoofable Resolution Map
    let mut resolved_state = BTreeMap::new();
    for ev in input.sorted_events {
        let key = (ev.event_type.clone(), ev.state_key.clone());
        resolved_state.insert(key, ev.event_id.clone());
    }

    // 3. Cryptographic Binding (SHA-256 inside the STARK constraint trace)
    let mut hasher = Sha256::new();
    for ((event_type, state_key), event_id) in resolved_state {
        hasher.update(event_type.as_bytes());
        hasher.update(state_key.as_bytes());
        hasher.update(event_id.as_bytes());
    }

    DAGMergeOutput {
        resolved_state_hash: hasher.finalize().into(),
        event_count,
    }
}
