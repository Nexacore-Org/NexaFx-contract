import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}




/**
 * Status of the escrow operation
 */
export type EscrowStatus = {tag: "Active", values: void} | {tag: "Disputed", values: void} | {tag: "Released", values: void} | {tag: "Refunded", values: void} | {tag: "AutoReleased", values: void} | {tag: "DisputeResolvedForRecipient", values: void} | {tag: "DisputeResolvedForSender", values: void};


/**
 * Dispute information
 */
export interface DisputeInfo {
  /**
 * Dispute period duration in seconds
 */
dispute_period: u64;
  /**
 * Timestamp when dispute was initiated
 */
initiated_at: u64;
  /**
 * Who initiated the dispute
 */
initiated_by: string;
  /**
 * Reason for the dispute
 */
reason: string;
}


/**
 * Configuration for the escrow
 */
export interface EscrowConfig {
  /**
 * Amount of tokens in escrow
 */
amount: i128;
  /**
 * Timestamp when the escrow was created
 */
created_at: u64;
  /**
 * Dispute period in seconds (default dispute window)
 */
dispute_period: u64;
  /**
 * Has dispute flag
 */
has_dispute: boolean;
  /**
 * The escrow identifier
 */
id: string;
  /**
 * Address of the user receiving the funds
 */
recipient: string;
  /**
 * Address of the user sending the funds
 */
sender: string;
  /**
 * Current status of the escrow
 */
status: EscrowStatus;
  /**
 * Timeout period in seconds after which funds auto-release
 */
timeout_duration: u64;
  /**
 * Token being held in escrow
 */
token: string;
}


/**
 * Public information about an escrow
 */
export interface EscrowInfo {
  amount: i128;
  created_at: u64;
  dispute_period: u64;
  has_dispute: boolean;
  id: string;
  recipient: string;
  sender: string;
  status: EscrowStatus;
  timeout_at: u64;
  token: string;
}


export interface EscrowCreatedData {
  amount: i128;
  created_at: u64;
  escrow_id: string;
  recipient: string;
  sender: string;
  timeout_at: u64;
  token: string;
}


export interface EscrowReleasedData {
  amount: i128;
  escrow_id: string;
  recipient: string;
  released_at: u64;
  released_by: string;
  token: string;
}


export interface EscrowRefundedData {
  amount: i128;
  escrow_id: string;
  refunded_at: u64;
  refunded_by: string;
  sender: string;
  token: string;
}


export interface EscrowDisputeInitiatedData {
  dispute_period: u64;
  escrow_id: string;
  initiated_at: u64;
  initiated_by: string;
  reason: string;
}


export interface EscrowDisputeResolvedData {
  escrow_id: string;
  resolved_at: u64;
  resolved_for: string;
  resolved_for_recipient: boolean;
}


export interface SwapOfferCreatedData {
  created_at: u64;
  creator: string;
  exchange_rate: i128;
  expires_at: u64;
  offer_amount: i128;
  offer_id: u64;
  offer_token: string;
  request_amount: i128;
  request_token: string;
}


export interface SwapOfferAcceptedData {
  accepted_at: u64;
  acceptor: string;
  creator: string;
  fee_amount: i128;
  fee_token: string;
  offer_amount: i128;
  offer_id: u64;
  offer_token: string;
  request_amount: i128;
  request_token: string;
}


export interface TokenTransferredData {
  amount: i128;
  from: string;
  from_balance: i128;
  to: string;
  to_balance: i128;
  token: string;
  transferred_at: u64;
}


export interface TokenMintedData {
  amount: i128;
  minted_at: u64;
  minter: string;
  to: string;
  token: string;
}


export interface MultisigTransactionProposedData {
  current_signatures: u32;
  nonce: u32;
  operation_hash: Buffer;
  proposed_at: u64;
  proposer: string;
  threshold: u32;
}


export interface MultisigTransactionExecutedData {
  executed_at: u64;
  nonce: u32;
  operation_hash: Buffer;
  signers: Array<string>;
}


export interface MultisigConfigUpdatedData {
  new_signers: Array<string>;
  new_threshold: u32;
  old_signers: Array<string>;
  old_threshold: u32;
  updated_at: u64;
}


export interface WalletToppedUpData {
  amount: i128;
  new_balance: i128;
  source: string;
  token: string;
  topped_up_at: u64;
  wallet: string;
}


export interface ContractErrorData {
  context_data: Buffer;
  contract_address: string;
  error_message: Buffer;
  error_type: string;
  occurred_at: u64;
}

export type DeFiEvent = {tag: "EscrowCreated", values: readonly [EscrowCreatedData]} | {tag: "EscrowReleased", values: readonly [EscrowReleasedData]} | {tag: "EscrowRefunded", values: readonly [EscrowRefundedData]} | {tag: "EscrowDisputeInitiated", values: readonly [EscrowDisputeInitiatedData]} | {tag: "EscrowDisputeResolved", values: readonly [EscrowDisputeResolvedData]} | {tag: "SwapOfferCreated", values: readonly [SwapOfferCreatedData]} | {tag: "SwapOfferAccepted", values: readonly [SwapOfferAcceptedData]} | {tag: "TokenTransferred", values: readonly [TokenTransferredData]} | {tag: "TokenMinted", values: readonly [TokenMintedData]} | {tag: "MultisigTransactionProposed", values: readonly [MultisigTransactionProposedData]} | {tag: "MultisigTransactionExecuted", values: readonly [MultisigTransactionExecutedData]} | {tag: "MultisigConfigUpdated", values: readonly [MultisigConfigUpdatedData]} | {tag: "WalletToppedUp", values: readonly [WalletToppedUpData]} | {tag: "ContractError", values: readonly [ContractErrorData]};

/**
 * Supported currencies for conversion
 */
export type Currency = {tag: "NGN", values: void} | {tag: "USD", values: void} | {tag: "EUR", values: void} | {tag: "GBP", values: void} | {tag: "BTC", values: void} | {tag: "ETH", values: void};


/**
 * Exchange rate information
 */
export interface ExchangeRate {
  /**
 * Base currency
 */
from_currency: Currency;
  /**
 * Whether the rate is locked for transactions
 */
is_locked: boolean;
  /**
 * Exchange rate (scaled by 10^8 for precision)
 */
rate: i128;
  /**
 * Target currency
 */
to_currency: Currency;
  /**
 * Timestamp when rate was set
 */
updated_at: u64;
  /**
 * Rate validity duration in seconds
 */
validity_duration: u64;
}


/**
 * User balance information
 */
export interface UserBalance {
  /**
 * Currency balances map
 */
balances: Map<Currency, i128>;
  /**
 * Last updated timestamp
 */
updated_at: u64;
  /**
 * User's address
 */
user: string;
}


/**
 * Conversion transaction details
 */
export interface ConversionTx {
  /**
 * Amount to convert (in source currency)
 */
amount: i128;
  /**
 * Amount received (after fees)
 */
amount_received: i128;
  /**
 * Source currency
 */
from_currency: Currency;
  /**
 * Platform fee charged
 */
platform_fee: i128;
  /**
 * Exchange rate used
 */
rate: i128;
  /**
 * Transaction status
 */
status: ConversionStatus;
  /**
 * Timestamp of conversion
 */
timestamp: u64;
  /**
 * Target currency
 */
to_currency: Currency;
  /**
 * Transaction ID
 */
tx_id: string;
  /**
 * User performing conversion
 */
user: string;
}

/**
 * Status of conversion transaction
 */
export type ConversionStatus = {tag: "Pending", values: void} | {tag: "Completed", values: void} | {tag: "Failed", values: void} | {tag: "Cancelled", values: void};


/**
 * Platform configuration
 */
export interface PlatformConfig {
  /**
 * Platform admin
 */
admin: string;
  /**
 * Platform fee in basis points (e.g., 50 = 0.5%)
 */
fee_bps: u32;
  /**
 * Fee collector address
 */
fee_collector: string;
  /**
 * Maximum conversion amount per transaction
 */
max_conversion_amount: i128;
  /**
 * Minimum conversion amount
 */
min_conversion_amount: i128;
  /**
 * Rate lock duration in seconds
 */
rate_lock_duration: u64;
}

/**
 * Events emitted by the conversion contract
 */
export type ConversionEvent = {tag: "ConversionCompleted", values: readonly [string, string, Currency, Currency, i128, i128, i128]} | {tag: "RateUpdated", values: readonly [Currency, Currency, i128, u64]} | {tag: "RateLocked", values: readonly [Currency, Currency, i128, u64]} | {tag: "FeeCollected", values: readonly [Currency, i128, string]};

/**
 * Storage keys for the contract
 */
export type DataKey = {tag: "Config", values: void} | {tag: "Rate", values: readonly [Currency, Currency]} | {tag: "Balance", values: readonly [string]} | {tag: "Transaction", values: readonly [string]} | {tag: "TxCounter", values: void} | {tag: "SupportedCurrencies", values: void};


/**
 * Liquidity pool for a specific currency
 */
export interface LiquidityPool {
  /**
 * Available liquidity for conversions
 */
available_liquidity: i128;
  /**
 * Pool creation timestamp
 */
created_at: u64;
  /**
 * Currency of the pool
 */
currency: Currency;
  /**
 * Last activity timestamp
 */
last_activity_at: u64;
  /**
 * Minimum liquidity threshold
 */
min_liquidity_threshold: i128;
  /**
 * Number of liquidity providers
 */
provider_count: u32;
  /**
 * Reserved liquidity (locked in active conversions)
 */
reserved_liquidity: i128;
  /**
 * Total liquidity in the pool
 */
total_liquidity: i128;
  /**
 * Pool utilization rate (basis points)
 */
utilization_rate_bps: u32;
}


/**
 * Individual liquidity provider position
 */
export interface LiquidityPosition {
  /**
 * Accumulated rewards from conversions
 */
accumulated_rewards: i128;
  /**
 * Timestamp when liquidity was added
 */
added_at: u64;
  /**
 * Currency of the position
 */
currency: Currency;
  /**
 * Last time position was modified
 */
last_modified_at: u64;
  /**
 * Amount of liquidity provided
 */
liquidity_amount: i128;
  /**
 * Lock period end timestamp (0 if not locked)
 */
lock_until: u64;
  /**
 * Share of the pool (basis points)
 */
pool_share_bps: u32;
  /**
 * Provider's address
 */
provider: string;
}


/**
 * Pool manager configuration
 */
export interface PoolManagerConfig {
  /**
 * Administrator address
 */
admin: string;
  /**
 * Default liquidity lock period (seconds)
 */
default_lock_period: u64;
  /**
 * Emergency pause flag
 */
is_paused: boolean;
  /**
 * Maximum liquidity amount per provider
 */
max_liquidity_amount: i128;
  /**
 * Minimum liquidity amount per provider
 */
min_liquidity_amount: i128;
  /**
 * Reward rate for liquidity providers (basis points)
 */
provider_reward_rate_bps: u32;
  /**
 * Pool utilization threshold for warnings (basis points)
 */
utilization_warning_bps: u32;
}

/**
 * Pool manager events
 */
export type PoolManagerEvent = {tag: "LiquidityAdded", values: readonly [string, Currency, i128, u32]} | {tag: "LiquidityRemoved", values: readonly [string, Currency, i128, u32]} | {tag: "PoolBalanceUpdated", values: readonly [Currency, i128, i128, i128]} | {tag: "ProviderRewarded", values: readonly [string, Currency, i128]} | {tag: "PoolUtilizationWarning", values: readonly [Currency, u32]} | {tag: "EmergencyPauseActivated", values: readonly [string]} | {tag: "EmergencyPauseDeactivated", values: readonly [string]};

/**
 * Storage keys for pool manager
 */
