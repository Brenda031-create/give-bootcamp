use soroban_sdk::{testutils::Address as _, token, Address, Env};

use crate::escrow::{EscrowContract, EscrowContractClient};

fn create_token_contract<'a>(
    env: &Env,
    admin: Address,
) -> (Address, token::StellarAssetClient<'a>) {
    let contract_id = env.register_stellar_asset_contract_v2(admin.clone());
    (
        contract_id.address(),
        token::StellarAssetClient::new(env, &contract_id.address()),
    )
}

struct SetUpResult<'a> {
    env: Env,
    client: EscrowContractClient<'a>,
    buyer: Address,
    seller: Address,
    usdc_asset: Address,
    token_client: token::StellarAssetClient<'a>,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();

    env.mock_all_auths();

    let admin = Address::generate(&env);

    let (usdc_asset, token_client) = create_token_contract(&env, admin.clone());

    let contract_id = env.register(EscrowContract, ());

    let client = EscrowContractClient::new(&env, &contract_id);

    let buyer = Address::generate(&env);

    let seller = Address::generate(&env);

    SetUpResult {
        env,
        client,
        buyer,
        seller,
        usdc_asset,
        token_client,
    }
}

#[test]
fn test_make_payment() {
    let setup_result = setup();

    let amount = 1_000_000i128;

    setup_result.token_client.mint(&setup_result.buyer, &amount);

    let result = setup_result.client.try_create_order(
        &setup_result.buyer,
        &setup_result.seller,
        &setup_result.usdc_asset,
        &amount,
    );

    assert!(result.is_ok());

    let purchase = setup_result.client.get_purchase();

    assert_eq!(purchase.buyer, setup_result.buyer);
    assert_eq!(purchase.seller, setup_result.seller);
    assert_eq!(purchase.amount, amount);
    assert!(!purchase.is_released);
}

#[test]
fn test_release() {
    let setup_result = setup();

    let amount = 1_000_000i128;

    setup_result.token_client.mint(&setup_result.buyer, &amount);

    let result = setup_result.client.try_create_order(
        &setup_result.buyer,
        &setup_result.seller,
        &setup_result.usdc_asset,
        &amount,
    );

    assert!(result.is_ok());

    let purchase = setup_result.client.get_purchase();

    assert_eq!(purchase.buyer, setup_result.buyer);
    assert_eq!(purchase.seller, setup_result.seller);
    assert_eq!(purchase.amount, amount);
    assert!(!purchase.is_released);

    setup_result.env.mock_all_auths();

    let release = setup_result.client.try_release();

    assert!(release.is_ok());

    let purchase = setup_result.client.get_purchase();

    assert!(purchase.is_released);
}

#[test]
fn test_withdraw() {
    let setup_result = setup();

    let amount = 1_000_000i128;

    setup_result.token_client.mint(&setup_result.buyer, &amount);

    let result = setup_result.client.try_create_order(
        &setup_result.buyer,
        &setup_result.seller,
        &setup_result.usdc_asset,
        &amount,
    );

    assert!(result.is_ok());

    let purchase = setup_result.client.get_purchase();

    assert_eq!(purchase.buyer, setup_result.buyer);
    assert_eq!(purchase.seller, setup_result.seller);
    assert_eq!(purchase.amount, amount);
    assert!(!purchase.is_released);

    setup_result.env.mock_all_auths();

    let release = setup_result.client.try_release();

    assert!(release.is_ok());

    let purchase = setup_result.client.get_purchase();

    assert!(purchase.is_released);

    let withdraw = setup_result.client.try_withdraw();

    assert!(withdraw.is_ok());

    let sellers_balance = setup_result.token_client.balance(&setup_result.seller);

    assert_eq!(sellers_balance, amount);
}
