use bus_mapping::circuit_input_builder::{ProtocolInstance, protocol_instance::{BlockEvidence, BlockMetadata}};
use eth_types::{Address, Bytes, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProofResult {
    /// The halo2 transcript
    pub proof: Bytes,
    /// Public inputs for the proof
    pub instance: Vec<String>,
    /// k of circuit parameters
    pub k: u8,
    /// Randomness used
    pub randomness: Bytes,
    /// Circuit name / identifier
    pub label: String,
    /// Auxiliary
    pub aux: ProofResultInstrumentation,
}

impl std::fmt::Debug for ProofResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Proof")
            .field("proof", &format!("{}", &self.proof))
            .field("instance", &self.instance)
            .field("k", &self.k)
            .field("randomness", &format!("{}", &self.randomness))
            .field("aux", &format!("{:#?}", self.aux))
            .finish()
    }
}

/// Timing information in milliseconds.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ProofResultInstrumentation {
    /// keygen_vk
    pub vk: u32,
    /// keygen_pk
    pub pk: u32,
    /// create_proof
    pub proof: u32,
    /// verify_proof
    pub verify: u32,
    /// MockProver.verify_par
    pub mock: u32,
    /// Circuit::new
    pub circuit: u32,
    /// RootCircuit::compile
    pub protocol: u32,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Proofs {
    /// Circuit configuration used
    pub config: CircuitConfig,
    // Proof result for circuit
    pub circuit: ProofResult,
    /// Aggregation proof for circuit, if requested
    pub aggregation: ProofResult,
    /// Gas used. Determines the upper ceiling for circuit parameters
    pub gas: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
// request extra instance corresponding to ProtocolInstance
pub struct RequestExtraInstance {
    /// l1 signal service address
    pub l1_signal_service: String,
    /// l2 signal service address
    pub l2_signal_service: String,
    /// l2 contract address
    pub l2_contract: String,
    /// meta hash
    pub meta_data: RequestMetaData,
    /// block hash value
    pub block_hash: String,
    /// the parent block hash
    pub parent_hash: String,
    /// signal root
    pub signal_root: String,
    /// extra message
    pub graffiti: String,
    /// Prover address
    pub prover: String,
    /// gas used
    pub gas_used: u32,
    /// parent gas used
    pub parent_gas_used: u32,
    /// blockMaxGasLimit
    pub block_max_gas_limit: u64,
    /// maxTransactionsPerBlock
    pub max_transactions_per_block: u64,
    /// maxBytesPerTxList
    pub max_bytes_per_tx_list: u64,
    /// anchor_gas_limit
    pub anchor_gas_limit: u64,
}

/// l1 meta hash
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestMetaData {
    /// meta id
    pub id: u64,
    /// meta timestamp
    pub timestamp: u64,
    /// l1 block height
    pub l1_height: u64,
    /// l1 block hash
    pub l1_hash: String,
    /// l1 block mix hash
    pub l1_mix_hash: String,
    /// deposits processed
    pub deposits_processed: String,
    /// tx list hash
    pub tx_list_hash: String,
    /// tx list byte start
    pub tx_list_byte_start: u32, // u24
    /// tx list byte end
    pub tx_list_byte_end: u32, // u24
    /// gas limit
    pub gas_limit: u32,
    /// beneficiary
    pub beneficiary: String,
    /// treasury
    pub treasury: String,
}

impl PartialEq for RequestExtraInstance {
    fn eq(&self, other: &Self) -> bool {
        self.l1_signal_service == other.l1_signal_service
            && self.l2_signal_service == other.l2_signal_service
            && self.l2_contract == other.l2_contract
            && self.meta_data == other.meta_data
            && self.block_hash == other.block_hash
            && self.parent_hash == other.parent_hash
            && self.signal_root == other.signal_root
            && self.graffiti == other.graffiti
            && self.prover == other.prover
            && self.gas_used == other.gas_used
            && self.parent_gas_used == other.parent_gas_used
            && self.block_max_gas_limit == other.block_max_gas_limit
            && self.max_transactions_per_block == other.max_transactions_per_block
            && self.max_bytes_per_tx_list == other.max_bytes_per_tx_list
    }
}

fn parse_hash(input: &str) -> [u8; 32] {
    H256::from_slice(&hex::decode(input).expect("parse_hash")).as_fixed_bytes().clone()
}

fn parse_address(input: &str) -> Address {
    Address::from_slice(&hex::decode(input).expect("parse_address"))
}

impl From<RequestExtraInstance> for ProtocolInstance {
    fn from(instance: RequestExtraInstance) -> Self {
        ProtocolInstance {
            block_evidence: BlockEvidence {
                blockMetadata: BlockMetadata {
                    l1Hash: parse_hash(&instance.meta_data.l1_hash).into(),            // constrain: anchor call
                    // difficulty: parse_hash(&instance.meta_data.difficult),         // constrain: l2 block's difficulty
                    txListHash: parse_hash(&instance.meta_data.tx_list_hash).into(),   // constrain: l2 txlist
                    // extraData;                                                     // constrain: l2 block's extra data
                    id: instance.meta_data.id,                                        // constrain: l2 block's number
                    timestamp:  instance.meta_data.timestamp,                         // constrain: l2 block's timestamp
                    l1Height: instance.meta_data.l1_height,                           // constrain: anchor
                    gasLimit: instance.meta_data.gas_limit,                           // constrain: l2 block's gas limit - anchor gas limit
                    coinbase:parse_address(&instance.meta_data.beneficiary).to_fixed_bytes().into(),    // constrain: L2 coinbase
                    // depositsProcessed: parse_hash(&instance.meta_data.deposits_processed).into(), // constrain: l2 withdraw root
                    ..Default::default()
                },
                parentHash: parse_hash(&instance.parent_hash).into(),
                blockHash: parse_hash(&instance.block_hash).into(),    // constrain: l2 block hash
                signalRoot:  parse_hash(&instance.signal_root).into(), // constrain: ??l2 service account storage root??
                graffiti: parse_hash(&instance.graffiti).into(),
            },
            prover: parse_address(&instance.prover),

            // l1_signal_service: parse_address(&instance.l1_signal_service),
            // l2_signal_service: parse_address(&instance.l2_signal_service),
            // l2_contract: parse_address(&instance.l2_contract),
            // meta_data: MetaData {
            //     id: instance.meta_data.id,
            //     timestamp: instance.meta_data.timestamp,
            //     l1_height: instance.meta_data.l1_height,
            //     l1_hash: parse_hash(&instance.meta_data.l1_hash),
            //     l1_mix_hash: parse_hash(&instance.meta_data.l1_mix_hash),
            //     deposits_processed: parse_hash(&instance.meta_data.deposits_processed),
            //     tx_list_hash: parse_hash(&instance.meta_data.tx_list_hash),
            //     tx_list_byte_start: instance.meta_data.tx_list_byte_start,
            //     tx_list_byte_end: instance.meta_data.tx_list_byte_end,
            //     gas_limit: instance.meta_data.gas_limit,
            //     beneficiary: parse_address(&instance.meta_data.beneficiary),
            //     treasury: parse_address(&instance.meta_data.treasury),
            // },
            // block_hash: parse_hash(&instance.block_hash),
            // parent_hash: parse_hash(&instance.parent_hash),
            // signal_root: parse_hash(&instance.signal_root),
            // graffiti: parse_hash(&instance.graffiti),
            // gas_used: instance.gas_used,
            // parent_gas_used: instance.parent_gas_used,
            // block_max_gas_limit: instance.block_max_gas_limit,
            // max_transactions_per_block: instance.max_transactions_per_block,
            // max_bytes_per_tx_list: instance.max_bytes_per_tx_list,
            // anchor_gas_limit: instance.anchor_gas_limit,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProofRequestOptions {
    /// The name of the circuit.
    /// "super", "pi"
    pub circuit: String,
    /// the block number
    pub block: u64,
    /// the l2 rpc url
    pub rpc: String,
    /// the protocol instance data
    pub protocol_instance: RequestExtraInstance,
    /// retry proof computation if error
    pub retry: bool,
    /// Parameters file or directory to use.
    /// Otherwise generates them on the fly.
    pub param: Option<String>,
    /// Only use MockProver if true.
    #[serde(default = "default_bool")]
    pub mock: bool,
    /// Additionaly aggregates the circuit proof if true
    #[serde(default = "default_bool")]
    pub aggregate: bool,
    /// Runs the MockProver if proofing fails.
    #[serde(default = "default_bool")]
    pub mock_feedback: bool,
    /// Verifies the proof after computation.
    #[serde(default = "default_bool")]
    pub verify_proof: bool,
}

impl PartialEq for ProofRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.block == other.block
            && self.protocol_instance == other.protocol_instance
            && self.rpc == other.rpc
            && self.param == other.param
            && self.circuit == other.circuit
            && self.mock == other.mock
            && self.aggregate == other.aggregate
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRequest {
    pub options: ProofRequestOptions,
    pub result: Option<Result<Proofs, String>>,
    /// A counter to keep track of changes of the `result` field
    pub edition: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInformation {
    pub id: String,
    pub tasks: Vec<ProofRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatus {
    pub id: String,
    /// The current active task this instance wants to obtain or is working on.
    pub task: Option<ProofRequestOptions>,
    /// `true` if this instance started working on `task`
    pub obtained: bool,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct CircuitConfig {
    pub block_gas_limit: usize,
    pub max_txs: usize,
    pub max_calldata: usize,
    pub max_bytecode: usize,
    pub max_rws: usize,
    pub max_copy_rows: usize,
    pub max_exp_steps: usize,
    pub min_k: usize,
    pub pad_to: usize,
    pub min_k_aggregation: usize,
    pub keccak_padding: usize,
}

fn default_bool() -> bool {
    false
}