export type PoolDataKey = {tag: "PoolConfig", values: void} | {tag: "Pool", values: readonly [Currency]} | {tag: "Position", values: readonly [string, Currency]} | {tag: "PositionCounter", values: void} | {tag: "ActiveCurrencies", values: void} | {tag: "UtilizationHistory", values: readonly [Currency, u64]} | {tag: "ProviderRewards", values: readonly [string]} | {tag: "CurrencyProviders", values: readonly [Currency]};

export type AppError = {tag: "InvalidAmount", values: void} | {tag: "InvalidAddress", values: void} | {tag: "InvalidTimestamp", values: void} | {tag: "InsufficientBalance", values: void} | {tag: "UnsupportedCurrency", values: void} | {tag: "RateExpired", values: void} | {tag: "ConversionLimitExceeded", values: void} | {tag: "Unauthorized", values: void};


export interface TokenConfig {
  admin: string;
  decimals: u32;
  name: string;
  symbol: string;
}


export interface Balance {
  amount: i128;
}


export interface MultiSigConfig {
  nonce: u32;
  signers: Array<string>;
  threshold: u32;
}


export interface Transaction {
  nonce: u32;
  operation: Buffer;
  timestamp: u64;
}

export const RateLockError = {
  1: {message:"NoRateLocked"},
  2: {message:"RateExpired"}
}

export const ContractError = {
  1: {message:"InvalidNonce"}
}

export type Event = {tag: "FeeCollected", values: readonly [string, i128]} | {tag: "OfferCreated", values: readonly [u64, string, i128]} | {tag: "OfferAccepted", values: readonly [u64, string]} | {tag: "OfferCancelled", values: readonly [u64]};


/**
 * Represents a swap offer in the contract's storage
 */
export interface SwapOffer {
  creator: string;
  expires_at: u64;
  offer_amount: i128;
  offer_token: string;
  request_amount: i128;
  request_token: string;
}


export interface SwapConfig {
  admin: string;
  fee_bps: u32;
  fee_collector: string;
}

export type DataKey = {tag: "Config", values: void} | {tag: "TotalDistributed", values: readonly [string]};


export interface FeeDistributionConfig {
  admin: string;
  reward_pool_address: string;
  reward_pool_bps: u32;
  treasury_address: string;
  treasury_bps: u32;
}


export interface TokenDistributionTotals {
  to_reward_pool: i128;
  to_treasury: i128;
}


export interface FeeDistributedEvent {
  fee_token: string;
  reward_pool_amount: i128;
  reward_pool_dest: string;
  total_collected_fee: i128;
  treasury_amount: i128;
  treasury_dest: string;
}

