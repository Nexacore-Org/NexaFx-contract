# NexaFx Soroban Contracts

This repository contains Soroban smart contracts for NexaFx, focusing on secure multi-signature and escrow functionality.

## Contracts Overview

### 1. MultiSig Contract

A multi-signature wallet that requires a threshold of signatures to execute transactions.

#### Features
- Configurable number of signers and threshold
- Transaction replay protection using nonce
- Signature verification and validation
- Support for arbitrary operation execution

### 2. Escrow Contract

A secure escrow mechanism for token transfers with timeout functionality.

#### Features
- Lock funds until specific conditions are met
- Auto-release after a configurable timeout period
- Refund path for returning funds to sender
- Complete state tracking (Active, Released, Refunded, AutoReleased)

## Testing 

To run the test suite:

```bash
cargo test
```

## Security Considerations

- All critical operations require appropriate authentication
- Timeout mechanism in the escrow contract prevents funds from being locked indefinitely
- Replay protection in the multisig contract prevents transaction replay attacks
- State management prevents double-spending scenarios

## Requirements

- Rust toolchain
- Soroban SDK v22.0.7