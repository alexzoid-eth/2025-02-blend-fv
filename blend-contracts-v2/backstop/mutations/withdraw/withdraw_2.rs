#[cfg(feature = "certora_storage_ghost")] // @note changed
use crate::{contract::require_nonnegative, certora_specs::summaries::storage, BackstopError};
#[cfg(not(feature = "certora_storage_ghost"))]
use crate::{contract::require_nonnegative, storage, BackstopError};

#[cfg(feature = "certora_emission_summarized")] 
use crate::certora_specs::summaries::emissions;
#[cfg(not(feature = "certora_emission_summarized"))] 
use crate::emissions;

#[cfg(feature = "certora_token_mock")]
use crate::certora_specs::mocks::token::TokenClient;
#[cfg(not(feature = "certora_token_mock"))]
use sep_41_token::TokenClient;

use soroban_sdk::{panic_with_error, unwrap::UnwrapOptimized, Address, Env};

use super::Q4W;

/// Perform a queue for withdraw from the backstop module
pub fn execute_queue_withdrawal(
    e: &Env,
    from: &Address,
    pool_address: &Address,
    amount: i128,
) -> Q4W {
    require_nonnegative(e, amount);

    let mut pool_balance = storage::get_pool_balance(e, pool_address);
    let mut user_balance = storage::get_user_balance(e, pool_address, from);

    // update emissions
    emissions::update_emissions(e, pool_address, &pool_balance, from, &user_balance);

    user_balance.queue_shares_for_withdrawal(e, amount);
    pool_balance.queue_for_withdraw(amount);

    storage::set_user_balance(e, pool_address, from, &user_balance);
    storage::set_pool_balance(e, pool_address, &pool_balance);

    user_balance.q4w.last().unwrap_optimized()
}

/// Perform a dequeue of queued for withdraw deposits from the backstop module
pub fn execute_dequeue_withdrawal(e: &Env, from: &Address, pool_address: &Address, amount: i128) {
    require_nonnegative(e, amount);

    let mut pool_balance = storage::get_pool_balance(e, pool_address);
    let mut user_balance = storage::get_user_balance(e, pool_address, from);

    // update emissions
    emissions::update_emissions(e, pool_address, &pool_balance, from, &user_balance);

    user_balance.dequeue_shares(e, amount);
    // user_balance.add_shares(amount); MUTANT
    pool_balance.dequeue_q4w(e, amount);

    storage::set_user_balance(e, pool_address, from, &user_balance);
    storage::set_pool_balance(e, pool_address, &pool_balance);
}

/// Perform a withdraw from the backstop module
pub fn execute_withdraw(e: &Env, from: &Address, pool_address: &Address, amount: i128) -> i128 {
    require_nonnegative(e, amount);

    let mut pool_balance = storage::get_pool_balance(e, pool_address);
    let mut user_balance = storage::get_user_balance(e, pool_address, from);
    
    user_balance.withdraw_shares(e, amount);

    let to_return = pool_balance.convert_to_tokens(amount);
    if to_return == 0 {
        panic_with_error!(e, &BackstopError::InvalidTokenWithdrawAmount);
    }
    pool_balance.withdraw(e, to_return, amount);

    storage::set_user_balance(e, pool_address, from, &user_balance);
    storage::set_pool_balance(e, pool_address, &pool_balance);

    #[cfg(feature = "certora_token_mock")] // @note changed
    {
        let backstop_token_addr = storage::get_backstop_token(e);
        let backstop_token_client = TokenClient::new(e, &backstop_token_addr);
        backstop_token_client.transfer(&e.current_contract_address(), from, &to_return);    
    }
    #[cfg(not(feature = "certora_token_mock"))]
    {
        let backstop_token_client = TokenClient::new(e, &storage::get_backstop_token(e));
        backstop_token_client.transfer(&e.current_contract_address(), from, &to_return);    
    }

    to_return
}

