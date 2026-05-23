use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    InsufficientFunds = 1,
    StudentNotFound = 2,

    // NEW ERROR:
    // Used when a deactivated student attempts payment.
    StudentNotRegistered = 3,
}