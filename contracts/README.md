# MultiSig Contract

A Soroban smart contract implementation of a multi-signature wallet that requires a threshold of signatures to execute transactions.

## Features

- Configurable number of signers and threshold
- Transaction replay protection using nonce
- Signature verification and validation
- Support for arbitrary operation execution

## Contract Structure

### MultiSigConfig

```rust
pub struct MultiSigConfig {
    signers: Vec<Address>,    // List of authorized signers
    threshold: u32,           // Required number of signatures
    nonce: u32,              // Transaction counter for replay protection
}
```

### Transaction

```rust
pub struct Transaction {
    operation: Vec<u8>,      // Operation to be executed
    timestamp: u64,          // Transaction timestamp
    nonce: u32,              // Current transaction nonce
}
```

## Testing Instructions

### Prerequisites

- Rust toolchain installed
- Soroban SDK

### Running Tests

To run the test suite:

```bash
cargo test
```

### Test Coverage

The contract includes comprehensive tests covering:

1. **Initialization Tests** (`test_initialize`)
   - Verifies correct initialization with valid signers and threshold
   - Tests error handling for invalid threshold values

2. **Transaction Proposal Tests** (`test_propose_transaction`)
   - Tests successful transaction execution with sufficient signatures
   - Verifies signature validation and threshold requirements
   - Confirms nonce increment after successful execution

3. **Insufficient Signatures Test** (`test_propose_transaction_insufficient_signatures`)
   - Verifies transaction rejection when signatures are below threshold
   - Confirms nonce remains unchanged on failed attempts

4. **Replay Protection Test** (`test_replay_protection`)
   - Ensures same transaction cannot be executed twice
   - Verifies nonce mechanism prevents replay attacks

### Test Scenarios

#### 1. Basic Initialization
```rust
// Initialize with 2-of-3 configuration
let signers = vec![&env, signer1.clone(), signer2.clone(), signer3.clone()];
let config = client.initialize(&signers, &2);
```

#### 2. Transaction Proposal
```rust
// Create and sign a transaction
let operation = vec![&env, 1, 2, 3];
let sig1 = Signature::sign_payload(&env, &signer1, &payload);
let sig2 = Signature::sign_payload(&env, &signer2, &payload);
let signatures = vec![&env, sig1, sig2];

// Submit transaction
let result = client.propose_transaction(&operation, &signatures);
```

### Verification Steps

1. **Contract Initialization**
   - Check if the contract initializes with correct signer configuration
   - Verify threshold validation

2. **Transaction Processing**
   - Confirm signature verification works correctly
   - Verify threshold enforcement
   - Check nonce increment on successful transactions

3. **Security Measures**
   - Verify replay protection mechanism
   - Confirm signature uniqueness validation
   - Test against invalid or unauthorized signers

## Security Considerations

- Always verify the nonce before signing transactions
- Ensure proper key management for signers
- Validate operation data before execution
- Consider gas costs for multiple signature verifications