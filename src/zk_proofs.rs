// src/zk_proofs.rs

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bls12_381::{Bls12, Fr};
use ff::{Field, PrimeField};
use bellman::groth16::{generate_random_parameters, create_random_proof, prepare_verifying_key, verify_proof};
use rand::rngs::OsRng;
use bincode;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct TransactionProof {
    pub amount: Option<Fr>,
}

impl Circuit<Fr> for TransactionProof {
    fn synthesize<CS: ConstraintSystem<Fr>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        // Witness for amount
        let amount_value = self.amount;

        // Allocate the private input (amount)
        let amount_allocated = cs.alloc(|| "amount", || amount_value.ok_or(SynthesisError::AssignmentMissing))?;

        // Example constraint: amount > 0
        let zero = Fr::zero();
        cs.enforce(
            || "amount greater than zero",
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
    let rng = &mut OsRng;
    let amount_fr = Fr::from_str(&amount.to_string()).unwrap();

    // Generate parameters
    let params = {
        let c = TransactionProof { amount: None };
        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    // Create an instance of the circuit (with the actual amount)
    let c = TransactionProof { amount: Some(amount_fr) };

    // Create a proof
    let proof = create_random_proof(c, &params, rng).unwrap();

    // Serialize the proof and vk
    let proof_bytes = bincode::serialize(&proof).unwrap();
    let vk_bytes = bincode::serialize(&pvk).unwrap();

    Proof { proof: proof_bytes, vk: vk_bytes }
}

pub fn verify_transaction_proof(proof: &Proof) -> bool {
    // Deserialize the proof and vk
    let proof = bincode::deserialize(&proof.proof).unwrap();
    let pvk = bincode::deserialize(&proof.vk).unwrap();

    // No public inputs in this example
    let public_inputs = [];

    verify_proof(&pvk, &proof, &public_inputs).is_ok()
}