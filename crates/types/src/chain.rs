// This file is part of Rundler.
//
// Rundler is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later version.
//
// Rundler is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Rundler.
// If not, see https://www.gnu.org/licenses/.

//! Chain specification for Rundler

use std::{collections::HashMap, str::FromStr, sync::Arc};

use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use crate::{aggregator::SignatureAggregator, da::DAGasOracleType, proxy::SubmissionProxy};

const ENTRY_POINT_ADDRESS_V0_6: &str = "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789";
const ENTRY_POINT_ADDRESS_V0_7: &str = "0x0000000071727De22E5E9d8BAf0edAc6f37da032";
const MULTICALL3_ADDRESS: &str = "0xcA11bde05977b3631167028862bE2a173976CA11";

/// Chain specification for Rundler
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainSpec {
    /*
     * Chain constants
     */
    /// name for logging purposes, e.g. "Ethereum", no logic is performed on this
    pub name: String,
    /// chain id
    pub id: u64,
    /// entry point address for v0_6
    pub entry_point_address_v0_6: Address,
    /// entry point address for v0_7
    pub entry_point_address_v0_7: Address,
    /// address of the multicall3 contract
    pub multicall3_address: Address,

    /// Overhead when preforming gas estimation to account for the deposit storage
    /// and transfer overhead.
    ///
    /// NOTE: This must take into account when the storage slot was originally 0
    /// and is now non-zero, making the overhead slightly higher for most operations.
    pub deposit_transfer_overhead: u64,
    /// The maximum size of a transaction in bytes
    pub max_transaction_size_bytes: usize,
    /// the block gas limit
    pub block_gas_limit: u64,
    /// Intrinsic gas cost for a transaction
    pub transaction_intrinsic_gas: u64,
    /// Per user operation gas cost for v0.6
    pub per_user_op_v0_6_gas: u64,
    /// Per user operation gas cost for v0.7
    pub per_user_op_v0_7_gas: u64,
    /// Per user operation deploy gas cost overhead, to capture
    /// deploy costs that are not metered by the entry point
    pub per_user_op_deploy_overhead_gas: u64,
    /// Gas cost for a user operation word in a bundle transaction
    pub per_user_op_word_gas: u64,
    /// Gas cost for a zero byte in calldata
    pub calldata_zero_byte_gas: u64,
    /// Gas cost for a non-zero byte in calldata
    pub calldata_non_zero_byte_gas: u64,
    /// Gas cost for a zero byte in calldata for the floor operation
    pub calldata_floor_zero_byte_gas: u64,
    /// Gas cost for a non-zero byte in calldata for the floor operation
    pub calldata_floor_non_zero_byte_gas: u64,

    /*
     * Gas estimation
     */
    /// true if DA is priced in preVerificationGas
    pub da_pre_verification_gas: bool,
    /// type of gas oracle contract for pricing calldata in preVerificationGas
    /// If da_pre_verification_gas is true, this must not be None
    pub da_gas_oracle_type: DAGasOracleType,
    /// address of gas oracle contract for pricing calldata in preVerificationGas
    pub da_gas_oracle_contract_address: Address,
    /// true if Data Availability (DA) calldata gas should be included in the gas limit
    /// only applies when da_pre_verification_gas is true
    pub include_da_gas_in_gas_limit: bool,

    /*
     * EIPS
     */
    /// true if eip1559 is enabled, and thus priority fees are used
    pub eip1559_enabled: bool,
    /// true if eip7702 is enabled, and thus the 7702 priority fee mechanism is used
    pub eip7702_enabled: bool,

    /*
     * Fee estimation
     */
    /// Type of oracle for estimating priority fees
    pub priority_fee_oracle_type: PriorityFeeOracleType,
    /// Minimum max priority fee per gas for the network
    pub min_max_priority_fee_per_gas: u64,
    /// Maximum max priority fee per gas for the network
    pub max_max_priority_fee_per_gas: u64,
    /// Usage ratio of the chain that determines "congestion"
    /// Some chains have artificially high block gas limits but
    /// actually cap block gas usage at a lower value.
    pub congestion_trigger_usage_ratio_threshold: f64,

    /*
     * Bundle building
     */
    /// The maximum amount of time to wait before sending a bundle.
    ///
    /// The bundle builder will always try to send a bundle when a new block is received.
    /// This parameter is used to trigger the builder to send a bundle after a specified
    /// amount of time, before a new block is not received.
    pub bundle_max_send_interval_millis: u64,

    /*
     * Senders
     */
    /// True if the flashbots sender is enabled on this chain
    pub flashbots_enabled: bool,
    /// URL for the flashbots relay, must be set if flashbots is enabled
    pub flashbots_relay_url: Option<String>,
    /// True if the bloxroute sender is enabled on this chain
    pub bloxroute_enabled: bool,

    /*
     * Pool
     */
    /// Size of the chain history to keep to handle reorgs
    pub chain_history_size: u64,

    /*
     * Contracts
     */
    /// Registry of signature aggregators
    #[serde(skip)]
    pub signature_aggregators: Arc<ContractRegistry<Arc<dyn SignatureAggregator>>>,

    /*
     * Submission Proxies
     */
    /// Registry of submission proxies
    #[serde(skip)]
    pub submission_proxies: Arc<ContractRegistry<Arc<dyn SubmissionProxy>>>,
}

