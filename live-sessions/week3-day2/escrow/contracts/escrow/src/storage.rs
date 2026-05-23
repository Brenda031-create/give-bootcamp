use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct EscrowPurchase {
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub amount: i128,
    pub is_released: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Purchase,
}
