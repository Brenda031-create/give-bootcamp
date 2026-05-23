use soroban_sdk::{contract, contractimpl, token, Address, Env};

use crate::{
    error::ContractError,
    storage::{DataKey, EscrowPurchase},
};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn create_order(
        env: &Env,
        buyer: Address,
        seller: Address,
        token: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        buyer.require_auth();

        if env.storage().instance().has(&DataKey::Purchase) {
            return Err(ContractError::PurchaseAlreadyExists);
        }

        let token_client = token::Client::new(env, &token);

        let buyer_balance = token_client.balance(&buyer);

        if buyer_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        token_client.transfer(&buyer, env.current_contract_address(), &amount);

        let purchase = EscrowPurchase {
            buyer,
            seller,
            token,
            amount,
            is_released: false,
        };

        env.storage().instance().set(&DataKey::Purchase, &purchase);

        Ok(())
    }

    pub fn release(env: &Env) -> Result<(), ContractError> {
        let mut purchase: EscrowPurchase = Self::get_purchase(env);

        purchase.buyer.require_auth();

        if purchase.is_released {
            return Err(ContractError::AlreadyReleased);
        }

        purchase.is_released = true;

        env.storage().instance().set(&DataKey::Purchase, &purchase);

        Ok(())
    }

    pub fn withdraw(env: &Env) -> Result<(), ContractError> {
        let purchase: EscrowPurchase = Self::get_purchase(env);

        purchase.seller.require_auth();

        if !purchase.is_released {
            return Err(ContractError::FundsNotReleased);
        }

        let token_client = token::Client::new(env, &purchase.token);

        token_client.transfer(
            &env.current_contract_address(),
            &purchase.seller,
            &purchase.amount,
        );

        Ok(())
    }

    pub fn get_purchase(env: &Env) -> EscrowPurchase {
        env.storage().instance().get(&DataKey::Purchase).unwrap()
    }
}
