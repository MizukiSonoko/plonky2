use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

/// An example of using Plonky2 to prove a statement of the form
/// "I'm 27 age, I am over 18 years old."
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    let age_t = builder.add_virtual_target();
    let thresh_hold_t = builder.constant(F::from_canonical_u64(18));
    let num_bits: usize = 128;

    // Public inputs are the one thresh_hold_t.
    builder.register_public_input(thresh_hold_t);

    let result_t = builder.is_greater_than(age_t, thresh_hold_t, num_bits);
    builder.assert_one(result_t.target);

    // Provide secret age.
    let mut pw = PartialWitness::new();
    pw.set_target(age_t, F::from_canonical_u64(27));

    let data = builder.build::<C>();
    let proof = data.prove(pw)?;

    println!("I am over {} years old.", proof.public_inputs[0]);

    data.verify(proof)
}