export interface Client {
  /**
   * Construct and simulate a create transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Create a new escrow
   */
  create: ({sender, recipient, token, amount, timeout_duration, dispute_period}: {sender: string, recipient: string, token: string, amount: i128, timeout_duration: u64, dispute_period: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a release transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Release funds to the recipient (can only be called by sender)
   */
  release: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a refund transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Refund the tokens back to the sender (can be called by both sender and recipient)
   */
  refund: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a check_timeout transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if the escrow has timed out and release funds if necessary
   */
  check_timeout: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a get_escrow transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get information about an escrow
   */
  get_escrow: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a get_all_escrows transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get all active escrows
   */
  get_all_escrows: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<EscrowInfo>>>

  /**
   * Construct and simulate a initiate_dispute transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initiate a dispute (can be called by sender or recipient)
   */
  initiate_dispute: ({escrow_id, reason}: {escrow_id: string, reason: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a resolve_dispute_for_recipient transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Resolve dispute in favor of recipient (admin function or automated)
   */
  resolve_dispute_for_recipient: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a resolve_dispute_for_sender transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Resolve dispute in favor of sender (admin function or automated)
   */
  resolve_dispute_for_sender: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a check_dispute_timeout transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if dispute has timed out and auto-resolve (default to recipient)
   */
  check_dispute_timeout: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a get_dispute_info transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get dispute information for an escrow
   */
  get_dispute_info: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<DisputeInfo>>>

  /**
   * Construct and simulate a can_dispute transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if an escrow can be disputed (is active and no existing dispute)
   */
  can_dispute: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a get_escrow_count transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get escrow count for statistics
   */
  get_escrow_count: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a escrow_exists transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if escrow exists
   */
  escrow_exists: ({escrow_id}: {escrow_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a get_escrows_by_status transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get escrows by status for filtering
   */
  get_escrows_by_status: ({status}: {status: EscrowStatus}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<EscrowInfo>>>

  /**
   * Construct and simulate a get_escrows_by_participant transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get escrows by participant (sender or recipient)
   */
  get_escrows_by_participant: ({participant}: {participant: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<EscrowInfo>>>

  /**
   * Construct and simulate a update_dispute_period transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Update dispute period for an active escrow (only by sender before dispute)
   */
  update_dispute_period: ({escrow_id, new_dispute_period}: {escrow_id: string, new_dispute_period: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initialize contract with admin (should be called once during deployment)
   */
  initialize: ({admin}: {admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a set_dispute_fee transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Set dispute fee (admin only)
   */
  set_dispute_fee: ({fee}: {fee: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a get_dispute_fee transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get current dispute fee
   */
  get_dispute_fee: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a get_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get contract admin
   */
  get_admin: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a transfer_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Transfer admin rights (admin only)
   */
  transfer_admin: ({new_admin}: {new_admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a set_paused transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Emergency pause/unpause functionality (admin only)
   */
  set_paused: ({paused}: {paused: boolean}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a is_paused transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if contract is paused
   */
  is_paused: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a admin_resolve_dispute transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Admin emergency resolution (admin only, for extreme cases)
   */
  admin_resolve_dispute: ({escrow_id, resolve_for_recipient}: {escrow_id: string, resolve_for_recipient: boolean}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<EscrowInfo>>

  /**
   * Construct and simulate a initialize_conversion transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initialize the conversion contract
   */
  initialize_conversion: ({admin, fee_bps, fee_collector, min_amount, max_amount}: {admin: string, fee_bps: u32, fee_collector: string, min_amount: i128, max_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<PlatformConfig>>

  /**
   * Construct and simulate a update_rate transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Update exchange rate for a currency pair
   */
  update_rate: ({from_currency, to_currency, rate, validity_duration}: {from_currency: Currency, to_currency: Currency, rate: i128, validity_duration: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<ExchangeRate>>

  /**
   * Construct and simulate a conversion_rate transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Lock exchange rate for a transaction
   */
  conversion_rate: ({from_currency, to_currency}: {from_currency: Currency, to_currency: Currency}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<ExchangeRate>>

  /**
   * Construct and simulate a convert_currency transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Perform currency conversion
   */
  convert_currency: ({user, from_currency, to_currency, amount}: {user: string, from_currency: Currency, to_currency: Currency, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<ConversionTx>>

  /**
   * Construct and simulate a get_user_balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get user balance for all currencies
   */
  get_user_balance: ({user}: {user: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<UserBalance>>

  /**
   * Construct and simulate a get_transaction transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get conversion transaction details
   */
  get_transaction: ({tx_id}: {tx_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<ConversionTx>>

  /**
   * Construct and simulate a get_rate transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get current exchange rate
   */
  get_rate: ({from_currency, to_currency}: {from_currency: Currency, to_currency: Currency}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<ExchangeRate>>

  /**
   * Construct and simulate a get_conversion_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get platform configuration
   */
  get_conversion_config: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<PlatformConfig>>

  /**
   * Construct and simulate a deposit transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  deposit: ({user, currency, amount}: {user: string, currency: Currency, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a initialize_pool_manager transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initialize the pool manager
   */
  initialize_pool_manager: ({admin, min_liquidity, max_liquidity, lock_period, reward_rate_bps}: {admin: string, min_liquidity: i128, max_liquidity: i128, lock_period: u64, reward_rate_bps: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<PoolManagerConfig>>

  /**
   * Construct and simulate a add_liquidity transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Add liquidity to a currency pool
   */
  add_liquidity: ({provider, currency, amount, lock_period}: {provider: string, currency: Currency, amount: i128, lock_period: Option<u64>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<LiquidityPosition>>

  /**
   * Construct and simulate a remove_liquidity transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Remove liquidity from a currency pool
   */
  remove_liquidity: ({provider, currency, amount}: {provider: string, currency: Currency, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<LiquidityPosition>>

  /**
   * Construct and simulate a update_pool_on_conversion transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Update pool balance during conversion operations
   */
  update_pool_on_conversion: ({from_currency, to_currency, from_amount, to_amount}: {from_currency: Currency, to_currency: Currency, from_amount: i128, to_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<readonly [LiquidityPool, LiquidityPool]>>

  /**
   * Construct and simulate a distribute_rewards transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Distribute rewards to liquidity providers
   */
  distribute_rewards: ({currency, total_fee_amount}: {currency: Currency, total_fee_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<readonly [string, i128]>>>

  /**
   * Construct and simulate a get_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get liquidity pool information
   */
  get_pool: ({currency}: {currency: Currency}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<LiquidityPool>>

  /**
   * Construct and simulate a get_position transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get liquidity position for a provider
   */
  get_position: ({provider, currency}: {provider: string, currency: Currency}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<LiquidityPosition>>

  /**
   * Construct and simulate a get_pool_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get pool manager configuration
   */
  get_pool_config: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<PoolManagerConfig>>

  /**
   * Construct and simulate a get_active_currencies transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get all active currencies with pools
   */
  get_active_currencies: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<Currency>>>

  /**
   * Construct and simulate a emergency_pause transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Emergency pause functionality
   */
  emergency_pause: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a resume_operations transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Resume operations after emergency pause
   */
  resume_operations: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a init transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  init: ({backend}: {backend: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a mint_token transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  mint_token: ({recipient, amount, token}: {recipient: string, amount: i128, token: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a initialize_token transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  initialize_token: ({admin, name, symbol, decimals}: {admin: string, name: string, symbol: string, decimals: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<TokenConfig>>

  /**
   * Construct and simulate a mint transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  mint: ({minter, to, amount}: {minter: string, to: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a transfer transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  transfer: ({from, to, amount}: {from: string, to: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  balance: ({of}: {of: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a get_token_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_token_config: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<TokenConfig>>

  /**
   * Construct and simulate a initialize_multisig transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  initialize_multisig: ({signers, threshold}: {signers: Array<string>, threshold: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<MultiSigConfig>>

  /**
   * Construct and simulate a propose_transaction transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  propose_transaction: ({operation, signatures, proposer}: {operation: Buffer, signatures: Array<Buffer>, proposer: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a get_multisig_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_multisig_config: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<MultiSigConfig>>

  /**
   * Construct and simulate a update_multisig_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update_multisig_config: ({new_signers, new_threshold, proposer}: {new_signers: Array<string>, new_threshold: u32, proposer: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<MultiSigConfig>>

  /**
   * Construct and simulate a lock_rate transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  lock_rate: ({user, rate, duration_seconds}: {user: string, rate: i128, duration_seconds: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a validate_conversion transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  validate_conversion: ({user}: {user: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<i128>>>

  /**
   * Construct and simulate a get_nonce transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_nonce: ({user}: {user: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a check_and_update_nonce transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  check_and_update_nonce: ({user, incoming}: {user: string, incoming: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u64>>>

  /**
   * Construct and simulate a initialize_fees transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  initialize_fees: ({admin, treasury_address, reward_pool_address, treasury_bps, reward_pool_bps}: {admin: string, treasury_address: string, reward_pool_address: string, treasury_bps: u32, reward_pool_bps: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a update_fees_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update_fees_config: ({treasury_address, reward_pool_address, treasury_bps, reward_pool_bps}: {treasury_address: Option<string>, reward_pool_address: Option<string>, treasury_bps: Option<u32>, reward_pool_bps: Option<u32>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<FeeDistributionConfig>>>

  /**
   * Construct and simulate a get_fees_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_fees_config: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<FeeDistributionConfig>>>

  /**
   * Construct and simulate a distribute_fees transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Distributes collected fees to treasury and reward pools.
   * This function should be called by the contract that collected the fees.
   * `fee_collector_contract` is the address holding the `total_fee_amount`.
   */
  distribute_fees: ({fee_token, total_fee_amount, fee_collector_contract}: {fee_token: string, total_fee_amount: i128, fee_collector_contract: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_total_distributed transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_total_distributed: ({token}: {token: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<TokenDistributionTotals>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAgAAAB5TdGF0dXMgb2YgdGhlIGVzY3JvdyBvcGVyYXRpb24AAAAAAAAAAAAMRXNjcm93U3RhdHVzAAAABwAAAAAAAAAlRXNjcm93IGlzIGFjdGl2ZSBhbmQgZnVuZHMgYXJlIGxvY2tlZAAAAAAAAAZBY3RpdmUAAAAAAAAAAAAcQSBkaXNwdXRlIGhhcyBiZWVuIGluaXRpYXRlZAAAAAhEaXNwdXRlZAAAAAAAAAApRnVuZHMgaGF2ZSBiZWVuIHJlbGVhc2VkIHRvIHRoZSByZWNpcGllbnQAAAAAAAAIUmVsZWFzZWQAAAAAAAAAJkZ1bmRzIGhhdmUgYmVlbiByZXR1cm5lZCB0byB0aGUgc2VuZGVyAAAAAAAIUmVmdW5kZWQAAAAAAAAAL0Z1bmRzIHdlcmUgYXV0b21hdGljYWxseSByZWxlYXNlZCBhZnRlciB0aW1lb3V0AAAAAAxBdXRvUmVsZWFzZWQAAAAAAAAAKkRpc3B1dGUgd2FzIHJlc29sdmVkIGluIGZhdm9yIG9mIHJlY2lwaWVudAAAAAAAG0Rpc3B1dGVSZXNvbHZlZEZvclJlY2lwaWVudAAAAAAAAAAAJ0Rpc3B1dGUgd2FzIHJlc29sdmVkIGluIGZhdm9yIG9mIHNlbmRlcgAAAAAYRGlzcHV0ZVJlc29sdmVkRm9yU2VuZGVy",
        "AAAAAQAAABNEaXNwdXRlIGluZm9ybWF0aW9uAAAAAAAAAAALRGlzcHV0ZUluZm8AAAAABAAAACJEaXNwdXRlIHBlcmlvZCBkdXJhdGlvbiBpbiBzZWNvbmRzAAAAAAAOZGlzcHV0ZV9wZXJpb2QAAAAAAAYAAAAkVGltZXN0YW1wIHdoZW4gZGlzcHV0ZSB3YXMgaW5pdGlhdGVkAAAADGluaXRpYXRlZF9hdAAAAAYAAAAZV2hvIGluaXRpYXRlZCB0aGUgZGlzcHV0ZQAAAAAAAAxpbml0aWF0ZWRfYnkAAAATAAAAFlJlYXNvbiBmb3IgdGhlIGRpc3B1dGUAAAAAAAZyZWFzb24AAAAAABE=",
        "AAAAAQAAABxDb25maWd1cmF0aW9uIGZvciB0aGUgZXNjcm93AAAAAAAAAAxFc2Nyb3dDb25maWcAAAAKAAAAGkFtb3VudCBvZiB0b2tlbnMgaW4gZXNjcm93AAAAAAAGYW1vdW50AAAAAAALAAAAJVRpbWVzdGFtcCB3aGVuIHRoZSBlc2Nyb3cgd2FzIGNyZWF0ZWQAAAAAAAAKY3JlYXRlZF9hdAAAAAAABgAAADJEaXNwdXRlIHBlcmlvZCBpbiBzZWNvbmRzIChkZWZhdWx0IGRpc3B1dGUgd2luZG93KQAAAAAADmRpc3B1dGVfcGVyaW9kAAAAAAAGAAAAEEhhcyBkaXNwdXRlIGZsYWcAAAALaGFzX2Rpc3B1dGUAAAAAAQAAABVUaGUgZXNjcm93IGlkZW50aWZpZXIAAAAAAAACaWQAAAAAABEAAAAnQWRkcmVzcyBvZiB0aGUgdXNlciByZWNlaXZpbmcgdGhlIGZ1bmRzAAAAAAlyZWNpcGllbnQAAAAAAAATAAAAJUFkZHJlc3Mgb2YgdGhlIHVzZXIgc2VuZGluZyB0aGUgZnVuZHMAAAAAAAAGc2VuZGVyAAAAAAATAAAAHEN1cnJlbnQgc3RhdHVzIG9mIHRoZSBlc2Nyb3cAAAAGc3RhdHVzAAAAAAfQAAAADEVzY3Jvd1N0YXR1cwAAADhUaW1lb3V0IHBlcmlvZCBpbiBzZWNvbmRzIGFmdGVyIHdoaWNoIGZ1bmRzIGF1dG8tcmVsZWFzZQAAABB0aW1lb3V0X2R1cmF0aW9uAAAABgAAABpUb2tlbiBiZWluZyBoZWxkIGluIGVzY3JvdwAAAAAABXRva2VuAAAAAAAAEw==",
        "AAAAAQAAACJQdWJsaWMgaW5mb3JtYXRpb24gYWJvdXQgYW4gZXNjcm93AAAAAAAAAAAACkVzY3Jvd0luZm8AAAAAAAoAAAAAAAAABmFtb3VudAAAAAAACwAAAAAAAAAKY3JlYXRlZF9hdAAAAAAABgAAAAAAAAAOZGlzcHV0ZV9wZXJpb2QAAAAAAAYAAAAAAAAAC2hhc19kaXNwdXRlAAAAAAEAAAAAAAAAAmlkAAAAAAARAAAAAAAAAAlyZWNpcGllbnQAAAAAAAATAAAAAAAAAAZzZW5kZXIAAAAAABMAAAAAAAAABnN0YXR1cwAAAAAH0AAAAAxFc2Nyb3dTdGF0dXMAAAAAAAAACnRpbWVvdXRfYXQAAAAAAAYAAAAAAAAABXRva2VuAAAAAAAAEw==",
        "AAAAAAAAABNDcmVhdGUgYSBuZXcgZXNjcm93AAAAAAZjcmVhdGUAAAAAAAYAAAAAAAAABnNlbmRlcgAAAAAAEwAAAAAAAAAJcmVjaXBpZW50AAAAAAAAEwAAAAAAAAAFdG9rZW4AAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAAEHRpbWVvdXRfZHVyYXRpb24AAAAGAAAAAAAAAA5kaXNwdXRlX3BlcmlvZAAAAAAABgAAAAEAAAfQAAAACkVzY3Jvd0luZm8AAA==",
        "AAAAAAAAAD1SZWxlYXNlIGZ1bmRzIHRvIHRoZSByZWNpcGllbnQgKGNhbiBvbmx5IGJlIGNhbGxlZCBieSBzZW5kZXIpAAAAAAAAB3JlbGVhc2UAAAAAAQAAAAAAAAAJZXNjcm93X2lkAAAAAAAAEQAAAAEAAAfQAAAACkVzY3Jvd0luZm8AAA==",
        "AAAAAAAAAFFSZWZ1bmQgdGhlIHRva2VucyBiYWNrIHRvIHRoZSBzZW5kZXIgKGNhbiBiZSBjYWxsZWQgYnkgYm90aCBzZW5kZXIgYW5kIHJlY2lwaWVudCkAAAAAAAAGcmVmdW5kAAAAAAABAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAQAAB9AAAAAKRXNjcm93SW5mbwAA",
        "AAAAAAAAAEBDaGVjayBpZiB0aGUgZXNjcm93IGhhcyB0aW1lZCBvdXQgYW5kIHJlbGVhc2UgZnVuZHMgaWYgbmVjZXNzYXJ5AAAADWNoZWNrX3RpbWVvdXQAAAAAAAABAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAQAAB9AAAAAKRXNjcm93SW5mbwAA",
        "AAAAAAAAAB9HZXQgaW5mb3JtYXRpb24gYWJvdXQgYW4gZXNjcm93AAAAAApnZXRfZXNjcm93AAAAAAABAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAQAAB9AAAAAKRXNjcm93SW5mbwAA",
        "AAAAAAAAABZHZXQgYWxsIGFjdGl2ZSBlc2Nyb3dzAAAAAAAPZ2V0X2FsbF9lc2Nyb3dzAAAAAAAAAAABAAAD6gAAB9AAAAAKRXNjcm93SW5mbwAA",
        "AAAAAAAAADlJbml0aWF0ZSBhIGRpc3B1dGUgKGNhbiBiZSBjYWxsZWQgYnkgc2VuZGVyIG9yIHJlY2lwaWVudCkAAAAAAAAQaW5pdGlhdGVfZGlzcHV0ZQAAAAIAAAAAAAAACWVzY3Jvd19pZAAAAAAAABEAAAAAAAAABnJlYXNvbgAAAAAAEQAAAAEAAAfQAAAACkVzY3Jvd0luZm8AAA==",
        "AAAAAAAAAENSZXNvbHZlIGRpc3B1dGUgaW4gZmF2b3Igb2YgcmVjaXBpZW50IChhZG1pbiBmdW5jdGlvbiBvciBhdXRvbWF0ZWQpAAAAAB1yZXNvbHZlX2Rpc3B1dGVfZm9yX3JlY2lwaWVudAAAAAAAAAEAAAAAAAAACWVzY3Jvd19pZAAAAAAAABEAAAABAAAH0AAAAApFc2Nyb3dJbmZvAAA=",
        "AAAAAAAAAEBSZXNvbHZlIGRpc3B1dGUgaW4gZmF2b3Igb2Ygc2VuZGVyIChhZG1pbiBmdW5jdGlvbiBvciBhdXRvbWF0ZWQpAAAAGnJlc29sdmVfZGlzcHV0ZV9mb3Jfc2VuZGVyAAAAAAABAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAQAAB9AAAAAKRXNjcm93SW5mbwAA",
        "AAAAAAAAAEZDaGVjayBpZiBkaXNwdXRlIGhhcyB0aW1lZCBvdXQgYW5kIGF1dG8tcmVzb2x2ZSAoZGVmYXVsdCB0byByZWNpcGllbnQpAAAAAAAVY2hlY2tfZGlzcHV0ZV90aW1lb3V0AAAAAAAAAQAAAAAAAAAJZXNjcm93X2lkAAAAAAAAEQAAAAEAAAfQAAAACkVzY3Jvd0luZm8AAA==",
        "AAAAAAAAACVHZXQgZGlzcHV0ZSBpbmZvcm1hdGlvbiBmb3IgYW4gZXNjcm93AAAAAAAAEGdldF9kaXNwdXRlX2luZm8AAAABAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAQAAA+gAAAfQAAAAC0Rpc3B1dGVJbmZvAA==",
        "AAAAAAAAAEZDaGVjayBpZiBhbiBlc2Nyb3cgY2FuIGJlIGRpc3B1dGVkIChpcyBhY3RpdmUgYW5kIG5vIGV4aXN0aW5nIGRpc3B1dGUpAAAAAAALY2FuX2Rpc3B1dGUAAAAAAQAAAAAAAAAJZXNjcm93X2lkAAAAAAAAEQAAAAEAAAAB",
        "AAAAAAAAAB9HZXQgZXNjcm93IGNvdW50IGZvciBzdGF0aXN0aWNzAAAAABBnZXRfZXNjcm93X2NvdW50AAAAAAAAAAEAAAAE",
        "AAAAAAAAABZDaGVjayBpZiBlc2Nyb3cgZXhpc3RzAAAAAAANZXNjcm93X2V4aXN0cwAAAAAAAAEAAAAAAAAACWVzY3Jvd19pZAAAAAAAABEAAAABAAAAAQ==",
        "AAAAAAAAACNHZXQgZXNjcm93cyBieSBzdGF0dXMgZm9yIGZpbHRlcmluZwAAAAAVZ2V0X2VzY3Jvd3NfYnlfc3RhdHVzAAAAAAAAAQAAAAAAAAAGc3RhdHVzAAAAAAfQAAAADEVzY3Jvd1N0YXR1cwAAAAEAAAPqAAAH0AAAAApFc2Nyb3dJbmZvAAA=",
        "AAAAAAAAADBHZXQgZXNjcm93cyBieSBwYXJ0aWNpcGFudCAoc2VuZGVyIG9yIHJlY2lwaWVudCkAAAAaZ2V0X2VzY3Jvd3NfYnlfcGFydGljaXBhbnQAAAAAAAEAAAAAAAAAC3BhcnRpY2lwYW50AAAAABMAAAABAAAD6gAAB9AAAAAKRXNjcm93SW5mbwAA",
        "AAAAAAAAAEpVcGRhdGUgZGlzcHV0ZSBwZXJpb2QgZm9yIGFuIGFjdGl2ZSBlc2Nyb3cgKG9ubHkgYnkgc2VuZGVyIGJlZm9yZSBkaXNwdXRlKQAAAAAAFXVwZGF0ZV9kaXNwdXRlX3BlcmlvZAAAAAAAAAIAAAAAAAAACWVzY3Jvd19pZAAAAAAAABEAAAAAAAAAEm5ld19kaXNwdXRlX3BlcmlvZAAAAAAABgAAAAEAAAfQAAAACkVzY3Jvd0luZm8AAA==",
        "AAAAAAAAAEhJbml0aWFsaXplIGNvbnRyYWN0IHdpdGggYWRtaW4gKHNob3VsZCBiZSBjYWxsZWQgb25jZSBkdXJpbmcgZGVwbG95bWVudCkAAAAKaW5pdGlhbGl6ZQAAAAAAAQAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAA==",
        "AAAAAAAAABxTZXQgZGlzcHV0ZSBmZWUgKGFkbWluIG9ubHkpAAAAD3NldF9kaXNwdXRlX2ZlZQAAAAABAAAAAAAAAANmZWUAAAAACwAAAAA=",
        "AAAAAAAAABdHZXQgY3VycmVudCBkaXNwdXRlIGZlZQAAAAAPZ2V0X2Rpc3B1dGVfZmVlAAAAAAAAAAABAAAACw==",
        "AAAAAAAAABJHZXQgY29udHJhY3QgYWRtaW4AAAAAAAlnZXRfYWRtaW4AAAAAAAAAAAAAAQAAABM=",
        "AAAAAAAAACJUcmFuc2ZlciBhZG1pbiByaWdodHMgKGFkbWluIG9ubHkpAAAAAAAOdHJhbnNmZXJfYWRtaW4AAAAAAAEAAAAAAAAACW5ld19hZG1pbgAAAAAAABMAAAAA",
        "AAAAAAAAADJFbWVyZ2VuY3kgcGF1c2UvdW5wYXVzZSBmdW5jdGlvbmFsaXR5IChhZG1pbiBvbmx5KQAAAAAACnNldF9wYXVzZWQAAAAAAAEAAAAAAAAABnBhdXNlZAAAAAAAAQAAAAA=",
        "AAAAAAAAABtDaGVjayBpZiBjb250cmFjdCBpcyBwYXVzZWQAAAAACWlzX3BhdXNlZAAAAAAAAAAAAAABAAAAAQ==",
        "AAAAAAAAADpBZG1pbiBlbWVyZ2VuY3kgcmVzb2x1dGlvbiAoYWRtaW4gb25seSwgZm9yIGV4dHJlbWUgY2FzZXMpAAAAAAAVYWRtaW5fcmVzb2x2ZV9kaXNwdXRlAAAAAAAAAgAAAAAAAAAJZXNjcm93X2lkAAAAAAAAEQAAAAAAAAAVcmVzb2x2ZV9mb3JfcmVjaXBpZW50AAAAAAAAAQAAAAEAAAfQAAAACkVzY3Jvd0luZm8AAA==",
        "AAAAAQAAAAAAAAAAAAAAEUVzY3Jvd0NyZWF0ZWREYXRhAAAAAAAABwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAApjcmVhdGVkX2F0AAAAAAAGAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAAAAAAlyZWNpcGllbnQAAAAAAAATAAAAAAAAAAZzZW5kZXIAAAAAABMAAAAAAAAACnRpbWVvdXRfYXQAAAAAAAYAAAAAAAAABXRva2VuAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAEkVzY3Jvd1JlbGVhc2VkRGF0YQAAAAAABgAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAAAAAAlyZWNpcGllbnQAAAAAAAATAAAAAAAAAAtyZWxlYXNlZF9hdAAAAAAGAAAAAAAAAAtyZWxlYXNlZF9ieQAAAAATAAAAAAAAAAV0b2tlbgAAAAAAABM=",
        "AAAAAQAAAAAAAAAAAAAAEkVzY3Jvd1JlZnVuZGVkRGF0YQAAAAAABgAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAAAAAAtyZWZ1bmRlZF9hdAAAAAAGAAAAAAAAAAtyZWZ1bmRlZF9ieQAAAAATAAAAAAAAAAZzZW5kZXIAAAAAABMAAAAAAAAABXRva2VuAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAGkVzY3Jvd0Rpc3B1dGVJbml0aWF0ZWREYXRhAAAAAAAFAAAAAAAAAA5kaXNwdXRlX3BlcmlvZAAAAAAABgAAAAAAAAAJZXNjcm93X2lkAAAAAAAAEQAAAAAAAAAMaW5pdGlhdGVkX2F0AAAABgAAAAAAAAAMaW5pdGlhdGVkX2J5AAAAEwAAAAAAAAAGcmVhc29uAAAAAAAR",
        "AAAAAQAAAAAAAAAAAAAAGUVzY3Jvd0Rpc3B1dGVSZXNvbHZlZERhdGEAAAAAAAAEAAAAAAAAAAllc2Nyb3dfaWQAAAAAAAARAAAAAAAAAAtyZXNvbHZlZF9hdAAAAAAGAAAAAAAAAAxyZXNvbHZlZF9mb3IAAAATAAAAAAAAABZyZXNvbHZlZF9mb3JfcmVjaXBpZW50AAAAAAAB",
        "AAAAAQAAAAAAAAAAAAAAFFN3YXBPZmZlckNyZWF0ZWREYXRhAAAACQAAAAAAAAAKY3JlYXRlZF9hdAAAAAAABgAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAA1leGNoYW5nZV9yYXRlAAAAAAAACwAAAAAAAAAKZXhwaXJlc19hdAAAAAAABgAAAAAAAAAMb2ZmZXJfYW1vdW50AAAACwAAAAAAAAAIb2ZmZXJfaWQAAAAGAAAAAAAAAAtvZmZlcl90b2tlbgAAAAATAAAAAAAAAA5yZXF1ZXN0X2Ftb3VudAAAAAAACwAAAAAAAAANcmVxdWVzdF90b2tlbgAAAAAAABM=",
        "AAAAAQAAAAAAAAAAAAAAFVN3YXBPZmZlckFjY2VwdGVkRGF0YQAAAAAAAAoAAAAAAAAAC2FjY2VwdGVkX2F0AAAAAAYAAAAAAAAACGFjY2VwdG9yAAAAEwAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAApmZWVfYW1vdW50AAAAAAALAAAAAAAAAAlmZWVfdG9rZW4AAAAAAAATAAAAAAAAAAxvZmZlcl9hbW91bnQAAAALAAAAAAAAAAhvZmZlcl9pZAAAAAYAAAAAAAAAC29mZmVyX3Rva2VuAAAAABMAAAAAAAAADnJlcXVlc3RfYW1vdW50AAAAAAALAAAAAAAAAA1yZXF1ZXN0X3Rva2VuAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAFFRva2VuVHJhbnNmZXJyZWREYXRhAAAABwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAMZnJvbV9iYWxhbmNlAAAACwAAAAAAAAACdG8AAAAAABMAAAAAAAAACnRvX2JhbGFuY2UAAAAAAAsAAAAAAAAABXRva2VuAAAAAAAAEwAAAAAAAAAOdHJhbnNmZXJyZWRfYXQAAAAAAAY=",
        "AAAAAQAAAAAAAAAAAAAAD1Rva2VuTWludGVkRGF0YQAAAAAFAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAACW1pbnRlZF9hdAAAAAAAAAYAAAAAAAAABm1pbnRlcgAAAAAAEwAAAAAAAAACdG8AAAAAABMAAAAAAAAABXRva2VuAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAH011bHRpc2lnVHJhbnNhY3Rpb25Qcm9wb3NlZERhdGEAAAAABgAAAAAAAAASY3VycmVudF9zaWduYXR1cmVzAAAAAAAEAAAAAAAAAAVub25jZQAAAAAAAAQAAAAAAAAADm9wZXJhdGlvbl9oYXNoAAAAAAPuAAAAIAAAAAAAAAALcHJvcG9zZWRfYXQAAAAABgAAAAAAAAAIcHJvcG9zZXIAAAATAAAAAAAAAAl0aHJlc2hvbGQAAAAAAAAE",
        "AAAAAQAAAAAAAAAAAAAAH011bHRpc2lnVHJhbnNhY3Rpb25FeGVjdXRlZERhdGEAAAAABAAAAAAAAAALZXhlY3V0ZWRfYXQAAAAABgAAAAAAAAAFbm9uY2UAAAAAAAAEAAAAAAAAAA5vcGVyYXRpb25faGFzaAAAAAAD7gAAACAAAAAAAAAAB3NpZ25lcnMAAAAD6gAAABM=",
        "AAAAAQAAAAAAAAAAAAAAGU11bHRpc2lnQ29uZmlnVXBkYXRlZERhdGEAAAAAAAAFAAAAAAAAAAtuZXdfc2lnbmVycwAAAAPqAAAAEwAAAAAAAAANbmV3X3RocmVzaG9sZAAAAAAAAAQAAAAAAAAAC29sZF9zaWduZXJzAAAAA+oAAAATAAAAAAAAAA1vbGRfdGhyZXNob2xkAAAAAAAABAAAAAAAAAAKdXBkYXRlZF9hdAAAAAAABg==",
        "AAAAAQAAAAAAAAAAAAAAEldhbGxldFRvcHBlZFVwRGF0YQAAAAAABgAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAtuZXdfYmFsYW5jZQAAAAALAAAAAAAAAAZzb3VyY2UAAAAAABMAAAAAAAAABXRva2VuAAAAAAAAEwAAAAAAAAAMdG9wcGVkX3VwX2F0AAAABgAAAAAAAAAGd2FsbGV0AAAAAAAT",
        "AAAAAQAAAAAAAAAAAAAAEUNvbnRyYWN0RXJyb3JEYXRhAAAAAAAABQAAAAAAAAAMY29udGV4dF9kYXRhAAAADgAAAAAAAAAQY29udHJhY3RfYWRkcmVzcwAAABMAAAAAAAAADWVycm9yX21lc3NhZ2UAAAAAAAAOAAAAAAAAAAplcnJvcl90eXBlAAAAAAARAAAAAAAAAAtvY2N1cnJlZF9hdAAAAAAG",
        "AAAAAgAAAAAAAAAAAAAACURlRmlFdmVudAAAAAAAAA4AAAABAAAAAAAAAA1Fc2Nyb3dDcmVhdGVkAAAAAAAAAQAAB9AAAAARRXNjcm93Q3JlYXRlZERhdGEAAAAAAAABAAAAAAAAAA5Fc2Nyb3dSZWxlYXNlZAAAAAAAAQAAB9AAAAASRXNjcm93UmVsZWFzZWREYXRhAAAAAAABAAAAAAAAAA5Fc2Nyb3dSZWZ1bmRlZAAAAAAAAQAAB9AAAAASRXNjcm93UmVmdW5kZWREYXRhAAAAAAABAAAAAAAAABZFc2Nyb3dEaXNwdXRlSW5pdGlhdGVkAAAAAAABAAAH0AAAABpFc2Nyb3dEaXNwdXRlSW5pdGlhdGVkRGF0YQAAAAAAAQAAAAAAAAAVRXNjcm93RGlzcHV0ZVJlc29sdmVkAAAAAAAAAQAAB9AAAAAZRXNjcm93RGlzcHV0ZVJlc29sdmVkRGF0YQAAAAAAAAEAAAAAAAAAEFN3YXBPZmZlckNyZWF0ZWQAAAABAAAH0AAAABRTd2FwT2ZmZXJDcmVhdGVkRGF0YQAAAAEAAAAAAAAAEVN3YXBPZmZlckFjY2VwdGVkAAAAAAAAAQAAB9AAAAAVU3dhcE9mZmVyQWNjZXB0ZWREYXRhAAAAAAAAAQAAAAAAAAAQVG9rZW5UcmFuc2ZlcnJlZAAAAAEAAAfQAAAAFFRva2VuVHJhbnNmZXJyZWREYXRhAAAAAQAAAAAAAAALVG9rZW5NaW50ZWQAAAAAAQAAB9AAAAAPVG9rZW5NaW50ZWREYXRhAAAAAAEAAAAAAAAAG011bHRpc2lnVHJhbnNhY3Rpb25Qcm9wb3NlZAAAAAABAAAH0AAAAB9NdWx0aXNpZ1RyYW5zYWN0aW9uUHJvcG9zZWREYXRhAAAAAAEAAAAAAAAAG011bHRpc2lnVHJhbnNhY3Rpb25FeGVjdXRlZAAAAAABAAAH0AAAAB9NdWx0aXNpZ1RyYW5zYWN0aW9uRXhlY3V0ZWREYXRhAAAAAAEAAAAAAAAAFU11bHRpc2lnQ29uZmlnVXBkYXRlZAAAAAAAAAEAAAfQAAAAGU11bHRpc2lnQ29uZmlnVXBkYXRlZERhdGEAAAAAAAABAAAAAAAAAA5XYWxsZXRUb3BwZWRVcAAAAAAAAQAAB9AAAAASV2FsbGV0VG9wcGVkVXBEYXRhAAAAAAABAAAAAAAAAA1Db250cmFjdEVycm9yAAAAAAAAAQAAB9AAAAARQ29udHJhY3RFcnJvckRhdGEAAAA=",
        "AAAAAgAAACNTdXBwb3J0ZWQgY3VycmVuY2llcyBmb3IgY29udmVyc2lvbgAAAAAAAAAACEN1cnJlbmN5AAAABgAAAAAAAAAAAAAAA05HTgAAAAAAAAAAAAAAAANVU0QAAAAAAAAAAAAAAAADRVVSAAAAAAAAAAAAAAAAA0dCUAAAAAAAAAAAAAAAAANCVEMAAAAAAAAAAAAAAAADRVRIAA==",
        "AAAAAQAAABlFeGNoYW5nZSByYXRlIGluZm9ybWF0aW9uAAAAAAAAAAAAAAxFeGNoYW5nZVJhdGUAAAAGAAAADUJhc2UgY3VycmVuY3kAAAAAAAANZnJvbV9jdXJyZW5jeQAAAAAAB9AAAAAIQ3VycmVuY3kAAAArV2hldGhlciB0aGUgcmF0ZSBpcyBsb2NrZWQgZm9yIHRyYW5zYWN0aW9ucwAAAAAJaXNfbG9ja2VkAAAAAAAAAQAAACxFeGNoYW5nZSByYXRlIChzY2FsZWQgYnkgMTBeOCBmb3IgcHJlY2lzaW9uKQAAAARyYXRlAAAACwAAAA9UYXJnZXQgY3VycmVuY3kAAAAAC3RvX2N1cnJlbmN5AAAAB9AAAAAIQ3VycmVuY3kAAAAbVGltZXN0YW1wIHdoZW4gcmF0ZSB3YXMgc2V0AAAAAAp1cGRhdGVkX2F0AAAAAAAGAAAAIVJhdGUgdmFsaWRpdHkgZHVyYXRpb24gaW4gc2Vjb25kcwAAAAAAABF2YWxpZGl0eV9kdXJhdGlvbgAAAAAAAAY=",
        "AAAAAQAAABhVc2VyIGJhbGFuY2UgaW5mb3JtYXRpb24AAAAAAAAAC1VzZXJCYWxhbmNlAAAAAAMAAAAVQ3VycmVuY3kgYmFsYW5jZXMgbWFwAAAAAAAACGJhbGFuY2VzAAAD7AAAB9AAAAAIQ3VycmVuY3kAAAALAAAAFkxhc3QgdXBkYXRlZCB0aW1lc3RhbXAAAAAAAAp1cGRhdGVkX2F0AAAAAAAGAAAADlVzZXIncyBhZGRyZXNzAAAAAAAEdXNlcgAAABM=",
        "AAAAAQAAAB5Db252ZXJzaW9uIHRyYW5zYWN0aW9uIGRldGFpbHMAAAAAAAAAAAAMQ29udmVyc2lvblR4AAAACgAAACZBbW91bnQgdG8gY29udmVydCAoaW4gc291cmNlIGN1cnJlbmN5KQAAAAAABmFtb3VudAAAAAAACwAAABxBbW91bnQgcmVjZWl2ZWQgKGFmdGVyIGZlZXMpAAAAD2Ftb3VudF9yZWNlaXZlZAAAAAALAAAAD1NvdXJjZSBjdXJyZW5jeQAAAAANZnJvbV9jdXJyZW5jeQAAAAAAB9AAAAAIQ3VycmVuY3kAAAAUUGxhdGZvcm0gZmVlIGNoYXJnZWQAAAAMcGxhdGZvcm1fZmVlAAAACwAAABJFeGNoYW5nZSByYXRlIHVzZWQAAAAAAARyYXRlAAAACwAAABJUcmFuc2FjdGlvbiBzdGF0dXMAAAAAAAZzdGF0dXMAAAAAB9AAAAAQQ29udmVyc2lvblN0YXR1cwAAABdUaW1lc3RhbXAgb2YgY29udmVyc2lvbgAAAAAJdGltZXN0YW1wAAAAAAAABgAAAA9UYXJnZXQgY3VycmVuY3kAAAAAC3RvX2N1cnJlbmN5AAAAB9AAAAAIQ3VycmVuY3kAAAAOVHJhbnNhY3Rpb24gSUQAAAAAAAV0eF9pZAAAAAAAABEAAAAaVXNlciBwZXJmb3JtaW5nIGNvbnZlcnNpb24AAAAAAAR1c2VyAAAAEw==",
        "AAAAAgAAACBTdGF0dXMgb2YgY29udmVyc2lvbiB0cmFuc2FjdGlvbgAAAAAAAAAQQ29udmVyc2lvblN0YXR1cwAAAAQAAAAAAAAAAAAAAAdQZW5kaW5nAAAAAAAAAAAAAAAACUNvbXBsZXRlZAAAAAAAAAAAAAAAAAAABkZhaWxlZAAAAAAAAAAAAAAAAAAJQ2FuY2VsbGVkAAAA",
        "AAAAAQAAABZQbGF0Zm9ybSBjb25maWd1cmF0aW9uAAAAAAAAAAAADlBsYXRmb3JtQ29uZmlnAAAAAAAGAAAADlBsYXRmb3JtIGFkbWluAAAAAAAFYWRtaW4AAAAAAAATAAAALlBsYXRmb3JtIGZlZSBpbiBiYXNpcyBwb2ludHMgKGUuZy4sIDUwID0gMC41JSkAAAAAAAdmZWVfYnBzAAAAAAQAAAAVRmVlIGNvbGxlY3RvciBhZGRyZXNzAAAAAAAADWZlZV9jb2xsZWN0b3IAAAAAAAATAAAAKU1heGltdW0gY29udmVyc2lvbiBhbW91bnQgcGVyIHRyYW5zYWN0aW9uAAAAAAAAFW1heF9jb252ZXJzaW9uX2Ftb3VudAAAAAAAAAsAAAAZTWluaW11bSBjb252ZXJzaW9uIGFtb3VudAAAAAAAABVtaW5fY29udmVyc2lvbl9hbW91bnQAAAAAAAALAAAAHVJhdGUgbG9jayBkdXJhdGlvbiBpbiBzZWNvbmRzAAAAAAAAEnJhdGVfbG9ja19kdXJhdGlvbgAAAAAABg==",
        "AAAAAgAAAClFdmVudHMgZW1pdHRlZCBieSB0aGUgY29udmVyc2lvbiBjb250cmFjdAAAAAAAAAAAAAAPQ29udmVyc2lvbkV2ZW50AAAAAAQAAAABAAAAIUNvbnZlcnNpb24gY29tcGxldGVkIHN1Y2Nlc3NmdWxseQAAAAAAABNDb252ZXJzaW9uQ29tcGxldGVkAAAAAAcAAAARAAAAEwAAB9AAAAAIQ3VycmVuY3kAAAfQAAAACEN1cnJlbmN5AAAACwAAAAsAAAALAAAAAQAAABVFeGNoYW5nZSByYXRlIHVwZGF0ZWQAAAAAAAALUmF0ZVVwZGF0ZWQAAAAABAAAB9AAAAAIQ3VycmVuY3kAAAfQAAAACEN1cnJlbmN5AAAACwAAAAYAAAABAAAAG1JhdGUgbG9ja2VkIGZvciB0cmFuc2FjdGlvbgAAAAAKUmF0ZUxvY2tlZAAAAAAABAAAB9AAAAAIQ3VycmVuY3kAAAfQAAAACEN1cnJlbmN5AAAACwAAAAYAAAABAAAADUZlZSBjb2xsZWN0ZWQAAAAAAAAMRmVlQ29sbGVjdGVkAAAAAwAAB9AAAAAIQ3VycmVuY3kAAAALAAAAEw==",
        "AAAAAgAAAB1TdG9yYWdlIGtleXMgZm9yIHRoZSBjb250cmFjdAAAAAAAAAAAAAAHRGF0YUtleQAAAAAGAAAAAAAAABZQbGF0Zm9ybSBjb25maWd1cmF0aW9uAAAAAAAGQ29uZmlnAAAAAAABAAAAH0V4Y2hhbmdlIHJhdGUgZm9yIGN1cnJlbmN5IHBhaXIAAAAABFJhdGUAAAACAAAH0AAAAAhDdXJyZW5jeQAAB9AAAAAIQ3VycmVuY3kAAAABAAAADFVzZXIgYmFsYW5jZQAAAAdCYWxhbmNlAAAAAAEAAAATAAAAAQAAABZDb252ZXJzaW9uIHRyYW5zYWN0aW9uAAAAAAALVHJhbnNhY3Rpb24AAAAAAQAAABEAAAAAAAAAE1RyYW5zYWN0aW9uIGNvdW50ZXIAAAAACVR4Q291bnRlcgAAAAAAAAAAAAAZU3VwcG9ydGVkIGN1cnJlbmNpZXMgbGlzdAAAAAAAABNTdXBwb3J0ZWRDdXJyZW5jaWVzAA==",
        "AAAAAAAAACJJbml0aWFsaXplIHRoZSBjb252ZXJzaW9uIGNvbnRyYWN0AAAAAAAVaW5pdGlhbGl6ZV9jb252ZXJzaW9uAAAAAAAABQAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAAdmZWVfYnBzAAAAAAQAAAAAAAAADWZlZV9jb2xsZWN0b3IAAAAAAAATAAAAAAAAAAptaW5fYW1vdW50AAAAAAALAAAAAAAAAAptYXhfYW1vdW50AAAAAAALAAAAAQAAB9AAAAAOUGxhdGZvcm1Db25maWcAAA==",
        "AAAAAAAAAChVcGRhdGUgZXhjaGFuZ2UgcmF0ZSBmb3IgYSBjdXJyZW5jeSBwYWlyAAAAC3VwZGF0ZV9yYXRlAAAAAAQAAAAAAAAADWZyb21fY3VycmVuY3kAAAAAAAfQAAAACEN1cnJlbmN5AAAAAAAAAAt0b19jdXJyZW5jeQAAAAfQAAAACEN1cnJlbmN5AAAAAAAAAARyYXRlAAAACwAAAAAAAAARdmFsaWRpdHlfZHVyYXRpb24AAAAAAAAGAAAAAQAAB9AAAAAMRXhjaGFuZ2VSYXRl",
        "AAAAAAAAACRMb2NrIGV4Y2hhbmdlIHJhdGUgZm9yIGEgdHJhbnNhY3Rpb24AAAAPY29udmVyc2lvbl9yYXRlAAAAAAIAAAAAAAAADWZyb21fY3VycmVuY3kAAAAAAAfQAAAACEN1cnJlbmN5AAAAAAAAAAt0b19jdXJyZW5jeQAAAAfQAAAACEN1cnJlbmN5AAAAAQAAB9AAAAAMRXhjaGFuZ2VSYXRl",
        "AAAAAAAAABtQZXJmb3JtIGN1cnJlbmN5IGNvbnZlcnNpb24AAAAAEGNvbnZlcnRfY3VycmVuY3kAAAAEAAAAAAAAAAR1c2VyAAAAEwAAAAAAAAANZnJvbV9jdXJyZW5jeQAAAAAAB9AAAAAIQ3VycmVuY3kAAAAAAAAAC3RvX2N1cnJlbmN5AAAAB9AAAAAIQ3VycmVuY3kAAAAAAAAABmFtb3VudAAAAAAACwAAAAEAAAfQAAAADENvbnZlcnNpb25UeA==",
        "AAAAAAAAACNHZXQgdXNlciBiYWxhbmNlIGZvciBhbGwgY3VycmVuY2llcwAAAAAQZ2V0X3VzZXJfYmFsYW5jZQAAAAEAAAAAAAAABHVzZXIAAAATAAAAAQAAB9AAAAALVXNlckJhbGFuY2UA",
        "AAAAAAAAACJHZXQgY29udmVyc2lvbiB0cmFuc2FjdGlvbiBkZXRhaWxzAAAAAAAPZ2V0X3RyYW5zYWN0aW9uAAAAAAEAAAAAAAAABXR4X2lkAAAAAAAAEQAAAAEAAAfQAAAADENvbnZlcnNpb25UeA==",
        "AAAAAAAAABlHZXQgY3VycmVudCBleGNoYW5nZSByYXRlAAAAAAAACGdldF9yYXRlAAAAAgAAAAAAAAANZnJvbV9jdXJyZW5jeQAAAAAAB9AAAAAIQ3VycmVuY3kAAAAAAAAAC3RvX2N1cnJlbmN5AAAAB9AAAAAIQ3VycmVuY3kAAAABAAAH0AAAAAxFeGNoYW5nZVJhdGU=",
        "AAAAAAAAABpHZXQgcGxhdGZvcm0gY29uZmlndXJhdGlvbgAAAAAAFWdldF9jb252ZXJzaW9uX2NvbmZpZwAAAAAAAAAAAAABAAAH0AAAAA5QbGF0Zm9ybUNvbmZpZwAA",
        "AAAAAAAAAAAAAAAHZGVwb3NpdAAAAAADAAAAAAAAAAR1c2VyAAAAEwAAAAAAAAAIY3VycmVuY3kAAAfQAAAACEN1cnJlbmN5AAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
        "AAAAAQAAACZMaXF1aWRpdHkgcG9vbCBmb3IgYSBzcGVjaWZpYyBjdXJyZW5jeQAAAAAAAAAAAA1MaXF1aWRpdHlQb29sAAAAAAAACQAAACNBdmFpbGFibGUgbGlxdWlkaXR5IGZvciBjb252ZXJzaW9ucwAAAAATYXZhaWxhYmxlX2xpcXVpZGl0eQAAAAALAAAAF1Bvb2wgY3JlYXRpb24gdGltZXN0YW1wAAAAAApjcmVhdGVkX2F0AAAAAAAGAAAAFEN1cnJlbmN5IG9mIHRoZSBwb29sAAAACGN1cnJlbmN5AAAH0AAAAAhDdXJyZW5jeQAAABdMYXN0IGFjdGl2aXR5IHRpbWVzdGFtcAAAAAAQbGFzdF9hY3Rpdml0eV9hdAAAAAYAAAAbTWluaW11bSBsaXF1aWRpdHkgdGhyZXNob2xkAAAAABdtaW5fbGlxdWlkaXR5X3RocmVzaG9sZAAAAAALAAAAHU51bWJlciBvZiBsaXF1aWRpdHkgcHJvdmlkZXJzAAAAAAAADnByb3ZpZGVyX2NvdW50AAAAAAAEAAAAMVJlc2VydmVkIGxpcXVpZGl0eSAobG9ja2VkIGluIGFjdGl2ZSBjb252ZXJzaW9ucykAAAAAAAAScmVzZXJ2ZWRfbGlxdWlkaXR5AAAAAAALAAAAG1RvdGFsIGxpcXVpZGl0eSBpbiB0aGUgcG9vbAAAAAAPdG90YWxfbGlxdWlkaXR5AAAAAAsAAAAkUG9vbCB1dGlsaXphdGlvbiByYXRlIChiYXNpcyBwb2ludHMpAAAAFHV0aWxpemF0aW9uX3JhdGVfYnBzAAAABA==",
        "AAAAAQAAACZJbmRpdmlkdWFsIGxpcXVpZGl0eSBwcm92aWRlciBwb3NpdGlvbgAAAAAAAAAAABFMaXF1aWRpdHlQb3NpdGlvbgAAAAAAAAgAAAAkQWNjdW11bGF0ZWQgcmV3YXJkcyBmcm9tIGNvbnZlcnNpb25zAAAAE2FjY3VtdWxhdGVkX3Jld2FyZHMAAAAACwAAACJUaW1lc3RhbXAgd2hlbiBsaXF1aWRpdHkgd2FzIGFkZGVkAAAAAAAIYWRkZWRfYXQAAAAGAAAAGEN1cnJlbmN5IG9mIHRoZSBwb3NpdGlvbgAAAAhjdXJyZW5jeQAAB9AAAAAIQ3VycmVuY3kAAAAfTGFzdCB0aW1lIHBvc2l0aW9uIHdhcyBtb2RpZmllZAAAAAAQbGFzdF9tb2RpZmllZF9hdAAAAAYAAAAcQW1vdW50IG9mIGxpcXVpZGl0eSBwcm92aWRlZAAAABBsaXF1aWRpdHlfYW1vdW50AAAACwAAACtMb2NrIHBlcmlvZCBlbmQgdGltZXN0YW1wICgwIGlmIG5vdCBsb2NrZWQpAAAAAApsb2NrX3VudGlsAAAAAAAGAAAAIFNoYXJlIG9mIHRoZSBwb29sIChiYXNpcyBwb2ludHMpAAAADnBvb2xfc2hhcmVfYnBzAAAAAAAEAAAAElByb3ZpZGVyJ3MgYWRkcmVzcwAAAAAACHByb3ZpZGVyAAAAEw==",
        "AAAAAQAAABpQb29sIG1hbmFnZXIgY29uZmlndXJhdGlvbgAAAAAAAAAAABFQb29sTWFuYWdlckNvbmZpZwAAAAAAAAcAAAAVQWRtaW5pc3RyYXRvciBhZGRyZXNzAAAAAAAABWFkbWluAAAAAAAAEwAAACdEZWZhdWx0IGxpcXVpZGl0eSBsb2NrIHBlcmlvZCAoc2Vjb25kcykAAAAAE2RlZmF1bHRfbG9ja19wZXJpb2QAAAAABgAAABRFbWVyZ2VuY3kgcGF1c2UgZmxhZwAAAAlpc19wYXVzZWQAAAAAAAABAAAAJU1heGltdW0gbGlxdWlkaXR5IGFtb3VudCBwZXIgcHJvdmlkZXIAAAAAAAAUbWF4X2xpcXVpZGl0eV9hbW91bnQAAAALAAAAJU1pbmltdW0gbGlxdWlkaXR5IGFtb3VudCBwZXIgcHJvdmlkZXIAAAAAAAAUbWluX2xpcXVpZGl0eV9hbW91bnQAAAALAAAAMlJld2FyZCByYXRlIGZvciBsaXF1aWRpdHkgcHJvdmlkZXJzIChiYXNpcyBwb2ludHMpAAAAAAAYcHJvdmlkZXJfcmV3YXJkX3JhdGVfYnBzAAAABAAAADZQb29sIHV0aWxpemF0aW9uIHRocmVzaG9sZCBmb3Igd2FybmluZ3MgKGJhc2lzIHBvaW50cykAAAAAABd1dGlsaXphdGlvbl93YXJuaW5nX2JwcwAAAAAE",
        "AAAAAgAAABNQb29sIG1hbmFnZXIgZXZlbnRzAAAAAAAAAAAQUG9vbE1hbmFnZXJFdmVudAAAAAcAAAABAAAAF0xpcXVpZGl0eSBhZGRlZCB0byBwb29sAAAAAA5MaXF1aWRpdHlBZGRlZAAAAAAABAAAABMAAAfQAAAACEN1cnJlbmN5AAAACwAAAAQAAAABAAAAG0xpcXVpZGl0eSByZW1vdmVkIGZyb20gcG9vbAAAAAAQTGlxdWlkaXR5UmVtb3ZlZAAAAAQAAAATAAAH0AAAAAhDdXJyZW5jeQAAAAsAAAAEAAAAAQAAACZQb29sIGJhbGFuY2UgdXBkYXRlZCBkdXJpbmcgY29udmVyc2lvbgAAAAAAElBvb2xCYWxhbmNlVXBkYXRlZAAAAAAABAAAB9AAAAAIQ3VycmVuY3kAAAALAAAACwAAAAsAAAABAAAAG0xpcXVpZGl0eSBwcm92aWRlciByZXdhcmRlZAAAAAAQUHJvdmlkZXJSZXdhcmRlZAAAAAMAAAATAAAH0AAAAAhDdXJyZW5jeQAAAAsAAAABAAAAGFBvb2wgdXRpbGl6YXRpb24gd2FybmluZwAAABZQb29sVXRpbGl6YXRpb25XYXJuaW5nAAAAAAACAAAH0AAAAAhDdXJyZW5jeQAAAAQAAAABAAAAGUVtZXJnZW5jeSBwYXVzZSBhY3RpdmF0ZWQAAAAAAAAXRW1lcmdlbmN5UGF1c2VBY3RpdmF0ZWQAAAAAAQAAABMAAAABAAAAG0VtZXJnZW5jeSBwYXVzZSBkZWFjdGl2YXRlZAAAAAAZRW1lcmdlbmN5UGF1c2VEZWFjdGl2YXRlZAAAAAAAAAEAAAAT",
        "AAAAAgAAAB1TdG9yYWdlIGtleXMgZm9yIHBvb2wgbWFuYWdlcgAAAAAAAAAAAAALUG9vbERhdGFLZXkAAAAACAAAAAAAAAAaUG9vbCBtYW5hZ2VyIGNvbmZpZ3VyYXRpb24AAAAAAApQb29sQ29uZmlnAAAAAAABAAAAJExpcXVpZGl0eSBwb29sIGZvciBzcGVjaWZpYyBjdXJyZW5jeQAAAARQb29sAAAAAQAAB9AAAAAIQ3VycmVuY3kAAAABAAAALExpcXVpZGl0eSBwb3NpdGlvbiBmb3IgcHJvdmlkZXIgYW5kIGN1cnJlbmN5AAAACFBvc2l0aW9uAAAAAgAAABMAAAfQAAAACEN1cnJlbmN5AAAAAAAAACFUb3RhbCBsaXF1aWRpdHkgcG9zaXRpb25zIGNvdW50ZXIAAAAAAAAPUG9zaXRpb25Db3VudGVyAAAAAAAAAAAbQWN0aXZlIHBvb2wgY3VycmVuY2llcyBsaXN0AAAAABBBY3RpdmVDdXJyZW5jaWVzAAAAAQAAABhQb29sIHV0aWxpemF0aW9uIGhpc3RvcnkAAAASVXRpbGl6YXRpb25IaXN0b3J5AAAAAAACAAAH0AAAAAhDdXJyZW5jeQAAAAYAAAABAAAAGVByb3ZpZGVyIHJld2FyZHMgdHJhY2tpbmcAAAAAAAAPUHJvdmlkZXJSZXdhcmRzAAAAAAEAAAATAAAAAQAAAC1MaXN0IG9mIGFsbCBwcm92aWRlcnMgZm9yIGEgc3BlY2lmaWMgY3VycmVuY3kAAAAAAAARQ3VycmVuY3lQcm92aWRlcnMAAAAAAAABAAAH0AAAAAhDdXJyZW5jeQ==",
        "AAAAAAAAABtJbml0aWFsaXplIHRoZSBwb29sIG1hbmFnZXIAAAAAF2luaXRpYWxpemVfcG9vbF9tYW5hZ2VyAAAAAAUAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAANbWluX2xpcXVpZGl0eQAAAAAAAAsAAAAAAAAADW1heF9saXF1aWRpdHkAAAAAAAALAAAAAAAAAAtsb2NrX3BlcmlvZAAAAAAGAAAAAAAAAA9yZXdhcmRfcmF0ZV9icHMAAAAABAAAAAEAAAfQAAAAEVBvb2xNYW5hZ2VyQ29uZmlnAAAA",
        "AAAAAAAAACBBZGQgbGlxdWlkaXR5IHRvIGEgY3VycmVuY3kgcG9vbAAAAA1hZGRfbGlxdWlkaXR5AAAAAAAABAAAAAAAAAAIcHJvdmlkZXIAAAATAAAAAAAAAAhjdXJyZW5jeQAAB9AAAAAIQ3VycmVuY3kAAAAAAAAABmFtb3VudAAAAAAACwAAAAAAAAALbG9ja19wZXJpb2QAAAAD6AAAAAYAAAABAAAH0AAAABFMaXF1aWRpdHlQb3NpdGlvbgAAAA==",
        "AAAAAAAAACVSZW1vdmUgbGlxdWlkaXR5IGZyb20gYSBjdXJyZW5jeSBwb29sAAAAAAAAEHJlbW92ZV9saXF1aWRpdHkAAAADAAAAAAAAAAhwcm92aWRlcgAAABMAAAAAAAAACGN1cnJlbmN5AAAH0AAAAAhDdXJyZW5jeQAAAAAAAAAGYW1vdW50AAAAAAALAAAAAQAAB9AAAAARTGlxdWlkaXR5UG9zaXRpb24AAAA=",
        "AAAAAAAAADBVcGRhdGUgcG9vbCBiYWxhbmNlIGR1cmluZyBjb252ZXJzaW9uIG9wZXJhdGlvbnMAAAAZdXBkYXRlX3Bvb2xfb25fY29udmVyc2lvbgAAAAAAAAQAAAAAAAAADWZyb21fY3VycmVuY3kAAAAAAAfQAAAACEN1cnJlbmN5AAAAAAAAAAt0b19jdXJyZW5jeQAAAAfQAAAACEN1cnJlbmN5AAAAAAAAAAtmcm9tX2Ftb3VudAAAAAALAAAAAAAAAAl0b19hbW91bnQAAAAAAAALAAAAAQAAA+0AAAACAAAH0AAAAA1MaXF1aWRpdHlQb29sAAAAAAAH0AAAAA1MaXF1aWRpdHlQb29sAAAA",
        "AAAAAAAAAClEaXN0cmlidXRlIHJld2FyZHMgdG8gbGlxdWlkaXR5IHByb3ZpZGVycwAAAAAAABJkaXN0cmlidXRlX3Jld2FyZHMAAAAAAAIAAAAAAAAACGN1cnJlbmN5AAAH0AAAAAhDdXJyZW5jeQAAAAAAAAAQdG90YWxfZmVlX2Ftb3VudAAAAAsAAAABAAAD6gAAA+0AAAACAAAAEwAAAAs=",
        "AAAAAAAAAB5HZXQgbGlxdWlkaXR5IHBvb2wgaW5mb3JtYXRpb24AAAAAAAhnZXRfcG9vbAAAAAEAAAAAAAAACGN1cnJlbmN5AAAH0AAAAAhDdXJyZW5jeQAAAAEAAAfQAAAADUxpcXVpZGl0eVBvb2wAAAA=",
        "AAAAAAAAACVHZXQgbGlxdWlkaXR5IHBvc2l0aW9uIGZvciBhIHByb3ZpZGVyAAAAAAAADGdldF9wb3NpdGlvbgAAAAIAAAAAAAAACHByb3ZpZGVyAAAAEwAAAAAAAAAIY3VycmVuY3kAAAfQAAAACEN1cnJlbmN5AAAAAQAAB9AAAAARTGlxdWlkaXR5UG9zaXRpb24AAAA=",
        "AAAAAAAAAB5HZXQgcG9vbCBtYW5hZ2VyIGNvbmZpZ3VyYXRpb24AAAAAAA9nZXRfcG9vbF9jb25maWcAAAAAAAAAAAEAAAfQAAAAEVBvb2xNYW5hZ2VyQ29uZmlnAAAA",
        "AAAAAAAAACRHZXQgYWxsIGFjdGl2ZSBjdXJyZW5jaWVzIHdpdGggcG9vbHMAAAAVZ2V0X2FjdGl2ZV9jdXJyZW5jaWVzAAAAAAAAAAAAAAEAAAPqAAAH0AAAAAhDdXJyZW5jeQ==",
        "AAAAAAAAAB1FbWVyZ2VuY3kgcGF1c2UgZnVuY3Rpb25hbGl0eQAAAAAAAA9lbWVyZ2VuY3lfcGF1c2UAAAAAAAAAAAEAAAAB",
        "AAAAAAAAACdSZXN1bWUgb3BlcmF0aW9ucyBhZnRlciBlbWVyZ2VuY3kgcGF1c2UAAAAAEXJlc3VtZV9vcGVyYXRpb25zAAAAAAAAAAAAAAEAAAAB",
        "AAAAAgAAAAAAAAAAAAAACEFwcEVycm9yAAAACAAAAAAAAAAAAAAADUludmFsaWRBbW91bnQAAAAAAAAAAAAAAAAAAA5JbnZhbGlkQWRkcmVzcwAAAAAAAAAAAAAAAAAQSW52YWxpZFRpbWVzdGFtcAAAAAAAAAAAAAAAE0luc3VmZmljaWVudEJhbGFuY2UAAAAAAAAAAAAAAAATVW5zdXBwb3J0ZWRDdXJyZW5jeQAAAAAAAAAAAAAAAAtSYXRlRXhwaXJlZAAAAAAAAAAAAAAAABdDb252ZXJzaW9uTGltaXRFeGNlZWRlZAAAAAAAAAAAAAAAAAxVbmF1dGhvcml6ZWQ=",
        "AAAAAAAAAAAAAAAEaW5pdAAAAAEAAAAAAAAAB2JhY2tlbmQAAAAAEwAAAAA=",
        "AAAAAAAAAAAAAAAKbWludF90b2tlbgAAAAAAAwAAAAAAAAAJcmVjaXBpZW50AAAAAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAV0b2tlbgAAAAAAABMAAAAA",
        "AAAAAQAAAAAAAAAAAAAAC1Rva2VuQ29uZmlnAAAAAAQAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAAIZGVjaW1hbHMAAAAEAAAAAAAAAARuYW1lAAAAEQAAAAAAAAAGc3ltYm9sAAAAAAAR",
        "AAAAAQAAAAAAAAAAAAAAB0JhbGFuY2UAAAAAAQAAAAAAAAAGYW1vdW50AAAAAAAL",
        "AAAAAAAAAAAAAAAQaW5pdGlhbGl6ZV90b2tlbgAAAAQAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAAEbmFtZQAAABEAAAAAAAAABnN5bWJvbAAAAAAAEQAAAAAAAAAIZGVjaW1hbHMAAAAEAAAAAQAAB9AAAAALVG9rZW5Db25maWcA",
        "AAAAAAAAAAAAAAAEbWludAAAAAMAAAAAAAAABm1pbnRlcgAAAAAAEwAAAAAAAAACdG8AAAAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAA=",
        "AAAAAAAAAAAAAAAIdHJhbnNmZXIAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAACdG8AAAAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAA=",
        "AAAAAAAAAAAAAAAHYmFsYW5jZQAAAAABAAAAAAAAAAJvZgAAAAAAEwAAAAEAAAAL",
        "AAAAAAAAAAAAAAAQZ2V0X3Rva2VuX2NvbmZpZwAAAAAAAAABAAAH0AAAAAtUb2tlbkNvbmZpZwA=",
        "AAAAAQAAAAAAAAAAAAAADk11bHRpU2lnQ29uZmlnAAAAAAADAAAAAAAAAAVub25jZQAAAAAAAAQAAAAAAAAAB3NpZ25lcnMAAAAD6gAAABMAAAAAAAAACXRocmVzaG9sZAAAAAAAAAQ=",
        "AAAAAQAAAAAAAAAAAAAAC1RyYW5zYWN0aW9uAAAAAAMAAAAAAAAABW5vbmNlAAAAAAAABAAAAAAAAAAJb3BlcmF0aW9uAAAAAAAD7gAAACAAAAAAAAAACXRpbWVzdGFtcAAAAAAAAAY=",
        "AAAAAAAAAAAAAAATaW5pdGlhbGl6ZV9tdWx0aXNpZwAAAAACAAAAAAAAAAdzaWduZXJzAAAAA+oAAAATAAAAAAAAAAl0aHJlc2hvbGQAAAAAAAAEAAAAAQAAB9AAAAAOTXVsdGlTaWdDb25maWcAAA==",
        "AAAAAAAAAAAAAAATcHJvcG9zZV90cmFuc2FjdGlvbgAAAAADAAAAAAAAAAlvcGVyYXRpb24AAAAAAAPuAAAAIAAAAAAAAAAKc2lnbmF0dXJlcwAAAAAD6gAAA+4AAABAAAAAAAAAAAhwcm9wb3NlcgAAABMAAAABAAAAAQ==",
        "AAAAAAAAAAAAAAATZ2V0X211bHRpc2lnX2NvbmZpZwAAAAAAAAAAAQAAB9AAAAAOTXVsdGlTaWdDb25maWcAAA==",
        "AAAAAAAAAAAAAAAWdXBkYXRlX211bHRpc2lnX2NvbmZpZwAAAAAAAwAAAAAAAAALbmV3X3NpZ25lcnMAAAAD6gAAABMAAAAAAAAADW5ld190aHJlc2hvbGQAAAAAAAAEAAAAAAAAAAhwcm9wb3NlcgAAABMAAAABAAAH0AAAAA5NdWx0aVNpZ0NvbmZpZwAA",
        "AAAABAAAAAAAAAAAAAAADVJhdGVMb2NrRXJyb3IAAAAAAAACAAAAAAAAAAxOb1JhdGVMb2NrZWQAAAABAAAAAAAAAAtSYXRlRXhwaXJlZAAAAAAC",
        "AAAAAAAAAAAAAAAJbG9ja19yYXRlAAAAAAAAAwAAAAAAAAAEdXNlcgAAABMAAAAAAAAABHJhdGUAAAALAAAAAAAAABBkdXJhdGlvbl9zZWNvbmRzAAAABgAAAAA=",
        "AAAAAAAAAAAAAAATdmFsaWRhdGVfY29udmVyc2lvbgAAAAABAAAAAAAAAAR1c2VyAAAAEwAAAAEAAAPpAAAACwAAB9AAAAANUmF0ZUxvY2tFcnJvcgAAAA==",
        "AAAABAAAAAAAAAAAAAAADUNvbnRyYWN0RXJyb3IAAAAAAAABAAAAAAAAAAxJbnZhbGlkTm9uY2UAAAAB",
        "AAAAAAAAAAAAAAAJZ2V0X25vbmNlAAAAAAAAAQAAAAAAAAAEdXNlcgAAABMAAAABAAAABg==",
        "AAAAAAAAAAAAAAAWY2hlY2tfYW5kX3VwZGF0ZV9ub25jZQAAAAAAAgAAAAAAAAAEdXNlcgAAABMAAAAAAAAACGluY29taW5nAAAABgAAAAEAAAPpAAAABgAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAgAAAAAAAAAAAAAABUV2ZW50AAAAAAAABAAAAAEAAAAAAAAADEZlZUNvbGxlY3RlZAAAAAIAAAATAAAACwAAAAEAAAAAAAAADE9mZmVyQ3JlYXRlZAAAAAMAAAAGAAAAEwAAAAsAAAABAAAAAAAAAA1PZmZlckFjY2VwdGVkAAAAAAAAAgAAAAYAAAATAAAAAQAAAAAAAAAOT2ZmZXJDYW5jZWxsZWQAAAAAAAEAAAAG",
        "AAAAAQAAADFSZXByZXNlbnRzIGEgc3dhcCBvZmZlciBpbiB0aGUgY29udHJhY3QncyBzdG9yYWdlAAAAAAAAAAAAAAlTd2FwT2ZmZXIAAAAAAAAGAAAAAAAAAAdjcmVhdG9yAAAAABMAAAAAAAAACmV4cGlyZXNfYXQAAAAAAAYAAAAAAAAADG9mZmVyX2Ftb3VudAAAAAsAAAAAAAAAC29mZmVyX3Rva2VuAAAAABMAAAAAAAAADnJlcXVlc3RfYW1vdW50AAAAAAALAAAAAAAAAA1yZXF1ZXN0X3Rva2VuAAAAAAAAEw==",
        "AAAAAQAAAAAAAAAAAAAAClN3YXBDb25maWcAAAAAAAMAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAAHZmVlX2JwcwAAAAAEAAAAAAAAAA1mZWVfY29sbGVjdG9yAAAAAAAAEw==",
        "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAAAgAAAAAAAAAAAAAABkNvbmZpZwAAAAAAAQAAAAAAAAAQVG90YWxEaXN0cmlidXRlZAAAAAEAAAAT",
        "AAAAAQAAAAAAAAAAAAAAFUZlZURpc3RyaWJ1dGlvbkNvbmZpZwAAAAAAAAUAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAATcmV3YXJkX3Bvb2xfYWRkcmVzcwAAAAATAAAAAAAAAA9yZXdhcmRfcG9vbF9icHMAAAAABAAAAAAAAAAQdHJlYXN1cnlfYWRkcmVzcwAAABMAAAAAAAAADHRyZWFzdXJ5X2JwcwAAAAQ=",
        "AAAAAQAAAAAAAAAAAAAAF1Rva2VuRGlzdHJpYnV0aW9uVG90YWxzAAAAAAIAAAAAAAAADnRvX3Jld2FyZF9wb29sAAAAAAALAAAAAAAAAAt0b190cmVhc3VyeQAAAAAL",
        "AAAAAQAAAAAAAAAAAAAAE0ZlZURpc3RyaWJ1dGVkRXZlbnQAAAAABgAAAAAAAAAJZmVlX3Rva2VuAAAAAAAAEwAAAAAAAAAScmV3YXJkX3Bvb2xfYW1vdW50AAAAAAALAAAAAAAAABByZXdhcmRfcG9vbF9kZXN0AAAAEwAAAAAAAAATdG90YWxfY29sbGVjdGVkX2ZlZQAAAAALAAAAAAAAAA90cmVhc3VyeV9hbW91bnQAAAAACwAAAAAAAAANdHJlYXN1cnlfZGVzdAAAAAAAABM=",
        "AAAAAAAAAAAAAAAPaW5pdGlhbGl6ZV9mZWVzAAAAAAUAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAAQdHJlYXN1cnlfYWRkcmVzcwAAABMAAAAAAAAAE3Jld2FyZF9wb29sX2FkZHJlc3MAAAAAEwAAAAAAAAAMdHJlYXN1cnlfYnBzAAAABAAAAAAAAAAPcmV3YXJkX3Bvb2xfYnBzAAAAAAQAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAASdXBkYXRlX2ZlZXNfY29uZmlnAAAAAAAEAAAAAAAAABB0cmVhc3VyeV9hZGRyZXNzAAAD6AAAABMAAAAAAAAAE3Jld2FyZF9wb29sX2FkZHJlc3MAAAAD6AAAABMAAAAAAAAADHRyZWFzdXJ5X2JwcwAAA+gAAAAEAAAAAAAAAA9yZXdhcmRfcG9vbF9icHMAAAAD6AAAAAQAAAABAAAD6QAAB9AAAAAVRmVlRGlzdHJpYnV0aW9uQ29uZmlnAAAAAAAAAw==",
        "AAAAAAAAAAAAAAAPZ2V0X2ZlZXNfY29uZmlnAAAAAAAAAAABAAAD6QAAB9AAAAAVRmVlRGlzdHJpYnV0aW9uQ29uZmlnAAAAAAAAAw==",
        "AAAAAAAAAMhEaXN0cmlidXRlcyBjb2xsZWN0ZWQgZmVlcyB0byB0cmVhc3VyeSBhbmQgcmV3YXJkIHBvb2xzLgpUaGlzIGZ1bmN0aW9uIHNob3VsZCBiZSBjYWxsZWQgYnkgdGhlIGNvbnRyYWN0IHRoYXQgY29sbGVjdGVkIHRoZSBmZWVzLgpgZmVlX2NvbGxlY3Rvcl9jb250cmFjdGAgaXMgdGhlIGFkZHJlc3MgaG9sZGluZyB0aGUgYHRvdGFsX2ZlZV9hbW91bnRgLgAAAA9kaXN0cmlidXRlX2ZlZXMAAAAAAwAAAAAAAAAJZmVlX3Rva2VuAAAAAAAAEwAAAAAAAAAQdG90YWxfZmVlX2Ftb3VudAAAAAsAAAAAAAAAFmZlZV9jb2xsZWN0b3JfY29udHJhY3QAAAAAABMAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAAVZ2V0X3RvdGFsX2Rpc3RyaWJ1dGVkAAAAAAAAAQAAAAAAAAAFdG9rZW4AAAAAAAATAAAAAQAAB9AAAAAXVG9rZW5EaXN0cmlidXRpb25Ub3RhbHMA" ]),
      options
    )
  }
  public readonly fromJSON = {
    create: this.txFromJSON<EscrowInfo>,
        release: this.txFromJSON<EscrowInfo>,
        refund: this.txFromJSON<EscrowInfo>,
        check_timeout: this.txFromJSON<EscrowInfo>,
        get_escrow: this.txFromJSON<EscrowInfo>,
        get_all_escrows: this.txFromJSON<Array<EscrowInfo>>,
        initiate_dispute: this.txFromJSON<EscrowInfo>,
        resolve_dispute_for_recipient: this.txFromJSON<EscrowInfo>,
        resolve_dispute_for_sender: this.txFromJSON<EscrowInfo>,
        check_dispute_timeout: this.txFromJSON<EscrowInfo>,
        get_dispute_info: this.txFromJSON<Option<DisputeInfo>>,
        can_dispute: this.txFromJSON<boolean>,
        get_escrow_count: this.txFromJSON<u32>,
        escrow_exists: this.txFromJSON<boolean>,
        get_escrows_by_status: this.txFromJSON<Array<EscrowInfo>>,
        get_escrows_by_participant: this.txFromJSON<Array<EscrowInfo>>,
        update_dispute_period: this.txFromJSON<EscrowInfo>,
        initialize: this.txFromJSON<null>,
        set_dispute_fee: this.txFromJSON<null>,
        get_dispute_fee: this.txFromJSON<i128>,
        get_admin: this.txFromJSON<string>,
        transfer_admin: this.txFromJSON<null>,
        set_paused: this.txFromJSON<null>,
        is_paused: this.txFromJSON<boolean>,
        admin_resolve_dispute: this.txFromJSON<EscrowInfo>,
        initialize_conversion: this.txFromJSON<PlatformConfig>,
        update_rate: this.txFromJSON<ExchangeRate>,
        conversion_rate: this.txFromJSON<ExchangeRate>,
        convert_currency: this.txFromJSON<ConversionTx>,
        get_user_balance: this.txFromJSON<UserBalance>,
        get_transaction: this.txFromJSON<ConversionTx>,
        get_rate: this.txFromJSON<ExchangeRate>,
        get_conversion_config: this.txFromJSON<PlatformConfig>,
        deposit: this.txFromJSON<null>,
        initialize_pool_manager: this.txFromJSON<PoolManagerConfig>,
        add_liquidity: this.txFromJSON<LiquidityPosition>,
        remove_liquidity: this.txFromJSON<LiquidityPosition>,
        update_pool_on_conversion: this.txFromJSON<readonly [LiquidityPool, LiquidityPool]>,
        distribute_rewards: this.txFromJSON<Array<readonly [string, i128]>>,
        get_pool: this.txFromJSON<LiquidityPool>,
        get_position: this.txFromJSON<LiquidityPosition>,
        get_pool_config: this.txFromJSON<PoolManagerConfig>,
        get_active_currencies: this.txFromJSON<Array<Currency>>,
        emergency_pause: this.txFromJSON<boolean>,
        resume_operations: this.txFromJSON<boolean>,
        init: this.txFromJSON<null>,
        mint_token: this.txFromJSON<null>,
        initialize_token: this.txFromJSON<TokenConfig>,
        mint: this.txFromJSON<null>,
        transfer: this.txFromJSON<null>,
        balance: this.txFromJSON<i128>,
        get_token_config: this.txFromJSON<TokenConfig>,
        initialize_multisig: this.txFromJSON<MultiSigConfig>,
        propose_transaction: this.txFromJSON<boolean>,
        get_multisig_config: this.txFromJSON<MultiSigConfig>,
        update_multisig_config: this.txFromJSON<MultiSigConfig>,
        lock_rate: this.txFromJSON<null>,
        validate_conversion: this.txFromJSON<Result<i128>>,
        get_nonce: this.txFromJSON<u64>,
        check_and_update_nonce: this.txFromJSON<Result<u64>>,
        initialize_fees: this.txFromJSON<Result<void>>,
        update_fees_config: this.txFromJSON<Result<FeeDistributionConfig>>,
        get_fees_config: this.txFromJSON<Result<FeeDistributionConfig>>,
        distribute_fees: this.txFromJSON<Result<void>>,
        get_total_distributed: this.txFromJSON<TokenDistributionTotals>
  }
}