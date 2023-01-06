use crate::circuit_witness::CircuitWitness;
use halo2_proofs::halo2curves::bn256::Fr;
use rand::Rng;
use zkevm_circuits::bytecode_circuit::bytecode_unroller::BytecodeCircuit;
use zkevm_circuits::copy_circuit::CopyCircuit;
use zkevm_circuits::evm_circuit::EvmCircuit;
use zkevm_circuits::exp_circuit::ExpCircuit;
use zkevm_circuits::keccak_circuit::keccak_packed_multi::KeccakCircuit;
use zkevm_circuits::pi_circuit::PiCircuit;
use zkevm_circuits::pi_circuit::PiTestCircuit;
use zkevm_circuits::state_circuit::StateCircuit;
use zkevm_circuits::super_circuit::SuperCircuit;
use zkevm_circuits::tx_circuit::TxCircuit;
use zkevm_circuits::util::SubCircuit;

/// Returns a instance of the `SuperCircuit`.
pub fn gen_super_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<SuperCircuit<Fr, MAX_TXS, MAX_CALLDATA, MAX_RWS>, String> {
    let block = witness.evm_witness();

    let evm_circuit = EvmCircuit::new_from_block(&block);
    let state_circuit = StateCircuit::new_from_block(&block);
    let tx_circuit = TxCircuit::new_from_block(&block);
    let pi_circuit = PiCircuit::new_from_block(&block);
    let bytecode_circuit = BytecodeCircuit::new_from_block(&block);
    let copy_circuit = CopyCircuit::new_from_block(&block);
    let exp_circuit = ExpCircuit::new_from_block(&block);
    let keccak_circuit = KeccakCircuit::new_from_block(&block);
    let circuit = SuperCircuit::<_, MAX_TXS, MAX_CALLDATA, MAX_RWS> {
        evm_circuit,
        state_circuit,
        tx_circuit,
        pi_circuit,
        bytecode_circuit,
        copy_circuit,
        exp_circuit,
        keccak_circuit,
    };

    Ok(circuit)
}

pub trait InstancesExport {
    fn num_instance() -> Vec<usize>;

    fn instances(&self) -> Vec<Vec<Fr>>;
}

impl<const MAX_TXS: usize, const MAX_CALLDATA: usize> InstancesExport
    for PiTestCircuit<Fr, MAX_TXS, MAX_CALLDATA>
{
    fn num_instance() -> Vec<usize> {
        vec![5]
    }

    fn instances(&self) -> Vec<Vec<Fr>> {
        // vec![vec![self.0]]
        self.0.instance()
    }
}

/// Returns a instance of the `PiTestCircuit`.
pub fn gen_pi_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<PiTestCircuit<Fr, MAX_TXS, MAX_CALLDATA>, String> {
    let block = witness.evm_witness();
    let circuit = PiTestCircuit::<Fr, MAX_TXS, MAX_CALLDATA>(PiCircuit::new_from_block(&block));

    Ok(circuit)
}

/// Returns a instance of the `EvmCircuit`.
pub fn gen_evm_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<EvmCircuit<Fr>, String> {
    let block = witness.evm_witness();
    Ok(EvmCircuit::new_from_block(&block))
}

/// Returns a instance of the `StateCircuit`.
pub fn gen_state_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<StateCircuit<Fr>, String> {
    let block = witness.evm_witness();
    Ok(StateCircuit::new_from_block(&block))
}

/// Returns a instance of the `TxCircuit`.
pub fn gen_tx_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<TxCircuit<Fr>, String> {
    let block = witness.evm_witness();
    Ok(TxCircuit::new_from_block(&block))
}

/// Returns a instance of the `BytecodeCircuit`.
pub fn gen_bytecode_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<BytecodeCircuit<Fr>, String> {
    let block = witness.evm_witness();
    Ok(BytecodeCircuit::new_from_block(&block))
}

/// Returns a instance of the `CopyCircuit`.
pub fn gen_copy_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<CopyCircuit<Fr>, String> {
    let block = witness.evm_witness();
    Ok(CopyCircuit::new_from_block(&block))
}

/// Returns a instance of the `ExpCircuit`.
pub fn gen_exp_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<ExpCircuit<Fr>, String> {
    let block = witness.evm_witness();
    Ok(ExpCircuit::new_from_block(&block))
}

/// Returns a instance of the `KeccakCircuit`.
pub fn gen_keccak_circuit<
    const MAX_TXS: usize,
    const MAX_CALLDATA: usize,
    const MAX_RWS: usize,
    RNG: Rng,
>(
    witness: &CircuitWitness,
    mut _rng: RNG,
) -> Result<KeccakCircuit<Fr>, String> {
    let block = witness.evm_witness();
    Ok(KeccakCircuit::new_from_block(&block))
}
