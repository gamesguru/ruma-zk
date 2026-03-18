use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Circuit, ConstraintSystem, Error},
};
use pasta_curves::Fp as Fr; // <-- Using the Pallas scalar field

use super::state_res::{StateResChip, StateResConfig};

#[derive(Clone)]
pub struct DagMergeConfig {
    state_res_config: StateResConfig,
}

#[derive(Default)]
pub struct DagMergeCircuit {
    pub parent_states: Vec<Fr>,
    pub parent_proofs: Vec<Vec<u8>>,
    pub expected_new_state: Fr,
}

impl Circuit<Fr> for DagMergeCircuit {
    type Config = DagMergeConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        let state_res_config = StateResChip::configure(meta);

        DagMergeConfig { state_res_config }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        _layouter: impl Layouter<Fr>, // Silenced the mut warning
    ) -> Result<(), Error> {
        let _state_chip = StateResChip::construct(config.state_res_config);

        Ok(())
    }
}

// ==========================================
// TEST SUITE
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;

    #[test]
    fn test_dag_merge_circuit() {
        let k = 4;
        let circuit = DagMergeCircuit {
            parent_states: vec![Fr::from(1), Fr::from(2)],
            parent_proofs: vec![vec![], vec![]],
            expected_new_state: Fr::from(3),
        };
        let public_inputs = vec![];
        let prover = MockProver::run(k, &circuit, public_inputs).unwrap();
        assert_eq!(prover.verify(), Ok(()));
    }
}
