#![cfg(test)]

use soroban_sdk::{testutils::Address as _, token, Address, Env, String};

use crate::{
    school_management::{SchoolManagement, SchoolManagementClient},
    storage::Class,
};

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
    client: SchoolManagementClient<'a>,
    student_wallet: Address,
    usdc_asset: Address,
    token_client: token::StellarAssetClient<'a>,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();

    env.mock_all_auths();

    let admin = Address::generate(&env);

    let (usdc_asset, token_client) = create_token_contract(&env, admin.clone());

    let contract_id = env.register(SchoolManagement, (&admin, &usdc_asset));

    let client = SchoolManagementClient::new(&env, &contract_id);

    let student_wallet = Address::generate(&env);

    SetUpResult {
        env,
        client,
        student_wallet,
        usdc_asset,
        token_client,
    }
}

#[test]
fn test_register_student() {
    let setup_result = setup();

    let name = String::from_str(&setup_result.env, "Sib");

    let class_name = Class::College;

    let registration_result =
        setup_result
            .client
            .register_student(&setup_result.student_wallet, &name, &class_name);

    assert_eq!(registration_result, 1);
}

#[test]
fn test_get_student() {
    let setup_result = setup();

    let name = String::from_str(&setup_result.env, "Sib");

    let class_name = Class::College;

    setup_result
        .client
        .register_student(&setup_result.student_wallet, &name, &class_name);

    let student_id = 1;

    let result = setup_result.client.get_student(&student_id);

    assert_eq!(result.student_id, 1);
    assert_eq!(result.name, name);
}

#[test]
fn test_make_payment() {
    let setup_result = setup();

    let name = String::from_str(&setup_result.env, "Sib");

    let class_name = Class::College;

    setup_result
        .client
        .register_student(&setup_result.student_wallet, &name, &class_name);

    let student_id = 1;

    let amount = 1_000_000i128;

    setup_result
        .token_client
        .mint(&setup_result.student_wallet, &amount);

    let result = setup_result.client.try_make_payment(&student_id, &amount);

    assert!(result.is_ok());

    let student = setup_result.client.get_student(&student_id);

    assert_eq!(student.total_paid, amount);
}