#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        vec, Address,
    };

    use crate::{
        backstop::{execute_deposit, execute_donate, execute_draw},
        testutils::{
            assert_eq_vec_q4w, create_backstop, create_backstop_token, create_mock_pool_factory,
        },
    };

    use super::*;

    #[test]
    fn test_execute_queue_withdrawal() {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        let backstop_address = create_backstop(&e);
        let pool_address = Address::generate(&e);
        let bombadil = Address::generate(&e);
        let samwise = Address::generate(&e);

        let (_, backstop_token_client) = create_backstop_token(&e, &backstop_address, &bombadil);
        backstop_token_client.mint(&samwise, &100_0000000);

        let (_, mock_pool_factory_client) = create_mock_pool_factory(&e, &backstop_address);
        mock_pool_factory_client.set_pool(&pool_address);

        // setup pool with deposits
        e.as_contract(&backstop_address, || {
            execute_deposit(&e, &samwise, &pool_address, 100_0000000);
        });

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 10000,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        e.as_contract(&backstop_address, || {
            execute_queue_withdrawal(&e, &samwise, &pool_address, 42_0000000);

            let new_user_balance = storage::get_user_balance(&e, &pool_address, &samwise);
            assert_eq!(new_user_balance.shares, 58_0000000);
            let expected_q4w = vec![
                &e,
                Q4W {
                    amount: 42_0000000,
                    exp: 10000 + 17 * 24 * 60 * 60,
                },
            ];
            assert_eq_vec_q4w(&new_user_balance.q4w, &expected_q4w);

            let new_pool_balance = storage::get_pool_balance(&e, &pool_address);
            assert_eq!(new_pool_balance.q4w, 42_0000000);
            assert_eq!(new_pool_balance.shares, 100_0000000);
            assert_eq!(new_pool_balance.tokens, 100_0000000);

            assert_eq!(
                backstop_token_client.balance(&backstop_address),
                100_0000000
            );
            assert_eq!(backstop_token_client.balance(&samwise), 0);
        });
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #8)")]
    fn test_execute_queue_withdrawal_negative_amount() {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        let backstop_address = create_backstop(&e);
        let pool_address = Address::generate(&e);
        let bombadil = Address::generate(&e);
        let samwise = Address::generate(&e);

        let (_, backstop_token_client) = create_backstop_token(&e, &backstop_address, &bombadil);
        backstop_token_client.mint(&samwise, &100_0000000);

        let (_, mock_pool_factory_client) = create_mock_pool_factory(&e, &backstop_address);
        mock_pool_factory_client.set_pool(&pool_address);

        // setup pool with deposits
        e.as_contract(&backstop_address, || {
            execute_deposit(&e, &samwise, &pool_address, 100_0000000);
        });

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 10000,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        e.as_contract(&backstop_address, || {
            execute_queue_withdrawal(&e, &samwise, &pool_address, -42_0000000);
        });
    }

    #[test]
    fn test_execute_dequeue_withdrawal() {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        let backstop_address = create_backstop(&e);
        let pool_address = Address::generate(&e);
        let bombadil = Address::generate(&e);
        let samwise = Address::generate(&e);

        let (_, backstop_token_client) = create_backstop_token(&e, &backstop_address, &bombadil);
        backstop_token_client.mint(&samwise, &100_0000000);

        let (_, mock_pool_factory_client) = create_mock_pool_factory(&e, &backstop_address);
        mock_pool_factory_client.set_pool(&pool_address);

        // queue shares for withdraw
        e.as_contract(&backstop_address, || {
            execute_deposit(&e, &samwise, &pool_address, 75_0000000);

            e.ledger().set(LedgerInfo {
                protocol_version: 22,
                sequence_number: 100,
                timestamp: 10000,
                network_id: Default::default(),
                base_reserve: 10,
                min_temp_entry_ttl: 10,
                min_persistent_entry_ttl: 10,
                max_entry_ttl: 3110400,
            });

            execute_queue_withdrawal(&e, &samwise, &pool_address, 25_0000000);

            e.ledger().set(LedgerInfo {
                protocol_version: 22,
                sequence_number: 100,
                timestamp: 20000,
                network_id: Default::default(),
                base_reserve: 10,
                min_temp_entry_ttl: 10,
                min_persistent_entry_ttl: 10,
                max_entry_ttl: 3110400,
            });

            execute_queue_withdrawal(&e, &samwise, &pool_address, 40_0000000);
        });

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 30000,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        e.as_contract(&backstop_address, || {
            execute_dequeue_withdrawal(&e, &samwise, &pool_address, 30_0000000);

            let new_user_balance = storage::get_user_balance(&e, &pool_address, &samwise);
            assert_eq!(new_user_balance.shares, 40_0000000);
            let expected_q4w = vec![
                &e,
                Q4W {
                    amount: 25_0000000,
                    exp: 10000 + 17 * 24 * 60 * 60,
                },
                Q4W {
                    amount: 10_0000000,
                    exp: 20000 + 17 * 24 * 60 * 60,
                },
            ];
            assert_eq_vec_q4w(&new_user_balance.q4w, &expected_q4w);

            let new_pool_balance = storage::get_pool_balance(&e, &pool_address);
            assert_eq!(new_pool_balance.q4w, 35_0000000);
            assert_eq!(new_pool_balance.shares, 75_0000000);
            assert_eq!(new_pool_balance.tokens, 75_0000000);
        });
    }
    #[test]
    #[should_panic(expected = "Error(Contract, #8)")]
    fn test_execute_dequeue_withdrawal_negative_amount() {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        let backstop_address = create_backstop(&e);
        let pool_address = Address::generate(&e);
        let bombadil = Address::generate(&e);
        let samwise = Address::generate(&e);

        let (_, backstop_token_client) = create_backstop_token(&e, &backstop_address, &bombadil);
        backstop_token_client.mint(&samwise, &100_0000000);

        let (_, mock_pool_factory_client) = create_mock_pool_factory(&e, &backstop_address);
        mock_pool_factory_client.set_pool(&pool_address);

        // queue shares for withdraw
        e.as_contract(&backstop_address, || {
            execute_deposit(&e, &samwise, &pool_address, 75_0000000);
            execute_queue_withdrawal(&e, &samwise, &pool_address, 25_0000000);

            e.ledger().set(LedgerInfo {
                protocol_version: 22,
                sequence_number: 100,
                timestamp: 10000,
                network_id: Default::default(),
                base_reserve: 10,
                min_temp_entry_ttl: 10,
                min_persistent_entry_ttl: 10,
                max_entry_ttl: 3110400,
            });

            execute_queue_withdrawal(&e, &samwise, &pool_address, 40_0000000);
        });

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 20000,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        e.as_contract(&backstop_address, || {
            execute_dequeue_withdrawal(&e, &samwise, &pool_address, -30_0000000);
        });
    }

    #[test]
    fn test_execute_withdrawal() {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        let backstop_address = create_backstop(&e);
        let pool_address = Address::generate(&e);
        let bombadil = Address::generate(&e);
        let samwise = Address::generate(&e);

        let (_, backstop_token_client) = create_backstop_token(&e, &backstop_address, &bombadil);
        backstop_token_client.mint(&samwise, &150_0000000);

        let (_, mock_pool_factory_client) = create_mock_pool_factory(&e, &backstop_address);
        mock_pool_factory_client.set_pool(&pool_address);

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 10000,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        backstop_token_client.approve(
            &samwise,
            &backstop_address,
            &50_0000000,
            &e.ledger().sequence(),
        );
        // setup pool with queue for withdrawal and allow the backstop to incur a profit
        e.as_contract(&backstop_address, || {
            execute_deposit(&e, &samwise, &pool_address, 100_0000000);
            execute_queue_withdrawal(&e, &samwise, &pool_address, 42_0000000);
            execute_donate(&e, &samwise, &pool_address, 50_0000000);
        });

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 10000 + 17 * 24 * 60 * 60 + 1,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        e.as_contract(&backstop_address, || {
            let tokens = execute_withdraw(&e, &samwise, &pool_address, 42_0000000);

            let new_user_balance = storage::get_user_balance(&e, &pool_address, &samwise);
            assert_eq!(new_user_balance.shares, 100_0000000 - 42_0000000);
            assert_eq!(new_user_balance.q4w.len(), 0);

            let new_pool_balance = storage::get_pool_balance(&e, &pool_address);
            assert_eq!(new_pool_balance.q4w, 0);
            assert_eq!(new_pool_balance.shares, 100_0000000 - 42_0000000);
            assert_eq!(new_pool_balance.tokens, 150_0000000 - tokens);
            assert_eq!(tokens, 63_0000000);

            assert_eq!(
                backstop_token_client.balance(&backstop_address),
                150_0000000 - tokens
            );
            assert_eq!(backstop_token_client.balance(&samwise), tokens);
        });
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #8)")]
    fn test_execute_withdrawal_negative_amount() {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        let backstop_address = create_backstop(&e);
        let pool_address = Address::generate(&e);
        let bombadil = Address::generate(&e);
        let samwise = Address::generate(&e);

        let (_, backstop_token_client) = create_backstop_token(&e, &backstop_address, &bombadil);
        backstop_token_client.mint(&samwise, &150_0000000);

        let (_, mock_pool_factory_client) = create_mock_pool_factory(&e, &backstop_address);
        mock_pool_factory_client.set_pool(&pool_address);

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 10000,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        backstop_token_client.approve(
            &samwise,
            &backstop_address,
            &50_0000000,
            &e.ledger().sequence(),
        );
        // setup pool with queue for withdrawal and allow the backstop to incur a profit
        e.as_contract(&backstop_address, || {
            execute_deposit(&e, &samwise, &pool_address, 100_0000000);
            execute_queue_withdrawal(&e, &samwise, &pool_address, 42_0000000);
            execute_donate(&e, &samwise, &pool_address, 50_0000000);
        });

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 10000 + 17 * 24 * 60 * 60 + 1,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        e.as_contract(&backstop_address, || {
            execute_withdraw(&e, &samwise, &pool_address, -42_0000000);
        });
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1006)")]
    fn test_execute_withdrawal_zero_tokens() {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        let backstop_address = create_backstop(&e);
        let pool_address = Address::generate(&e);
        let bombadil = Address::generate(&e);
        let samwise = Address::generate(&e);
        let frodo = Address::generate(&e);

        let (_, backstop_token_client) = create_backstop_token(&e, &backstop_address, &bombadil);
        backstop_token_client.mint(&samwise, &150_0000000);
        backstop_token_client.mint(&frodo, &150_0000000);

        let (_, mock_pool_factory_client) = create_mock_pool_factory(&e, &backstop_address);
        mock_pool_factory_client.set_pool(&pool_address);

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 10000,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        // setup pool with queue for withdrawal and allow the backstop to incur a profit
        e.as_contract(&backstop_address, || {
            execute_deposit(&e, &frodo, &pool_address, 1_0000001);
            execute_deposit(&e, &samwise, &pool_address, 1_0000000);
            execute_queue_withdrawal(&e, &samwise, &pool_address, 1_0000000);
            execute_draw(&e, &pool_address, 1_9999999, &frodo);
        });

        e.ledger().set(LedgerInfo {
            protocol_version: 22,
            sequence_number: 200,
            timestamp: 10000 + 17 * 24 * 60 * 60 + 1,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 10,
            min_persistent_entry_ttl: 10,
            max_entry_ttl: 3110400,
        });

        e.as_contract(&backstop_address, || {
            execute_withdraw(&e, &samwise, &pool_address, 1_0000000);
        });
    }
}
