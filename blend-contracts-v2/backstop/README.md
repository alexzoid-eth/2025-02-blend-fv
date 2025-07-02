# Formal Verification Report: Blend Protocol v2 Backstop

- Competition: https://code4rena.com/audits/2025-02-blend-v2-audit-certora-formal-verification
- Repository: https://github.com/code-423n4/2025-02-blend-fv
- Latest Commit Hash: [6b803fa](https://github.com/code-423n4/2025-02-blend-fv/commit/6b803fa4605f731cacdadbc80d89161b0c27b781)
- Scope: [blend-contracts-v2/backstop](https://github.com/code-423n4/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop)
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

Simply put, the formal verification process involves crafting properties (similar to writing tests) in native RUST language and submitting them alongside compiled programs to a remote prover. This prover essentially transforms the program bytecode and rules into a mathematical model and determines the validity of rules.

### Types of Properties

When constructing properties in formal verification, we mainly deal with two types: **Invariants** and **Rules**. Invariants are implemented in parametric style (one property for each external function) with `parametric_rule!()` macros. 

The structure of parametric rule:
- Initialize ghost storage from rule parameters (this a hack to reduce complexity of storage infractions)
- Assume realistic timestamp
- Assume valid state invariants hold
- Log all storage variables
- Execute external function
- Log all storage variables again

#### Invariants
- Conditions that MUST **always remain true** throughout the contract's lifecycle. Implemented with `invariant_rule!()`. Similar to parametric rules, but check the property hold with `cvlr_assert!()` macros. 
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
  1. Setup: Set valid state assumptions (e.g., "user balance is non-zero") with `init_verification!()` macros.
  2. Execution: Simulate contract behavior by calling external functions.
  3. Verification:
     - Use `cvlr_assert!()` to check if a condition is **always true** (e.g., "balance never goes negative").
     - Use `cvlr_satisfy!()` to verify a condition is **reachable** (e.g., "a user can withdraw funds").
- Example: "A withdrawal decreases the user's balance."
- Use Case: Verifies a broad range of properties, from simple state changes to complex business logic.

### Verification Process
The process is divided into two stages: **Setup** and **Crafting Properties**.

#### Setup
This stage prepares the contract and prover for verification. Use conditional source code compilation with `just features`.
- Resolve external contract calls, declare mocks in `mocks` and `summaries` directory (with `certora_token_mock`, `certora_pool_factory_mock` and `certora_emission_summarized` features)
- Simplify complex operations (mirroring storage r/w operations into ghosts variables with `certora_storage_ghost` feature, vector interactions with `certora_vec_q4w`) to reduce timeouts.
- Prove **Valid State properties** (invariants) as a foundation for further checks.

#### Crafting Properties
This stage defines and implements the properties:
- Write properties in **plain English** for clarity.
- Categorize properties by purpose (e.g., Valid State, Variable Transition). Some of them are implemented in parametric style (`valid_state.rs`, `state_trans.rs`, `sanity.rs`, `isolation.rs`), while others (`integrity_*.rs`, `high_level.rs`) as regular rules. 
- Use proven valid state invariants as assumptions in **Rules** for efficiency.

### Assumptions

Assumptions simplify the verification process and are classified as **Safe** or **Unsafe**. Safe assumptions are backed by valid state invariants or required by the environment. Unsafe made to reduce complexity, potentially limiting coverage.

#### Safe Assumptions

##### Timestamp Constraints
- Block timestamps are always non-zero (`e.ledger().timestamp() > 0`)

##### Valid State Invariants
These invariants are proven to always hold and can be safely assumed:

**Non-negative Value Invariants:**
- `valid_state_nonnegative_pb_shares`: Pool balance shares are non-negative
- `valid_state_nonnegative_pb_tokens`: Pool balance tokens are non-negative
- `valid_state_nonnegative_pb_q4w`: Pool balance Q4W amounts are non-negative
- `valid_state_nonnegative_ub_shares`: User balance shares are non-negative
- `valid_state_nonnegative_ub_q4w_amount`: User Q4W entry amounts are non-negative

**Pool Balance Invariants:**
- `valid_state_pb_q4w_leq_shares`: Pool Q4W total does not exceed pool shares

**User Balance Invariants:**
- `valid_state_ub_shares_plus_q4w_sum_eq_pb_shares`: User shares + Q4W amounts equal pool shares
- `valid_state_ub_q4w_sum_eq_pb_q4w`: Sum of user Q4W amounts equals pool Q4W total
- `valid_state_ub_q4w_expiration`: Q4W entry expiration times do not exceed timestamp + Q4W_LOCK_TIME
- `valid_state_ub_q4w_exp_implies_amount`: Q4W entries with expiration have non-zero amounts

**General State Invariants:**
- `valid_state_user_not_pool`: User addresses cannot be pool or contract addresses (zero balance enforced)
- `valid_state_pool_from_factory`: Only factory-deployed pools can have non-zero balances

#### Unsafe Assumptions

##### Mocks and Summaries
- Token contracts mocked with `certora_token_mock` feature
- Pool factory mocked with `certora_pool_factory_mock` feature
- Emission calculations summarized with `certora_emission_summarized` feature

##### Loop Unrolling
- Vector iterations limited to 2 iterations (`loop_iter = 2` in configs)

---

## Verification Properties

The verification properties are categorized into the following types:

1. **Valid State (VS)**: System state invariants that MUST always hold
2. **State Transitions (ST)**: Rules governing state changes during operations
3. **Isolation (ISO)**: Properties verifying operation independence and non-interference
4. **Sanity (SA)**: Basic reachability and functionality checks
5. **High Level (HL)**: Complex business logic and protocol-specific rules
6. **Integrity (INT)**: Properties ensuring data consistency and correctness

Each job status linked to a corresponding run in the dashboard with a specific status:

- ✅ completed successfully
- ⚠️ reached global timeout
- ❌ violated

### Valid State

The states define the possible values that the system's variables can take. These invariants ensure the backstop contract maintains consistency at all times.

| Source | Invariant | Description | Caught mutations | Links |
|------------|---------------|-------------|------------------|------|
| [VS-01](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_nonnegative_pb_shares | Pool balance shares are non-negative | - | ✅ |
| [VS-02](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_nonnegative_pb_tokens | Pool balance tokens are non-negative | [deposit_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/deposit/deposit_3.rs), [fundmanagement_2](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/fundmanagement/fundmanagement_2.rs), [fundmanagement_4](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/fundmanagement/fundmanagement_4.rs) | ✅ |
| [VS-03](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_nonnegative_pb_q4w | Pool balance Q4W amounts are non-negative | [pool_4](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_4.rs) | ✅ |
| [VS-04](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_nonnegative_ub_shares | User balance shares are non-negative | - | ✅ |
| [VS-05](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_nonnegative_ub_q4w_amount | User Q4W entry amounts are non-negative | - | ✅ |
| [VS-06](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_pb_q4w_leq_shares | Pool Q4W total must not exceed pool shares | [pool_2](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_2.rs), [withdraw_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_1.rs) | ✅ |
| [VS-07](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_ub_shares_plus_q4w_sum_eq_pb_shares | User shares + Q4W amounts must equal pool shares | [deposit_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/deposit/deposit_0.rs), [deposit_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/deposit/deposit_1.rs), [pool_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_1.rs), [user_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_0.rs), [user_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_1.rs), [user_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_3.rs), [withdraw_2](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_2.rs) | ✅ |
| [VS-08](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_ub_q4w_sum_eq_pb_q4w | Sum of user Q4W amounts must equal pool Q4W total | [pool_2](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_2.rs), [pool_4](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_4.rs), [user_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_3.rs), [withdraw_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_0.rs), [withdraw_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_1.rs) | ✅ |
| [VS-09](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_ub_q4w_expiration | Q4W entry expiration times do not exceed timestamp + Q4W_LOCK_TIME | - | ✅ |
| [VS-10](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_ub_q4w_exp_implies_amount | Q4W entries with expiration must have non-zero amounts | - | ✅ |
| [VS-11](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_user_not_pool | User addresses cannot be pool or contract addresses | [fund_management_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/fundmanagement/fund_management_1.rs) | ✅ |
| [VS-12](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/valid_state.rs) | valid_state_pool_from_factory | Only factory-deployed pools can have non-zero balances | - | ✅ |

### State Transitions

These properties verify that state changes occur correctly during contract operations.

| Source | Rule | Description | Caught mutations | Links |
|------------|---------------|-------------|------------------|------|
| [ST-01](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/state_trans.rs) | state_trans_pb_shares_tokens_directional_change | Pool shares and tokens change in same direction | [deposit_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/deposit/deposit_3.rs), [pool_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_0.rs), [pool_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_3.rs), [withdraw_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_3.rs) | ✅ |
| [ST-02](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/state_trans.rs) | state_trans_pb_q4w_consistency | Pool Q4W changes are consistent with operations | [user_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_1.rs), [user_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_3.rs), [withdraw_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_1.rs) | ✅ |
| [ST-03](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/state_trans.rs) | state_trans_ub_shares_increase_consistency | User balance consistency when shares increase | - | ✅ |
| [ST-04](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/state_trans.rs) | state_trans_ub_shares_decrease_consistency | User balance consistency when shares decrease | [pool_4](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_4.rs) | ✅ |
| [ST-05](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/state_trans.rs) | state_trans_ub_q4w_amount_consistency | User Q4W amount changes are properly tracked | [pool_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_1.rs), [pool_2](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_2.rs), [pool_4](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_4.rs), [user_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_1.rs), [user_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_3.rs) | ✅ |

### Isolation

Properties verifying that operations on different pools and users are properly isolated.

| Source | Rule | Description | Caught mutations | Links |
|------------|---------------|-------------|------------------|------|
| [ISO-01](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/isolation.rs) | isolation_pool | Operations on one pool don't affect others | - | ✅ |
| [ISO-02](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/isolation.rs) | isolation_user | Operations by one user don't affect others | - | ✅ |

### Sanity

Basic checks ensuring contract functions remain accessible and operational.

| Source | Rule | Description | Caught mutations | Links |
|------------|---------------|-------------|------------------|------|
| [SA-01](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/sanity.rs) | sanity | All external functions remain callable under valid state | - | ✅ |

### High Level

Complex business logic and protocol-specific properties.

| Source | Rule | Description | Caught mutations | Links |
|------------|---------------|-------------|------------------|------|
| [HL-01](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/high_level.rs) | high_level_deposit_returns_converted_shares | Deposit returns correct share conversion | - | ✅ |
| [HL-02](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/high_level.rs) | high_level_withdrawal_expiration_enforced | Withdrawals can't happen before expiration | - | ✅ |
| [HL-03](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/high_level.rs) | high_level_share_token_initial_conversion | 1:1 conversion when pool is empty | - | ✅ |
| [HL-04](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/high_level.rs) | high_level_share_token_conversion | Consistent token/share conversion rates | - | ✅ |

### Integrity

Properties ensuring data integrity and calculation correctness throughout operations.

#### Balance Integrity

| Source | Rule | Description | Caught mutations | Links |
|------------|---------------|-------------|------------------|------|
| [INT-01](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_balance.rs) | integrity_balance_deposit | Deposits correctly update balances | [deposit_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/deposit/deposit_1.rs), [deposit_2](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/deposit/deposit_2.rs), [deposit_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/deposit/deposit_3.rs), [pool_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_3.rs), [user_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_0.rs) | ✅ |
| [INT-02](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_balance.rs) | integrity_balance_withdraw | Withdrawals correctly update balances | [pool_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_0.rs), [pool_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_1.rs), [pool_2](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_2.rs), [user_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_3.rs) | ✅ |
| [INT-03](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_balance.rs) | integrity_balance_queue_withdrawal | Queue withdrawal correctly updates balances | [pool_4](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_4.rs), [user_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_1.rs), [withdraw_1](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_1.rs) | ✅ |
| [INT-04](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_balance.rs) | integrity_balance_dequeue_withdrawal | Dequeue withdrawal correctly updates balances | [user_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/user/user_0.rs), [withdraw_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_0.rs), [withdraw_2](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/withdraw/withdraw_2.rs) | ✅ |
| [INT-05](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_balance.rs) | integrity_balance_donate | Donations correctly update balances | [fundmanagement_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/fundmanagement/fundmanagement_3.rs), [fundmanagement_4](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/fundmanagement/fundmanagement_4.rs), [pool_3](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_3.rs) | ✅ |
| [INT-06](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_balance.rs) | integrity_balance_draw | Draw operations correctly update balances | [fundmanagement_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/fundmanagement/fundmanagement_0.rs), [pool_0](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/mutations/pool/pool_0.rs) | ✅ |
| [INT-07](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_balance.rs) | integrity_balance_load_pool_backstop_data | Data loading doesn't change state | - | ✅ |

#### Emission Integrity

| Source | Rule | Description | Caught mutations | Links |
|------------|---------------|-------------|------------------|------|
| [INT-08](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_emission.rs) | integrity_emission_deposit | Emission state correct during deposit | - | ✅ |
| [INT-09](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_emission.rs) | integrity_emission_withdraw | Emission state correct during withdraw | - | ✅ |
| [INT-10](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_emission.rs) | integrity_emission_queue_withdrawal | Emission state correct during queue withdrawal | - | ✅ |
| [INT-11](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_emission.rs) | integrity_emission_dequeue_withdrawal | Emission state correct during dequeue withdrawal | - | ✅ |
| [INT-12](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_emission.rs) | integrity_emission_donate | Emission state correct during donate | - | ✅ |
| [INT-13](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_emission.rs) | integrity_emission_draw | Emission state correct during draw | - | ✅ |

#### Token Integrity

| Source | Rule | Description | Caught mutations | Links |
|------------|---------------|-------------|------------------|------|
| [INT-14](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_token.rs) | integrity_token_deposit | Token transfers correct during deposit | - | ✅ |
| [INT-15](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_token.rs) | integrity_token_withdraw | Token transfers correct during withdraw | - | ✅ |
| [INT-16](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_token.rs) | integrity_token_donate | Token transfers correct during donate | - | ✅ |
| [INT-17](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_token.rs) | integrity_token_draw | Token transfers correct during draw | - | ✅ |
| [INT-18](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_token.rs) | integrity_token_queue_withdrawal | No token transfers during queue withdrawal | - | ✅ |
| [INT-19](https://github.com/alexzoid-eth/2025-02-blend-fv/tree/main/blend-contracts-v2/backstop/src/certora_specs/integrity_token.rs) | integrity_token_dequeue_withdrawal | No token transfers during dequeue withdrawal | - | ✅ |

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

For step-by-step installation steps refer to this setup [tutorial](https://alexzoid.com/first-steps-with-certora-fv-catching-a-real-bug#heading-setup).  

### Verification Execution

1. Build the backstop contract with Certora features:
```bash
cd blend-contracts-v2/backstop/confs
just build
```

2. Run a specific verification:
```bash
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