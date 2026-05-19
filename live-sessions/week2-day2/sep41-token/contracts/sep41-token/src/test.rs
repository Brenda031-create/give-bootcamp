#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::{
    our_token::{SibToken, SibTokenClient},
    storage::{AllowanceKey, DataKey},
};
struct SetUpResult<'a> {
    env: Env,
    client: SibTokenClient<'a>,
    contract_id: Address,
    sender: Address,
    receiver: Address,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();

    let contract_id = env.register(SibToken, ());

    let client = SibTokenClient::new(&env, &contract_id);

    let sender = Address::generate(&env);

    let receiver = Address::generate(&env);

    SetUpResult {
        env,
        client,
        contract_id,
        sender,
        receiver,
    }
}

#[test]
fn test_name() {
    let setup_result = setup();

    let name = setup_result.client.name();
    let token_name = String::from_str(&setup_result.env, "SibToken");
    assert_eq!(name, token_name);
}

#[test]
fn test_symbol() {
    let setup_result = setup();

    let name = setup_result.client.symbol();
    let token_name = String::from_str(&setup_result.env, "SIB");

    let not_token_name = String::from_str(&setup_result.env, "Sib");
    assert_eq!(name, token_name);
    assert_ne!(name, not_token_name);
}

#[test]
fn test_decimal() {
    let setup_result = setup();

    let decimal = setup_result.client.decimals();
    let token_decimal = 18;

    assert_eq!(decimal, token_decimal);
}

#[test]
fn test_transfer() {
    let setup_result = setup();
}
//assignment: implement burn test
#[test]
fn test_burn() {
    let setup_result = setup();

    setup_result.env.mock_all_auths();

    setup_result.env.as_contract(&setup_result.contract_id, || {
        setup_result
            .env
            .storage()
            .persistent()
            .set(&DataKey::Balance(setup_result.sender.clone()), &1000i128);
    });

    setup_result.client.burn(&setup_result.sender, &500);

    let balance = setup_result.client.balance(&setup_result.sender);
    assert_eq!(balance, 500);
}
#[test]
fn test_burn_from() {
    let setup_result = setup();

    setup_result.env.mock_all_auths();

    setup_result.env.as_contract(&setup_result.contract_id, || {
        setup_result
            .env
            .storage()
            .persistent()
            .set(&DataKey::Balance(setup_result.sender.clone()), &1000i128);

        setup_result.env.storage().persistent().set(
            &DataKey::Allowance(AllowanceKey {
                from: setup_result.sender.clone(),
                spender: setup_result.receiver.clone(),
            }),
            &700i128,
        );
    });

    setup_result
        .client
        .burn_from(&setup_result.receiver, &setup_result.sender, &500);

    let sender_balance = setup_result.client.balance(&setup_result.sender);
    let remaining_allowance = setup_result
        .client
        .allowance(&setup_result.sender, &setup_result.receiver);

    assert_eq!(sender_balance, 500);
    assert_eq!(remaining_allowance, 200);
}
#[test]
fn test_mint() {
    let setup_result = setup();

    setup_result.env.mock_all_auths();

    setup_result.client.mint(&setup_result.receiver, &1000);

    let balance = setup_result.client.balance(&setup_result.receiver);
    assert_eq!(balance, 1000);
}
