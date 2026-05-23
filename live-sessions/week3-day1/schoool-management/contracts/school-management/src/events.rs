use soroban_sdk::{contractevent, Address};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentEvent {
    #[topic]
    pub wallet_address: Address,
    pub student_id: u64,
    pub amount: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StudentRegisteredEvent {
    pub student_id: u64,
    #[topic]
    pub wallet: Address,
}
