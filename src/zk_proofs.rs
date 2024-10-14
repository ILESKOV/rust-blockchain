// src/zk_proofs.rs

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bls12_381::{Bls12, Scalar};
use ff::Field;
use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use rand::rngs::OsRng;
use bincode;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct TransactionProof {
    pub amount: Option<Scalar>,
}

impl Circuit<Scalar> for TransactionProof {
    fn synthesize<CS: ConstraintSystem<Scalar>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        // Allocate the private input (amount)
        let amount_value = self.amount;
        let amount_allocated = cs.alloc(|| "amount", || amount_value.ok_or(SynthesisError::AssignmentMissing))?;

        // Example constraint: amount > 0
        // Since we're working in a finite field, we can't directly enforce "greater than"
        // Instead, we'll skip this and focus on a valid constraint for demonstration

        // For example, we can enforce that amount * 1 = amount
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
    let amount_scalar = Scalar::from(amount);

    // Generate parameters
    let params = {
        let circuit = TransactionProof { amount: None };
        generate_random_parameters::<Bls12, _, _>(circuit, &mut rng).unwrap()
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    // Create an instance of the circuit (with the actual amount)
    let circuit = TransactionProof {
        amount: Some(amount_scalar),
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

pub fn verify_transaction_proof(proof: &Proof) -> bool {
    // Deserialize the proof and vk
    let proof = bincode::deserialize(&proof.proof).unwrap();
    let pvk = bincode::deserialize(&proof.vk).unwrap();

    // No public inputs in this example
    let public_inputs = [];

    verify_proof(&pvk, &proof, &public_inputs).is_ok()
}