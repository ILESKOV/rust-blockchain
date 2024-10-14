// src/zk_proofs.rs

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use blstrs::{Bls12, Scalar as Fr};
use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    VerifyingKey,
};
use rand_core::OsRng; // Use rand_core's OsRng
use serde::{Deserialize, Serialize};
use std::io::Cursor;

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
pub struct ProofData {
    pub proof: Vec<u8>,
    pub vk: Vec<u8>,
}

pub fn generate_transaction_proof(amount: u64) -> ProofData {
    let mut rng = OsRng; // Initialize OsRng from rand_core
    let amount_fr = Fr::from(amount);

    // Generate parameters
    let params = {
        let circuit = TransactionProof { amount: None };
        generate_random_parameters::<Bls12, _, _>(circuit, &mut rng).unwrap()
    };

    // Create an instance of the circuit with the actual amount
    let circuit = TransactionProof {
        amount: Some(amount_fr),
    };

    // Create a proof
    let proof = create_random_proof(circuit, &params, &mut rng).unwrap();

    // Serialize the proof and vk using write methods
    let mut proof_bytes = vec![];
    proof.write(&mut proof_bytes).unwrap();

    let mut vk_bytes = vec![];
    params.vk.write(&mut vk_bytes).unwrap();

    ProofData {
        proof: proof_bytes,
        vk: vk_bytes,
    }
}

pub fn verify_transaction_proof(proof_data: &ProofData) -> bool {
    // Deserialize the proof and vk using read methods
    let mut proof_cursor = Cursor::new(&proof_data.proof);
    let proof = match Proof::<Bls12>::read(&mut proof_cursor) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let mut vk_cursor = Cursor::new(&proof_data.vk);
    let vk = match VerifyingKey::<Bls12>::read(&mut vk_cursor) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let pvk = prepare_verifying_key(&vk);

    // No public inputs in this example
    let public_inputs = [];

    verify_proof(&pvk, &proof, &public_inputs).is_ok()
}
