# Multi-Currency Pool Manager

## Overview

The Pool Manager is a Soroban smart contract module that manages liquidity pools for multiple currencies in the NexaFx platform. It enables liquidity providers to add and remove liquidity, tracks pool balances during conversions, and emits events for all liquidity operations.

## Features

### Core Functionality
- **Multi-Currency Support**: Manage separate liquidity pools for NGN, USD, EUR, GBP, BTC, and ETH
- **Liquidity Management**: Add and remove liquidity with proper validation and lock periods
- **Pool Balance Tracking**: Automatically update pool balances during conversion operations
- **Utilization Monitoring**: Track pool utilization rates and emit warnings when thresholds are exceeded
- **Emergency Controls**: Pause/resume operations for emergency situations

### Data Structures

#### LiquidityPool
- Tracks total, available, and reserved liquidity per currency
- Monitors provider count and utilization rates
- Records creation and activity timestamps

#### LiquidityPosition
- Individual provider positions with currency-specific amounts
- Pool share calculations in basis points
- Lock periods for liquidity withdrawal restrictions
- Accumulated rewards tracking

#### Pool Manager Configuration
- Admin controls and operational parameters
- Liquidity amount limits (min/max per provider)
- Lock periods and reward rates
- Emergency pause functionality

### Key Functions

#### Administrative Functions
- `initialize_pool_manager()`: Initialize the pool manager with configuration
- `emergency_pause()` / `resume_operations()`: Emergency controls
- `distribute_rewards()`: Distribute fees to liquidity providers

#### Liquidity Operations
- `add_liquidity()`: Add liquidity to a currency pool
- `remove_liquidity()`: Remove liquidity (respecting lock periods)
- `update_pool_on_conversion()`: Update balances during conversions

#### Query Functions
- `get_pool()`: Retrieve pool information for a currency
- `get_position()`: Get provider's liquidity position
- `get_active_currencies()`: List all currencies with active pools
- `get_pool_config()`: Retrieve pool manager configuration

### Events

The pool manager emits the following events:
- `LiquidityAdded`: When liquidity is added to a pool
- `LiquidityRemoved`: When liquidity is removed from a pool
- `PoolBalanceUpdated`: When pool balances change during conversions
- `ProviderRewarded`: When rewards are distributed to providers
- `PoolUtilizationWarning`: When utilization exceeds warning thresholds
- `EmergencyPauseActivated/Deactivated`: Emergency state changes

### Testing

Comprehensive tests are provided in `tests/pool_manager_tests.rs` covering:
- Pool initialization and configuration
- Liquidity addition and removal scenarios
- Pool balance updates during conversions
- Emergency pause functionality
- Multi-currency operations
- Error conditions and edge cases

### Integration

The pool manager integrates with the existing NexaFx conversion system:
- Conversion operations update pool balances automatically
- Pool liquidity provides the backing for currency conversions
- Fee collection can be distributed to liquidity providers
- Utilization monitoring helps ensure sufficient liquidity

### Configuration

Default parameters:
- Minimum liquidity: 1 unit (100,000,000 with 8 decimals)
- Maximum liquidity: 10,000 units (1,000,000,000,000 with 8 decimals)
- Default lock period: 24 hours (86,400 seconds)
- Provider reward rate: 0.1% (10 basis points)
- Utilization warning threshold: 80% (8,000 basis points)

### Limitations

Current implementation limitations:
- Pool share recalculation for multiple providers requires manual implementation
- No automatic rebalancing between currency pools
- Simplified reward distribution mechanism
- No slashing or penalty mechanisms for providers

### Future Enhancements

Potential improvements for production use:
- Automated pool share recalculation system
- Cross-currency pool balancing algorithms
- Advanced reward distribution with performance metrics
- Governance mechanisms for parameter changes
- Integration with external price feeds for dynamic pricing
