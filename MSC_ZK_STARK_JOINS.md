# MSCXXXX: Trustless ZK-STARK Federated Room Joins

**Author:** [Your Name / Matrix ID]
**Created:** [Current Date]
**Status:** WIP / Concept

## 1. Introduction and Motivation

The Matrix protocol is built on a fully decentralized "don't trust, verify" architecture. Currently, when a homeserver joins a federated room, it has two theoretical paths:

1.  **Full Join (Status Quo):** Download the entire multi-gigabyte historical Directed Acyclic Graph (DAG) known as the "Auth Chain" and locally execute the State Resolution v2 algorithm from the genesis event. While this guarantees trustlessness, it is computationally prohibitive and can take seconds or minutes (or longer) for massive rooms.
2.  **Faster Joins (MSC3902):** The server temporarily _blindly trusts_ the remote server's assertion of the "current state" so users can participate immediately. In the background, it syncs the multi-gigabyte Auth Chain. This compromises the immediate trustless nature of the network.

**The Solution:** This MSC proposes introducing Zero-Knowledge STARKs (Scalable Transparent ARguments of Knowledge) to securely prove State Resolution v2 execution. A prover node calculates the state resolution and outputs a tiny, O(1) verifiable STARK proof. The joining server merely downloads the current state map and this proof—verifying mathematical correctness in milliseconds without downloading the historical DAG.

## 2. Proposed Endpoints

We propose a new versioned endpoint under the Federation API designed specifically for ZK-Joins.

`GET /_matrix/federation/v3/zk_state_proof/{roomId}`

**Request Parameters:**

- `roomId`: The ID of the room to join.

**Response payload:**

```json
{
  "room_version": "10",
  "state": [
    {
      /* Array of current state events */
    }
  ],
  "zk_proof": {
    "system": "sp1_riscv",
    "stark_payload": "<base64_encoded_stark_proof>",
    "public_values": {
      "resolved_state_root_hash": "<sha256_hash>"
    }
  }
}
```

The joining homeserver will hash the provided `state` array and assert it matches the `resolved_state_root_hash` provided in the STARK `public_values`. It then executes the STARK verifier against the `stark_payload`. If valid, the state is 100% mathematically sound.

## 3. Benchmarks (Conceptual)

The delta between downloading an Auth Chain versus a ZK-STARK proof is orders of magnitude different:

| Metric                    | Traditional Full Join         | Trustless ZK-Join              |
| :------------------------ | :---------------------------- | :----------------------------- |
| **Data Transfer**         | ~50 MB (Auth Chain)           | ~2 MB (State) + 250 KB (Proof) |
| **CPU Verification Time** | ~10 Seconds (Sorting/Ed25519) | ~50 Milliseconds               |
| **Trust Model**           | 100% Trustless                | 100% Trustless                 |

## 4. The Light Client Angle

A crucial secondary benefit of migrating complex state resolution to a STARK proof is that verifiers are extremely lightweight. The SP1 STARK verifier can be compiled entirely to WebAssembly (WASM).

This allows clients like **Element Web** or mobile browsers to verify the state of a room _trustlessly_. A client no longer has to trust that its connected Homeserver isn't lying about the room state—it can verify the ZK-Proof directly on the edge device, shifting Matrix closer to a true peer-to-peer paradigm.

## 5. Implementation Details

Reference implementation utilizing the `sp1-zkvm` framework for the Rust `ruma-state-res` logic can be found in the associated demo repository.
