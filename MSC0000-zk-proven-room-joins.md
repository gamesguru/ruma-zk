# MSC0000: Trustless ZK-STARK Federated Room Joins

**Author:** [@gamesguru]
**Created:** [Sun 23 Mar 2026]
**Status:** Draft

## 1. Introduction and Motivation

The Matrix protocol is built on a fully decentralized "don't trust, verify" architecture. Currently, when a homeserver joins a federated room, it has two theoretical paths:

1. **Full Join (Status Quo):** Download the entire multi-gigabyte historical Directed Acyclic Graph (DAG) known as the "Auth Chain" and locally execute the State Resolution v2 algorithm from the genesis event. While this guarantees trustlessness, it is computationally prohibitive and can take seconds or minutes (or longer) for massive rooms.
2. **Faster [Partial] Joins (MSC3902):** The server temporarily _blindly trusts_ the remote server's assertion of the "current state" so users can participate immediately. In the background, it syncs the multi-gigabyte Auth Chain. This compromises the immediate trustless nature of the network.

**The Solution:** This MSC proposes introducing Zero-Knowledge proofs (specifically compressed Groth16/Plonk SNARKs wrapping RISC-V STARKs) to securely prove State Resolution v2 execution. A prover node calculates the state resolution and outputs a tiny, `~300 byte` verifiable SNARK proof. The joining server merely downloads the current state map and this proof—verifying mathematical correctness in milliseconds without downloading the historical DAG.

## 2. Proposed Endpoints

We propose a new versioned endpoint under the Federation API designed specifically for ZK-Joins.

`GET /_matrix/federation/v3/zk_state_proof/{roomId}`

**Request Parameters:**

- `roomId`: The ID of the room to join.

**Response payload:**

```json
{
  "room_version": "12",
  "checkpoint": {
    "event_id": "$historic_event_X",
    "resolved_state_root_hash": "<sha256_hash>",
    "zk_proof": "<base64_growth16_snark>",
    "image_id": "<sp1_vkey_hash>"
  },
  "delta": {
    "recent_state_events": [
      { /* Raw State Event 1 (happened today) */ },
      { /* Raw State Event 2 (happened today) */ }
    ]
  }
}
```

## 3. Program Consensus & Image IDs

If a joining server receives a zero-knowledge proof, it must know exactly which circuit program verified the logic. Otherwise, a malicious host could write a circuit that simply returns `true` and bypasses power levels.

**Rule:** Matrix Room Versions MUST dictate the allowed zkVM program.
*When joining Room Version 12 via ZK-Proof, the receiving server MUST assert that the SNARK receipt's `image_id` perfectly matches the protocol-defined canonical Hash of the official SP1 Guest ELF for Room Version 12.*

## 4. Epochs and Asynchronous Proving

To prevent unacceptable CPU load, homeservers MUST NOT generate ZK proofs synchronously during a federation or client join request.

Proving nodes SHALL periodically generate "Checkpoint Proofs" for a given Room's state asynchronously in the background. When serving a `/zk_state_proof` request, the resident server returns the most recent Checkpoint Proof alongside the minimal, unproven Auth Chain delta that has accumulated since that checkpoint.

The joining server cryptographically verifies the Checkpoint in `O(1)` time, and natively resolves the tiny unproven delta in trivial `O(N)` time. This "Hybrid Verification" guarantees 100% trustless state while allowing proof generation to happen entirely offline.

## 5. The Light Client Angle

A crucial secondary benefit of migrating complex state resolution to a SNARK proof is that verifiers are extremely lightweight. The SP1 Growth16 verifier (`ark-bn254` based) is compiled entirely to WebAssembly (WASM).

This allows clients like **Element Web** or mobile browsers to verify the state of a room _trustlessly_ in 10-15 milliseconds. A client no longer has to trust that its connected Homeserver isn't lying about the room state—it can verify the ZK-Proof directly on the edge device, shifting Matrix closer to a true peer-to-peer paradigm.
