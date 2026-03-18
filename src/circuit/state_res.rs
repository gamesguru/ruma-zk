use halo2_proofs::{
    circuit::{AssignedCell, Layouter},
    plonk::{Advice, Column, ConstraintSystem, Error, Selector, TableColumn, VirtualCells},
    poly::Rotation,
};
use pasta_curves::Fp as Fr; // <-- Using the Pallas scalar field

#[derive(Clone)]
pub struct StateResConfig {
    pub q_sort: Selector,
    pub q_hash: Selector,
    pub hash_input: Column<Advice>,
    pub sorted_output: Column<Advice>,
    pub u8_table: TableColumn,
}

pub struct StateResChip {
    pub config: StateResConfig,
}

impl StateResChip {
    pub fn construct(config: StateResConfig) -> Self {
        Self { config }
    }

    pub fn configure(meta: &mut ConstraintSystem<Fr>) -> StateResConfig {
        let q_sort = meta.complex_selector();
        let q_hash = meta.selector();
        let hash_input = meta.advice_column();
        let sorted_output = meta.advice_column();

        let u8_table = meta.lookup_table_column();

        // Fixed the strict closure signature for Halo2 v0.3.0
        meta.lookup(|meta: &mut VirtualCells<'_, Fr>| {
            let s = meta.query_selector(q_sort);
            let limb_difference = meta.query_advice(sorted_output, Rotation::cur());

            vec![(s * limb_difference, u8_table)]
        });

        StateResConfig {
            q_sort,
            q_hash,
            hash_input,
            sorted_output,
            u8_table,
        }
    }

    pub fn resolve_state(
        &self,
        mut layouter: impl Layouter<Fr>,
        conflicting_states: Vec<AssignedCell<Fr, Fr>>,
    ) -> Result<AssignedCell<Fr, Fr>, Error> {
        layouter.assign_region(
            || "StateResV2 Tie Breaker",
            |_region| Ok(conflicting_states[0].clone()),
        )
    }
}
