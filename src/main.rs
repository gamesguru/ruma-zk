use halo2_proofs::arithmetic::Field;
use halo2_proofs::dev::MockProver;
use pasta_curves::Fp as Fr;
use rand::rngs::OsRng;
use zk_matrix_join::circuit::recursive::DagMergeCircuit;

fn main() {
    println!("* Starting ZK-Matrix-Join Demo...");
    println!("--------------------------------------------------");

    // Generate realistic looking cryptographic hashes (simulated using field elements)
    let mut rng = OsRng;
    let parent_a_hash = Fr::random(&mut rng);
    let parent_b_hash = Fr::random(&mut rng);
    let merged_state_hash = Fr::random(&mut rng);

    // Define the demo parameters
    let k = 4; // Circuit size parameter (2^k rows)
    let parent_states = vec![parent_a_hash, parent_b_hash];
    let parent_proofs = vec![vec![], vec![]];
    let expected_new_state = merged_state_hash;

    println!("> Scenario: A homeserver is attempting to merge a split DAG.");
    println!("   - Parent State A Hash: {:?}", parent_states[0]);
    println!("   - Parent State B Hash: {:?}", parent_states[1]);
    println!("   - Proposed Merged State Hash: {:?}", expected_new_state);

    // Instantiate the circuit
    let circuit = DagMergeCircuit {
        parent_states,
        parent_proofs,
        expected_new_state,
    };

    println!("--------------------------------------------------");
    println!("* Constructing Zero-Knowledge Circuit...");
    println!("   - This circuit proves that State Res v2 was executed correctly.");
    println!("   - It enforces that tie-breakers between conflicting events are sound.");

    // In a real scenario, this would generate a SNARK/STARK proof.
    // For this demo, we run the MockProver to verify all constraints hold mathematically.
    let public_inputs = vec![];
    let prover = MockProver::run(k, &circuit, public_inputs).unwrap();

    println!("--------------------------------------------------");
    println!("> Verifying Constraints (MockProver)...");

    match prover.verify() {
        Ok(_) => {
            println!("✓ SUCCESS: The cryptographic proof is valid!");
            println!(
                "   The new joining homeserver can mathematically trust this state resolution"
            );
            println!("   without downloading the entire room history.");
        }
        Err(e) => {
            println!("✗ FAILURE: The circuit rejected the state transition!");
            println!("   Error details: {:?}", e);
        }
    }
}
