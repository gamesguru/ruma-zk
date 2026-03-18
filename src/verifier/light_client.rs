use crate::circuit::recursive::DagMergeCircuit;
use halo2_proofs::{
    plonk::{keygen_vk, verify_proof, VerificationStrategy},
    poly::commitment::Params,
    transcript::{Challenge255, TranscriptRead},
};
use pasta_curves::{vesta::Affine as G1Affine, Fp as Fr}; // <-- Using the Vesta Affine point

/// The lightweight verification function that a joining homeserver runs.
pub fn verify_zk_join<'params, V, T>(
    params: &'params Params<G1Affine>,
    _room_vk_bytes: &[u8],
    latest_state_hash: Fr,
    strategy: V,
    mut transcript: T,
) -> Result<bool, &'static str>
where
    V: VerificationStrategy<'params, G1Affine>,
    T: TranscriptRead<G1Affine, Challenge255<G1Affine>>,
{
    let empty_circuit = DagMergeCircuit::default();

    // Generate the Verification Key (VK)
    let vk = keygen_vk(params, &empty_circuit).map_err(|_| "Failed to load VK")?;

    // The state we are checking against (Fixed the 3D array signature)
    let instances: &[&[&[Fr]]] = &[&[&[latest_state_hash]]];

    // Verify the proof!
    let result = verify_proof(params, &vk, strategy, instances, &mut transcript);

    Ok(result.is_ok())
}