/// Type of oracle for estimating priority fees
#[derive(Clone, Debug, Deserialize, Default, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriorityFeeOracleType {
    /// Use eth_maxPriorityFeePerGas on the provider
    #[default]
    Provider,
    /// Use the usage based oracle
    UsageBased,
}

impl Default for ChainSpec {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            id: 0,
            block_gas_limit: 30_000_000,
            entry_point_address_v0_6: Address::from_str(ENTRY_POINT_ADDRESS_V0_6).unwrap(),
            entry_point_address_v0_7: Address::from_str(ENTRY_POINT_ADDRESS_V0_7).unwrap(),
            multicall3_address: Address::from_str(MULTICALL3_ADDRESS).unwrap(),
            deposit_transfer_overhead: 30_000,
            transaction_intrinsic_gas: 21_000,
            per_user_op_v0_6_gas: 18_300,
            per_user_op_v0_7_gas: 19_500,
            per_user_op_deploy_overhead_gas: 0,
            per_user_op_word_gas: 4,
            calldata_zero_byte_gas: 4,
            calldata_non_zero_byte_gas: 16,
            calldata_floor_zero_byte_gas: 0,
            calldata_floor_non_zero_byte_gas: 0,
            eip1559_enabled: true,
            eip7702_enabled: false,
            da_pre_verification_gas: false,
            da_gas_oracle_type: DAGasOracleType::default(),
            da_gas_oracle_contract_address: Address::ZERO,
            include_da_gas_in_gas_limit: false,
            priority_fee_oracle_type: PriorityFeeOracleType::default(),
            min_max_priority_fee_per_gas: 0,
            max_max_priority_fee_per_gas: u64::MAX,
            congestion_trigger_usage_ratio_threshold: 0.75,
            max_transaction_size_bytes: 131072, // 128 KiB
            bundle_max_send_interval_millis: 1000,
            flashbots_enabled: false,
            flashbots_relay_url: None,
            bloxroute_enabled: false,
            chain_history_size: 64,
            signature_aggregators: Arc::new(ContractRegistry::default()),
            submission_proxies: Arc::new(ContractRegistry::default()),
        }
    }
}

impl ChainSpec {
    /// Get the deposit transfer overhead
    pub fn deposit_transfer_overhead(&self) -> u128 {
        self.deposit_transfer_overhead as u128
    }

    /// Get the transaction intrinsic gas
    pub fn transaction_intrinsic_gas(&self) -> u128 {
        self.transaction_intrinsic_gas as u128
    }

    /// Get the minimum max priority fee per gas
    pub fn min_max_priority_fee_per_gas(&self) -> u128 {
        self.min_max_priority_fee_per_gas as u128
    }

    /// Get the maximum max priority fee per gas
    pub fn max_max_priority_fee_per_gas(&self) -> u128 {
        self.max_max_priority_fee_per_gas as u128
    }

    /// Get the per user operation word gas
    pub fn per_user_op_word_gas(&self) -> u128 {
        self.per_user_op_word_gas as u128
    }

    /// Get the per user operation v0_6 gas
    pub fn per_user_op_v0_6_gas(&self) -> u128 {
        self.per_user_op_v0_6_gas as u128
    }

    /// Get the per user operation v0_7 gas
    pub fn per_user_op_v0_7_gas(&self) -> u128 {
        self.per_user_op_v0_7_gas as u128
    }

    /// Get the calldata zero byte gas
    pub fn calldata_zero_byte_gas(&self) -> u128 {
        self.calldata_zero_byte_gas as u128
    }

