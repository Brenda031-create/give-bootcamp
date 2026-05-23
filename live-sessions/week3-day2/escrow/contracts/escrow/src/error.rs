use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractError {
    InsufficientFunds = 1,
    PurchaseAlreadyExists = 2,
    AlreadyReleased = 3,
    FundsNotReleased = 4,
}
