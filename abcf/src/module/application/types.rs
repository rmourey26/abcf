use alloc::{string::String, vec::Vec};
use tm_protos::abci::{ConsensusParams, ValidatorUpdate};

/// Response for deliver_tx.
#[derive(Debug, Default)]
pub struct ResponseDeliverTx {
    /// Result code.
    pub code: u32,
    /// Result data.
    pub data: Vec<u8>,
    /// Amount of gas requested for transaction.
    pub gas_wanted: i64,
    /// Amount of gas consumed for transaction.
    pub gas_used: i64,
}

/// Response for check_tx
#[derive(Debug, Default)]
pub struct ResponseCheckTx {
    /// Result code.
    pub code: u32,
    /// Result data.
    pub data: Vec<u8>,
    /// Amount of gas requested for transaction.
    pub gas_wanted: i64,
    /// Amount of gas consumed for transaction.
    pub gas_used: i64,
    /// The transaction's sender/signer.
    pub sender: String,
    /// The transaction's priority (for mempool ordering).
    pub priority: i64,
}

/// Response for end_block
#[derive(Debug, Default)]
pub struct ResponseEndBlock {
    /// Changes to validator set (set voting power to 0 to remove).
    pub validator_updates: Vec<ValidatorUpdate>,
    /// Changes to consensus-critical time, size, and other parameters.
    pub consensus_param_updates: Option<ConsensusParams>,
}