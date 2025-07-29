# NFT Staking Program

A Solana program built with Anchor for staking NFTs and earning reward tokens over time.

## Overview

This program allows users to stake NFTs from verified collections and earn points that can be claimed as reward tokens. The system includes configurable parameters for staking mechanics and enforces security constraints.

## Features

- Stake NFTs from verified collections
- Earn points over time while staked
- Configurable points per stake and freeze periods
- Maximum stake limits per user
- Freeze period enforcement to prevent immediate unstaking
- Reward token minting and distribution
- Account closure for rent optimization

## Program Structure

### Instructions

- `initialize_config` - Sets up global staking configuration
- `initialize_user` - Creates user staking account
- `stake` - Stakes an NFT with collection verification
- `unstake` - Unstakes an NFT after freeze period, calculates points
- `claim` - Claims accumulated points as reward tokens

### Account Types

- `StakeConfig` - Global configuration (points_per_stake, max_stake, freeze_period)
- `User` - User account (points, amount_staked, bump)
- `StakeAccount` - Individual stake record (owner, mint, last_update, bump)

### Security Features

- Ownership verification for all operations
- Collection verification for staked NFTs
- Freeze period enforcement (prevents immediate unstaking)
- Maximum stake limits per user
- Input validation for all parameters
- PDA-based account addressing

## Development

### Prerequisites

- Rust 1.70+
- Solana CLI 1.18+
- Anchor CLI 0.31.1
- Node.js 18+

### Building

```bash
anchor build
```

### Testing

```bash
# Run all tests
anchor test

# Run specific test suites
yarn test:unit        # Core unit tests
yarn test:workflow    # Advanced workflow tests
yarn test:all         # All test files
```

### Deployment

```bash
# Deploy to localnet
anchor deploy

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Configuration

The program uses configurable parameters set during initialization:

- `points_per_stake` (u8) - Points earned per day per staked NFT
- `max_stake` (u8) - Maximum NFTs a user can stake
- `freeze_period` (u32) - Minimum seconds before unstaking allowed

## Usage

### Initialize System

```typescript
await program.methods
  .initializeConfig(10, 5, 86400) // 10 points/day, max 5 stakes, 1 day freeze
  .accounts({
    admin: adminPublicKey,
    rewardsMint,
    config: configAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Create User Account

```typescript
await program.methods
  .initializeUser()
  .accounts({
    user: userPublicKey,
    userAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .signers([userKeypair])
  .rpc();
```

### Stake NFT

```typescript
await program.methods
  .stake()
  .accounts({
    user: userPublicKey,
    mint: nftMint,
    collection: collectionMint,
    mintAta: userNftTokenAccount,
    metadata: metadataAccount,
    edition: masterEditionAccount,
    config: configAccount,
    stakeAccount,
    userAccount,
    metadataProgram: METADATA_PROGRAM_ID,
    // ... other required accounts
  })
  .signers([userKeypair])
  .rpc();
```

### Unstake NFT

```typescript
await program.methods
  .unstake()
  .accounts({
    user: userPublicKey,
    mint: nftMint,
    collection: collectionMint,
    mintAta: userNftTokenAccount,
    metadata: metadataAccount,
    edition: masterEditionAccount,
    config: configAccount,
    stakeAccount,
    userAccount,
    metadataProgram: METADATA_PROGRAM_ID,
    // ... other required accounts
  })
  .signers([userKeypair])
  .rpc();
```

### Claim Rewards

```typescript
await program.methods
  .claim()
  .accounts({
    user: userPublicKey,
    rewardsMint,
    rewardsAta: userRewardsTokenAccount,
    userAccount,
    config: configAccount,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .signers([userKeypair])
  .rpc();
```

## Account Derivation

The program uses Program Derived Addresses (PDAs) for deterministic account addressing:

```typescript
// Config account
const [configAccount] = PublicKey.findProgramAddressSync(
  [Buffer.from("config")],
  programId
);

// User account
const [userAccount] = PublicKey.findProgramAddressSync(
  [Buffer.from("user"), userPublicKey.toBuffer()],
  programId
);

// Stake account
const [stakeAccount] = PublicKey.findProgramAddressSync(
  [Buffer.from("stake"), nftMint.toBuffer(), configAccount.toBuffer()],
  programId
);

// Rewards mint
const [rewardsMint] = PublicKey.findProgramAddressSync(
  [Buffer.from("rewards_mint"), configAccount.toBuffer()],
  programId
);
```

## Error Handling

The program defines custom error types:

- `IncorrectMint` - Wrong NFT mint provided
- `IncorrectCollection` - NFT not from verified collection
- `CollectionNotVerified` - Collection verification failed
- `MaxStakeReached` - User exceeded maximum stake limit
- `UnstakeDelayNotMet` - Freeze period not elapsed
- `Unauthorized` - Operation not permitted for user
- `NoPointsToClaim` - User has no points to claim
- `InvalidConfiguration` - Invalid configuration parameters
- `ArithmeticOverflow` - Calculation overflow

## Points Calculation

Points are calculated based on staking duration:

```
points_earned = (time_staked_in_seconds / 86400) * points_per_stake
```

Points are accumulated when unstaking and can be claimed as reward tokens with 6 decimal precision.

## Testing

The project includes comprehensive test suites covering:

- Configuration validation
- User account management
- Security constraints
- Error handling
- Edge cases
- Integration workflows

See `TESTING.md` for detailed testing information.

## Security Considerations

- All operations require proper authorization
- NFTs must be from verified collections
- Freeze periods prevent immediate unstaking
- Input validation prevents invalid configurations
- Account derivation ensures proper addressing
- Points validation prevents unauthorized claims

## License

ISC