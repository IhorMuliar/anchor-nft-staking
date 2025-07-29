# Testing Guide

This document provides comprehensive information about testing the NFT Staking program.

## Test Structure

The testing suite is organized into multiple test files, each focusing on different aspects:

### 1. `tests/anchor-nft-staking.ts` - Core Unit Tests
- **Configuration Tests**: Validates config initialization and parameter validation
- **User Account Tests**: Tests user account creation and management
- **Staking Tests**: Mock tests for staking functionality (requires metadata setup)
- **Unstaking Tests**: Mock tests for unstaking and constraint validation
- **Rewards Tests**: Tests claim functionality and validation
- **Security Tests**: Tests authorization, ownership, and account derivation
- **Edge Cases**: Tests maximum values, account space, and rent calculations
- **Integration Tests**: End-to-end workflow validation

### 2. `tests/staking-workflow.ts` - Advanced Workflow Tests
- **Complete Staking Flow**: Step-by-step workflow validation
- **Reward System Tests**: Token calculation and claim validation
- **Error Handling Tests**: Comprehensive error scenario testing
- **Performance Tests**: Gas optimization and rent calculation tests
- **State Consistency Tests**: Account state and upgradability validation
- **Documentation Examples**: Live examples of user journeys and security features

## Running Tests

### Prerequisites
```bash
# Install dependencies
yarn install

# Ensure Solana test validator is running
solana-test-validator
```

### Test Commands

```bash
# Run all tests (full anchor test suite)
yarn test

# Run only unit tests
yarn test:unit

# Run only workflow tests  
yarn test:workflow

# Run all test files
yarn test:all

# Build program before testing
yarn build
```

### Individual Test Categories

```bash
# Configuration and setup tests
yarn test:unit --grep "Configuration Tests"

# User account management tests
yarn test:unit --grep "User Account Tests"

# Security and authorization tests
yarn test:unit --grep "Security Tests"

# Reward system tests
yarn test:workflow --grep "Reward System Tests"

# Error handling tests
yarn test:workflow --grep "Error Handling Tests"
```

## Test Coverage

### ✅ Currently Tested
- ✅ Configuration initialization with valid/invalid parameters
- ✅ User account creation and validation
- ✅ Ownership verification and authorization
- ✅ Account derivation validation (PDA seeds)
- ✅ Error handling for all error types
- ✅ Points calculation logic simulation
- ✅ Freeze period constraint validation
- ✅ Reward token calculation
- ✅ Claim validation (with/without points)
- ✅ Account space and rent calculations
- ✅ Multi-user isolation
- ✅ State consistency validation
- ✅ Edge cases and maximum values

### 🚧 Requires Full NFT Metadata Setup
- 🚧 Actual NFT staking (requires Metaplex metadata)
- 🚧 Collection verification
- 🚧 NFT freezing/thawing
- 🚧 Real staking/unstaking flow
- 🚧 Time-based points accumulation

## Security Test Coverage

### Access Control
- ✅ User can only initialize their own accounts
- ✅ Invalid account derivations are rejected
- ✅ Wrong signers are rejected
- ✅ Authorization checks for all operations

### Constraint Validation
- ✅ Max stake limits enforced
- ✅ Freeze period validation
- ✅ Points validation before claiming
- ✅ Configuration parameter validation

### Account Safety
- ✅ Account rent calculations
- ✅ Account closure and rent refunds
- ✅ PDA derivation validation
- ✅ Account space optimization

## Test Results Interpretation

### Expected Outputs

**Successful Configuration Test:**
```
✓ Config loaded: { pointsPerStake: 10, maxStake: 3, freezePeriod: 5 }
✓ All account states are consistent
```

**Security Test Success:**
```
✓ Correctly rejected invalid account derivation
✓ Multi-user accounts are properly isolated
✓ Account derivation is correctly validated
```

**Workflow Validation:**
```
✓ Complete workflow simulation shows proper state flow
✓ All security features are functioning
✓ Gas optimizations are in place
```

