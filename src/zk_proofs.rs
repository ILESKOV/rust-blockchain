// src/zk_proofs.rs

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use blstrs::{Bls12, Scalar as Fr};
use ff::PrimeField;
use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use bincode;

#[derive(Clone)]
pub struct TransactionProof {
    pub amount: Option<Fr>,
}

impl Circuit<Fr> for TransactionProof {
    fn synthesize<CS: ConstraintSystem<Fr>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        // Allocate the private input (amount)
        let amount_value = self.amount;
        let amount_allocated = cs.alloc(
            || "amount",
            || amount_value.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // Enforce that amount * 1 = amount (simple constraint)
        cs.enforce(
            || "amount consistency",
            |lc| lc + amount_allocated,
            |lc| lc + CS::one(),
            |lc| lc + amount_allocated,
        );

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proof {
    pub proof: Vec<u8>,
    pub vk: Vec<u8>,
}

pub fn generate_transaction_proof(amount: u64) -> Proof {
    let mut rng = OsRng;
    let amount_fr = Fr::from(amount);

    // Generate parameters
    let params = {
        let circuit = TransactionProof { amount: None };
        generate_random_parameters::<Bls12, _, _>(circuit, &mut rng).unwrap()
    };

    // Prepare the verification key
    let pvk = prepare_verifying_key(&params.vk);

    // Create an instance of the circuit with the actual amount
    let circuit = TransactionProof {
        amount: Some(amount_fr),
    };

    // Create a proof
    let proof = create_random_proof(circuit, &params, &mut rng).unwrap();

    // Serialize the proof and vk
    let proof_bytes = bincode::serialize(&proof).unwrap();
    let vk_bytes = bincode::serialize(&pvk).unwrap();

    Proof {
        proof: proof_bytes,
        vk: vk_bytes,
    }
}

pub fn verify_transaction_proof(proof_data: &Proof) -> bool {
    // Deserialize the proof and vk
    let proof: bellman::groth16::Proof<Bls12> = bincode::deserialize(&proof_data.proof).unwrap();
    let pvk: bellman::groth16::PreparedVerifyingKey<Bls12> = bincode::deserialize(&proof_data.vk).unwrap();

    // No public inputs in this example
    let public_inputs = [];

    verify_proof(&pvk, &proof, &public_inputs).is_ok()
}