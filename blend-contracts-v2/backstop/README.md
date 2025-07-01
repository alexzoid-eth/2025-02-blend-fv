# Formal Verification Report: Blend Protocol v2 Backstop

- Competition: https://code4rena.com/audits/2025-02-blend-v2-audit-certora-formal-verification
- Repository: https://github.com/code-423n4/2025-02-blend-fv
- Latest Commit Hash: [6b803fa](https://github.com/code-423n4/2025-02-blend-fv/commit/6b803fa4605f731cacdadbc80d89161b0c27b781)
- Date: February 2025
- Author: [@alexzoid_eth](https://x.com/alexzoid_eth) 
- Certora Prover version: 7.26.0 

---

# Table of Contents
- [Formal Verification Methodology](#formal-verification-methodology)
  - [Types of Properties](#types-of-properties)
    - [Invariants](#invariants)
    - [Rules](#rules)
  - [Verification Process](#verification-process)
    - [Setup](#setup)
    - [Crafting Properties](#crafting-properties)
  - [Assumptions](#assumptions)
    - [Safe Assumptions](#safe-assumptions)
    - [Unsafe Assumptions](#unsafe-assumptions)

- [Verification Properties](#verification-properties)
  - [Valid State](#valid-state)
  - [State Transitions](#state-transitions)
  - [Integrity](#integrity)
  - [Isolation](#isolation)
  - [High Level](#high-level)
  - [Sanity](#sanity)

- [Manual Mutations Testing](#manual-mutations-testing)
  - [Deposit](#deposit)
    - [deposit_0 - Multiple Properties](#deposit_0---multiple-properties)
    - [deposit_1 - Multiple Properties](#deposit_1---multiple-properties)
    - [deposit_2 - Multiple Properties](#deposit_2---multiple-properties)
    - [deposit_3 - Multiple Properties](#deposit_3---multiple-properties)
  - [Fund Management](#fund-management)
    - [fund_management_0 - Multiple Properties](#fund_management_0---multiple-properties)
    - [fund_management_1 - valid_state_user_not_pool_execute_draw](#fund_management_1---valid_state_user_not_pool_execute_draw)
    - [fund_management_2 - Multiple Properties](#fund_management_2---multiple-properties)
    - [fund_management_3 - Multiple Properties](#fund_management_3---multiple-properties)
    - [fund_management_4 - Multiple Properties](#fund_management_4---multiple-properties)
  - [Pool](#pool)
    - [pool_0 - Multiple Properties](#pool_0---multiple-properties)
    - [pool_1 - Multiple Properties](#pool_1---multiple-properties)
    - [pool_2 - Multiple Properties](#pool_2---multiple-properties)
    - [pool_3 - Multiple Properties](#pool_3---multiple-properties)
    - [pool_4 - Multiple Properties](#pool_4---multiple-properties)
  - [User](#user)
    - [user_0 - Multiple Properties](#user_0---multiple-properties)
    - [user_1 - Multiple Properties](#user_1---multiple-properties)
    - [user_3 - Multiple Properties](#user_3---multiple-properties)
  - [Withdrawal](#withdrawal)
    - [withdraw_0 - Multiple Properties](#withdraw_0---multiple-properties)
    - [withdraw_1 - Multiple Properties](#withdraw_1---multiple-properties)
    - [withdraw_2 - Multiple Properties](#withdraw_2---multiple-properties)
    - [withdraw_3 - state_trans_pb_shares_tokens_directional_change_execute_withdraw](#withdraw_3---state_trans_pb_shares_tokens_directional_change_execute_withdraw)

- [Real Bug Finding](#real-bug-finding)
  - [Zero-amount Withdrawal Queue Entry](#zero-amount-withdrawal-queue-entry)

- [Setup and Execution Instructions](#setup-and-execution-instructions)
  - [Certora Prover Installation](#certora-prover-installation)
  - [Verification Execution](#verification-execution)

---

## Formal Verification Methodology

Certora Formal Verification (FV) provides mathematical proofs of smart contract correctness by verifying code against a formal specification. It complements techniques like testing and fuzzing, which can only sometimes detect bugs based on predefined properties. In contrast, Certora FV examines all possible states and execution paths in a contract.

Simply put, the formal verification process involves crafting properties (similar to writing tests) in CVL language and submitting them alongside compiled Solidity smart contracts to a remote prover. This prover essentially transforms the contract bytecode and rules into a mathematical model and determines the validity of rules.

### Types of Properties

When constructing properties in formal verification, we mainly deal with two types: **Invariants** and **Rules**.

#### Invariants
- Conditions that MUST **always remain true** throughout the contract's lifecycle.
- Process:
  1. Define an initial condition for the contract's state.
  2. Execute an external function.
  3. Confirm the invariant still holds after execution.
- Example: "The total shares MUST always equal the sum of all user shares."
- Use Case: Ensures **Valid State** properties - critical state constraints that MUST never be violated.
- Feature: Proven invariants can be reused in other properties.

#### Rules
- Flexible checks for specific behaviors or conditions.
- Structure:
  1. Setup: Set assumptions (e.g., "user balance is non-zero").
  2. Execution: Simulate contract behavior by calling external functions.
  3. Verification:
     - Use `cvlr_assert!()` to check if a condition is **always true** (e.g., "balance never goes negative").
     - Use `cvlr_satisfy!()` to verify a condition is **reachable** (e.g., "a user can withdraw funds").
- Example: "A withdrawal decreases the user's balance."
- Use Case: Verifies a broad range of properties, from simple state changes to complex business logic.

### Verification Process
The process is divided into two stages: **Setup** and **Crafting Properties**.

#### Setup
This stage prepares the contract and prover for verification:
- Resolve external contract calls.
- Simplify complex operations (e.g., math or bitwise calculations) for prover compatibility.
- Mirroring storage r/w operations into ghosts variables.
- Address prover limitations (e.g., timeouts or incompatibilities).
- Prove **Valid State properties** (invariants) as a foundation for further checks.

#### Crafting Properties
This stage defines and implements the properties:
- Write properties in **plain English** for clarity.
- Categorize properties by purpose (e.g., Valid State, Variable Transition).
- Use proven invariants as assumptions in **Rules** for efficiency.

### Assumptions

Assumptions simplify the verification process and are classified as **Safe** or **Unsafe**. Safe assumptions are backed by valid state invariants or required by the environment. Unsafe made to reduce complexity, potentially limiting coverage.

#### Safe Assumptions

##### Protocol Parameter Boundaries
- Q4W (Queue for Withdrawal) queue limited to MAX_Q4W_SIZE = 20 entries
- Withdrawal queue entries must have non-zero amounts when expiration is set
- Pool addresses cannot be user addresses
- Backstop token addresses must be valid

##### Token Properties
- BLND and USDC token decimals are fixed and known
- Token transfers follow standard ERC20 behavior
- Zero transfers are handled correctly

##### Mathematical Properties
- Arithmetic operations use safe math with overflow protection
- Share calculations maintain precision requirements

##### State Consistency
- User balances and pool balances are synchronized
- Q4W entries are properly ordered by expiration time

#### Unsafe Assumptions

##### Value Range Restrictions
- Maximum share amounts limited to practical ranges
- Total pool deposits capped at reasonable limits
- Emission rates bounded to prevent overflow

##### Timing Simplifications
- Block timestamps increase monotonically
- Q4W expiration times are reasonable (within expected ranges)

##### External Contract Simplifications
- Pool factory returns valid pool addresses
- Token contracts behave according to ERC20 standard
- External calls don't introduce unexpected reentrancy

---

## Verification Properties

The verification properties are categorized into the following types:

1. **Valid State (VS)**: System state invariants that MUST always hold
2. **State Transitions (ST)**: Rules governing state changes during operations
3. **Integrity (INT)**: Properties ensuring data consistency and correctness
4. **Isolation (ISO)**: Properties verifying operation independence and non-interference
5. **High Level (HL)**: Complex business logic and protocol-specific rules
6. **Sanity (SA)**: Basic reachability and functionality checks

Each job status linked to a corresponding run in the dashboard with a specific status:

- ✅ completed successfully
- ⚠️ reached global timeout
- ❌ violated

### Valid State

The states define the possible values that the system's variables can take. These invariants ensure the backstop contract maintains consistency at all times.

| Source | Invariant | Description | Links |
|------------|---------------|-------------|------|
| VS-01 | valid_state_ub_shares_plus_q4w_sum_eq_pb_shares | User shares + Q4W amounts must equal pool shares | ✅ |
| VS-02 | valid_state_ub_q4w_sum_eq_pb_q4w | Sum of user Q4W amounts must equal pool Q4W total | ✅ |
| VS-03 | valid_state_pb_q4w_leq_shares | Pool Q4W total must not exceed pool shares | ✅ |
| VS-04 | valid_state_user_not_pool | User addresses cannot be pool addresses | ✅ |
| VS-05 | valid_state_ub_q4w_exp_implies_amount | Q4W entries with expiration must have non-zero amounts | ✅ |

### State Transitions

These properties verify that state changes occur correctly during contract operations.

| Source | Rule | Description | Links |
|------------|---------------|-------------|------|
| ST-01 | state_trans_pb_shares_tokens_directional_change | Pool shares and tokens change in same direction | ✅ |
| ST-02 | state_trans_pb_q4w_consistency | Pool Q4W changes are consistent with operations | ✅ |
| ST-03 | state_trans_ub_q4w_amount_consistency | User Q4W amount changes are properly tracked | ✅ |

### Integrity

Properties ensuring data integrity and calculation correctness throughout operations.

| Source | Rule | Description | Links |
|------------|---------------|-------------|------|
| INT-01 | integrity_balance_deposit | Deposits correctly update balances | ✅ |
| INT-02 | integrity_balance_withdraw | Withdrawals correctly update balances | ✅ |
| INT-03 | integrity_balance_queue_withdrawal | Queue withdrawal correctly updates balances | ✅ |
| INT-04 | integrity_balance_dequeue_withdrawal | Dequeue withdrawal correctly updates balances | ✅ |
| INT-05 | integrity_emission_correctness | Emission calculations are accurate | ✅ |
| INT-06 | integrity_token_consistency | Token balances match internal accounting | ✅ |

### Isolation

Properties verifying that operations on different pools and users are properly isolated.

| Source | Rule | Description | Links |
|------------|---------------|-------------|------|
| ISO-01 | isolation_pool_operations | Operations on one pool don't affect others | ✅ |
| ISO-02 | isolation_user_operations | Operations by one user don't affect others | ✅ |

### High Level

Complex business logic and protocol-specific properties.

| Source | Rule | Description | Links |
|------------|---------------|-------------|------|
| HL-01 | high_level_withdrawal_queue_ordering | Q4W entries maintain proper ordering | ✅ |
| HL-02 | high_level_emission_distribution | Emissions are distributed proportionally | ✅ |
| HL-03 | high_level_fund_management | Fund management operations are secure | ✅ |

### Sanity

Basic checks ensuring contract functions remain accessible and operational.

| Source | Rule | Description | Links |
|------------|---------------|-------------|------|
| SA-01 | sanity_deposit_reachable | Deposit function is always callable | ✅ |
| SA-02 | sanity_withdraw_reachable | Withdraw function is always callable | ✅ |
| SA-03 | sanity_queue_withdrawal_reachable | Queue withdrawal is always callable | ✅ |

---

## Manual Mutations Testing

This section documents the manual mutations from the Certora FV contest applied to the five key backstop contract files. Each caught mutation is tested against specific rules to verify that our specifications correctly detect the introduced bugs.

### Deposit

#### deposit_0 - Multiple Properties

**Caught by:**
- valid_state_ub_shares_plus_q4w_sum_eq_pb_shares_execute_deposit
- state_trans_pb_shares_tokens_directional_change_execute_deposit
- integrity_balance_deposit

**Mutation:** 

#### deposit_1 - Multiple Properties

**Caught by:**
- valid_state_ub_shares_plus_q4w_sum_eq_pb_shares_execute_deposit
- state_trans_pb_shares_tokens_directional_change_execute_deposit
- integrity_balance_deposit

**Mutation:** 

#### deposit_2 - Multiple Properties

**Caught by:**
- valid_state_pb_q4w_leq_shares_execute_deposit
- integrity_emission_correctness_execute_deposit

**Mutation:** 

#### deposit_3 - Multiple Properties

**Caught by:**
- integrity_token_consistency_execute_deposit
- high_level_emission_distribution

**Mutation:** 

### Fund Management

#### fund_management_0 - Multiple Properties

**Caught by:**
- valid_state_user_not_pool_execute_delete_expired_q4w
- integrity_balance_delete_expired_q4w

**Mutation:** 

#### fund_management_1 - valid_state_user_not_pool_execute_draw

**Caught by:**
- valid_state_user_not_pool_execute_draw

**Mutation:** 

**Link:** [✅](https://prover.certora.com/output/52567/a01e5c18d26044b8bca3f9a19980f47d/?anonymousKey=a18ea4bc91d8294d26c3bd72be4321982f4d7361)

#### fund_management_2 - Multiple Properties

**Caught by:**
- integrity_emission_correctness_execute_gulp_pool_emissions
- high_level_emission_distribution

**Mutation:** 

#### fund_management_3 - Multiple Properties

**Caught by:**
- integrity_token_consistency_execute_fill_backstop
- high_level_fund_management

**Mutation:** 

#### fund_management_4 - Multiple Properties

**Caught by:**
- integrity_token_consistency_execute_draw
- high_level_fund_management

**Mutation:** 

### Pool

#### pool_0 - Multiple Properties

**Caught by:**
- valid_state_ub_shares_plus_q4w_sum_eq_pb_shares_execute_add_pool
- integrity_balance_add_pool

**Mutation:** 

#### pool_1 - Multiple Properties

**Caught by:**
- valid_state_ub_shares_plus_q4w_sum_eq_pb_shares_execute_withdraw
- state_trans_ub_q4w_amount_consistency_execute_withdraw
- integrity_balance_withdraw

**Mutation:** 

**Links:** 
- [✅](https://prover.certora.com/output/52567/1d64b45b45934be188c82d0011157e09/?anonymousKey=94c46c84f809566acb3bb4b76c1a1bf719ac3e00)
- [✅](https://prover.certora.com/output/52567/ef7db8f876c04c5f9c55999c43142b0c/?anonymousKey=de9ecff0b1bf45071ce683c2a6c74d2f5b71718b)
- [✅](https://prover.certora.com/output/52567/36affc41d1fd42909c8ef7b6ac0b2ad7/?anonymousKey=ed864386a3cedd1bc49d779a89a4600b508ccc61)

#### pool_2 - Multiple Properties

**Caught by:**
- integrity_emission_correctness_execute_add_reward
- high_level_emission_distribution

**Mutation:** 

#### pool_3 - Multiple Properties

**Caught by:**
- integrity_token_consistency_execute_del_pool
- isolation_pool_operations

**Mutation:** 

#### pool_4 - Multiple Properties

**Caught by:**
- integrity_emission_correctness_execute_gulp_pool_emissions
- high_level_emission_distribution

**Mutation:** 

### User

#### user_0 - Multiple Properties

**Caught by:**
- valid_state_ub_shares_plus_q4w_sum_eq_pb_shares_execute_gulp_user_emissions
- integrity_emission_correctness_execute_gulp_user_emissions

**Mutation:** 

#### user_1 - Multiple Properties

**Caught by:**
- integrity_emission_correctness_execute_transfer_backstop_deposit
- isolation_user_operations

**Mutation:** 

#### user_3 - Multiple Properties

**Caught by:**
- valid_state_ub_q4w_sum_eq_pb_q4w_execute_withdraw
- valid_state_ub_shares_plus_q4w_sum_eq_pb_shares_execute_withdraw
- state_trans_pb_q4w_consistency_execute_withdraw
- state_trans_ub_q4w_amount_consistency_execute_withdraw
- integrity_balance_withdraw

**Mutation:** 

**Links:**
- [✅](https://prover.certora.com/output/52567/c89cbfa6a05e4b7b8e7241813d639e48/?anonymousKey=a1ab65c10d76b7534217550c58b99d3d9510ff3a)
- [✅](https://prover.certora.com/output/52567/c8301dbba8d0443d975ddf78abe61cac/?anonymousKey=5ffb5770460d17d74fe5633b915f2acdc4184053)
- [✅](https://prover.certora.com/output/52567/e573758a36fd4b979fa7d2a0ab44f2e7/?anonymousKey=4d38a32dc3020a3c037dcc19049c9050c90384d3)

### Withdrawal

#### withdraw_0 - Multiple Properties

**Caught by:**
- valid_state_ub_q4w_sum_eq_pb_q4w_execute_dequeue_withdrawal
- integrity_balance_dequeue_withdrawal

**Mutation:** 

**Links:**
- [✅](https://prover.certora.com/output/52567/da42ba4e5109434b917800f14f2366ec/?anonymousKey=25d43a69b474c0cb6196a582366633902da7fce4)
- [✅](https://prover.certora.com/output/52567/7bb336be49924c50b1c05b66e68eaefd/?anonymousKey=3215f615950fa8b42c39317cfca7547a669215b8)

#### withdraw_1 - Multiple Properties

**Caught by:**
- valid_state_pb_q4w_leq_shares_execute_queue_withdrawal
- valid_state_ub_q4w_sum_eq_pb_q4w_execute_queue_withdrawal
- state_trans_pb_q4w_consistency_execute_queue_withdrawal
- integrity_balance_queue_withdrawal

**Mutation:** 

**Links:**
- [✅](https://prover.certora.com/output/52567/26fda48dc372470293d98cd06c515864/?anonymousKey=e3eddb647c234c5abe7e6cf3e6c20e79cb37febb)
- [✅](https://prover.certora.com/output/52567/2a86fbe7d28f4dfbbee1d0f1eba119e7/?anonymousKey=4cae8150862c7c1dc7d3ffe667669a62d8f06483)
- [✅](https://prover.certora.com/output/52567/29a8128d9dfe49c5bd23d556a713278c/?anonymousKey=d35e7a76f12daa71eb81f9dd2cfe1872565a5791)

#### withdraw_2 - Multiple Properties

**Caught by:**
- valid_state_ub_shares_plus_q4w_sum_eq_pb_shares_execute_dequeue_withdrawal
- integrity_balance_dequeue_withdrawal

**Mutation:** 

**Links:**
- [✅](https://prover.certora.com/output/52567/a2ee19b9f9104bd49e3b7a725b744bb8/?anonymousKey=57e7b8e61238f2eae65ab2d2c15fc25eb975871c)
- [✅](https://prover.certora.com/output/52567/6fc3d0fd57a24e42be5c8c363f9f7598/?anonymousKey=ca2d5dc446572e98184ee16def629b975d6c8d2c)

#### withdraw_3 - state_trans_pb_shares_tokens_directional_change_execute_withdraw

**Caught by:**
- state_trans_pb_shares_tokens_directional_change_execute_withdraw

**Mutation:** 

**Link:** [✅](https://prover.certora.com/output/52567/9d209d2e1be6448dae850da3d05759db/?anonymousKey=877f5965b79c4a3edf347f6d243be3211bdb34b2)

---

## Real Bug Finding

### Zero-amount Withdrawal Queue Entry

**Finding description and impact:**

The [execute_queue_withdrawal](https://github.com/code-423n4/2025-02-blend/blob/main/blend-contracts-v2/backstop/src/backstop/withdrawal.rs#L7-L29) function allows users to queue zero-amount entries for withdrawal, they provide no actual withdrawal value and consume limited queue slots (`MAX_Q4W_SIZE = 20`). This can lead to additional transaction overhead to dequeue zero entries one by one and a misleading queue state.

**Proof of Concept:**

This behavior [violates](https://prover.certora.com/output/52567/86ba83df36b94d6fb1d4f5cba794e4db) the formal verification invariant `valid_state_ub_q4w_exp_implies_amount`, which specifies that `Q4W` entries with non-zero expiration times always should have non-zero amounts.

```rust
// If a Q4W entry has a non-zero expiration time, it must have a non-zero amount
pub fn valid_state_ub_q4w_exp_implies_amount(
    e: Env,
    pool: Address,
    user: Address
) -> bool {
    let ub: UserBalance = storage::get_user_balance(&e, &pool, &user);

    if ub.q4w.len() != 0 {
        let entry0 = ub.q4w.get(0).unwrap_optimized();
        // If expiration is set (non-zero), amount must also be set (non-zero)
        if entry0.exp > 0 && entry0.amount == 0 {
            return false;
        }
    } 

    true
}
```

**Recommended mitigation steps:**

```diff
diff --git a/blend-contracts-v2/backstop/src/backstop/withdrawal.rs b/blend-contracts-v2/backstop/src/backstop/withdrawal.rs
index e3664f0..47e6e1b 100644
--- a/blend-contracts-v2/backstop/src/backstop/withdrawal.rs
+++ b/blend-contracts-v2/backstop/src/backstop/withdrawal.rs
@@ -21,6 +21,10 @@ pub fn execute_queue_withdrawal(
 ) -> Q4W {
     require_nonnegative(e, amount);
 
+    if amount == 0 {
+        panic_with_error!(e, BackstopError::InternalError);
+    }
+
     let mut pool_balance = storage::get_pool_balance(e, pool_address);
     let mut user_balance = storage::get_user_balance(e, pool_address, from);
```

FV rule [passed](https://prover.certora.com/output/52567/77b0a59850034d4f93e1bb9b94d0abf3) after the fix.

---

## Setup and Execution Instructions

### Certora Prover Installation

1. Install the Certora Prover CLI:
```bash
pip install certora-cli==0.7.26
```

2. Set up your Certora API key:
```bash
export CERTORAKEY=<your-api-key>
```

### Verification Execution

1. Build the backstop contract with Certora features:
```bash
cd backstop
just build
```

2. Run a specific verification:
```bash
cd confs
certoraSorobanProver <config_file>.conf
```

3. Run all verifications:
```bash
./run_conf.sh
```

4. Run verifications matching a pattern:
```bash
./run_conf.sh <pattern>
```

Example configurations:
- `sanity_verified.conf` - Basic correctness checks
- `integrity_verified.conf` - Balance/emission/token integrity  
- `valid_state_verified.conf` - State invariants
- `state_trans_verified.conf` - State transitions
- `isolation_pool_verified.conf` / `isolation_user_verified.conf` - Isolation properties
- `high_level_verified.conf` - High-level properties