### Common Issues and Solutions

**Issue: "InvalidConfiguration" error**
- **Cause**: Zero values in configuration parameters
- **Solution**: All config parameters must be > 0

**Issue: "NoPointsToClaim" error**
- **Cause**: User has 0 points when trying to claim
- **Solution**: User must stake and unstake to accumulate points first

**Issue: Account derivation errors**
- **Cause**: Incorrect PDA seeds or program ID mismatch
- **Solution**: Verify all PDA derivations use correct seeds

**Issue: Signature verification failures**
- **Cause**: Wrong signer or missing required signatures
- **Solution**: Ensure correct keypairs are used as signers

## Performance Benchmarks

### Transaction Costs (Estimated)
- **Initialize Config**: ~0.002 SOL (rent + gas)
- **Initialize User**: ~0.001 SOL (rent + gas)  
- **Stake NFT**: ~0.003 SOL (rent + gas + CPI costs)
- **Unstake NFT**: Account closed, rent refunded
- **Claim Rewards**: ~0.001 SOL (gas + ATA creation if needed)

### Account Sizes
- **StakeConfig**: 8 + 13 bytes = 21 bytes
- **User**: 8 + 9 bytes = 17 bytes
- **StakeAccount**: 8 + 41 bytes = 49 bytes

## Advanced Testing

### Custom Test Scenarios

You can create custom test scenarios by modifying the test parameters:

```typescript
// Custom configuration for testing
const CUSTOM_CONFIG = {
  POINTS_PER_STAKE: 50,    // Custom points per day
  MAX_STAKE: 20,           // Custom max stakes
  FREEZE_PERIOD: 3600,     // Custom freeze period (1 hour)
};
```

### Mock Data Generation

The tests include helper functions for generating test data:

```typescript
// Create test NFT
const { mint, tokenAccount } = await createTestNFT(user);

// Setup test accounts
await setupAccounts();

// Airdrop SOL for testing
await airdropSol(user.publicKey, 5); // 5 SOL
```

### Debugging Tests

Enable verbose logging:

```bash
# Run with detailed logs
ANCHOR_LOG=true yarn test:unit

# Run specific test with timeout
yarn test:unit --timeout 300000 --grep "specific test name"
```

## Integration with CI/CD

### GitHub Actions Example

```yaml
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup Node
        uses: actions/setup-node@v2
        with:
          node-version: '18'
      - name: Install Solana
        run: |
          curl -sSfL https://release.solana.com/v1.18.0/install | sh
          echo "$HOME/.local/share/solana/install/active_release/bin" >> $GITHUB_PATH
      - name: Install Anchor
        run: npm install -g @coral-xyz/anchor-cli@0.31.1
      - name: Install dependencies
        run: yarn install
      - name: Build program
        run: anchor build
      - name: Run tests
        run: yarn test:all
```

## Test Maintenance

### Adding New Tests

1. **For new instructions**: Add tests to `anchor-nft-staking.ts`
2. **For workflow changes**: Add tests to `staking-workflow.ts`
3. **For security concerns**: Add to security test section
4. **For performance**: Add to performance test section

### Test Data Cleanup

Tests automatically clean up by:
- Using temporary keypairs for each test
- Closing accounts where appropriate
- Resetting state between test runs

### Updating Tests

When modifying the program:
1. Update relevant test cases
2. Add new error types to error handling tests
3. Update expected account structures
4. Verify all existing tests still pass

## Best Practices

### Test Organization
- Keep tests focused and atomic
- Use descriptive test names
- Group related tests in describe blocks
- Use helper functions to reduce duplication

### Test Data Management
- Generate fresh test data for each test
- Clean up accounts when possible
- Use realistic but efficient test parameters

### Performance Considerations
- Use shorter freeze periods for testing (5-10 seconds)
- Minimize unnecessary account creation
- Batch similar operations when possible

### Security Testing
- Always test negative cases (should fail scenarios)
- Verify all error conditions
- Test boundary conditions and edge cases
- Validate all access controls