    /// Get the calldata non zero byte gas
    pub fn calldata_non_zero_byte_gas(&self) -> u128 {
        self.calldata_non_zero_byte_gas as u128
    }

    /// Get the calldata floor zero byte gas
    pub fn calldata_floor_zero_byte_gas(&self) -> u128 {
        self.calldata_floor_zero_byte_gas as u128
    }

    /// Get the calldata floor non zero byte gas
    pub fn calldata_floor_non_zero_byte_gas(&self) -> u128 {
        self.calldata_floor_non_zero_byte_gas as u128
    }

    /// Get the per user operation deploy overhead gas
    pub fn per_user_op_deploy_overhead_gas(&self) -> u128 {
        self.per_user_op_deploy_overhead_gas as u128
    }

    /// Calculate a multiple of the block limit
    pub fn block_gas_limit_mult(&self, mult: f64) -> u128 {
        (self.block_gas_limit as f64 * mult) as u128
    }

    /// Set signature aggregators
    pub fn set_signature_aggregators(
        &mut self,
        signature_aggregators: Arc<ContractRegistry<Arc<dyn SignatureAggregator>>>,
    ) {
        self.signature_aggregators = signature_aggregators;
    }

    /// Get a signature aggregator from the registry
    pub fn get_signature_aggregator(
        &self,
        address: &Address,
    ) -> Option<&Arc<dyn SignatureAggregator>> {
        self.signature_aggregators.get(address)
    }

    /// Set submission proxies
    pub fn set_submission_proxies(
        &mut self,
        submission_proxies: Arc<ContractRegistry<Arc<dyn SubmissionProxy>>>,
    ) {
        self.submission_proxies = submission_proxies;
    }

    /// Get a submission proxy from the registry
    pub fn get_submission_proxy(&self, address: &Address) -> Option<&Arc<dyn SubmissionProxy>> {
        self.submission_proxies.get(address)
    }

    /// Get all known proxy addresses
    pub fn known_proxy_addresses(&self) -> impl Iterator<Item = &Address> {
        self.submission_proxies.contracts.keys()
    }

    /// Check if the chain supports EIP-7702
    pub fn supports_eip7702(&self, entry_point: Address) -> bool {
        self.eip7702_enabled || entry_point == self.entry_point_address_v0_7
    }
}

/// Registry of contracts
#[derive(Debug)]
pub struct ContractRegistry<T> {
    contracts: HashMap<Address, T>,
}

impl<T> ContractRegistry<T> {
    /// Register a contract in the registry
    pub fn register(&mut self, address: Address, contract: T) {
        self.contracts.insert(address, contract);
    }

    /// Get a contract from the registry
    pub fn get(&self, address: &Address) -> Option<&T> {
        self.contracts.get(address)
    }
}

impl<T> Default for ContractRegistry<T> {
    fn default() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }
}

/// Fallibly convert types with the help of the chain spec
pub trait TryFromWithSpec<T>: Sized {
    /// Convert error
    type Error;

    /// Fallibly convert types with the help of the chain spec
    fn try_from_with_spec(value: T, chain_spec: &ChainSpec) -> Result<Self, Self::Error>;
}

/// Fallibly convert types with the help of the chain spec
pub trait TryIntoWithSpec<T>: Sized {
    /// Convert error
    type Error;

    /// Fallibly convert types with the help of the chain spec
    fn try_into_with_spec(self, chain_spec: &ChainSpec) -> Result<T, Self::Error>;
}

impl<T, U> TryIntoWithSpec<U> for T
where
    U: TryFromWithSpec<T>,
{
    type Error = U::Error;
    fn try_into_with_spec(self, chain_spec: &ChainSpec) -> Result<U, U::Error> {
        U::try_from_with_spec(self, chain_spec)
    }
}

/// Convert types with the help of the chain spec
pub trait FromWithSpec<T>: Sized {
    /// Convert types with the help of the chain spec
    fn from_with_spec(value: T, chain_spec: &ChainSpec) -> Self;
}

/// Convert types with the help of the chain spec
pub trait IntoWithSpec<T>: Sized {
    /// Convert types with the help of the chain spec
    fn into_with_spec(self, chain_spec: &ChainSpec) -> T;
}

impl<T, U> IntoWithSpec<U> for T
where
    U: FromWithSpec<T>,
{
    fn into_with_spec(self, chain_spec: &ChainSpec) -> U {
        U::from_with_spec(self, chain_spec)
    }
}
