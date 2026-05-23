use core::panic;

use soroban_sdk::{
    contract, contractimpl, token, Address, Env, String, Vec,
};

use crate::{
    error::ContractError,
    events::{PaymentEvent, StudentRegisteredEvent},
    storage::{Class, DataKey, Payment, StudentDetails},
};

#[contract]
pub struct SchoolManagement;

#[contractimpl]
impl SchoolManagement {
    pub fn __constructor(env: &Env, admin: Address, token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("The contract is already initialized");
        }

        admin.require_auth();
               env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::StudentCount, &0u64);
    }

    // HELPER FUNCTION to Ensures only the admin can perform certain actions.
    fn require_admin(env: &Env) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();

        admin.require_auth();
    }
   pub fn register_student(
        env: &Env,
        student_wallet: Address,
        name: String,
        class_name: Class,
    ) -> u64 {
        student_wallet.require_auth();

        let mut count = env
            .storage()
            .instance()
            .get(&DataKey::StudentCount)
            .unwrap();

        count += 1;
        let student = StudentDetails {
            student_id: count,
            name,
            wallet_address: student_wallet.clone(),
            class_name,
            total_paid: 0,
            is_registered: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Student(count), &student);

        let payments: Vec<Payment> = Vec::new(env);
 env.storage()
            .persistent()
            .set(&DataKey::StudentPayments(count), &payments);

        env.storage().instance().set(&DataKey::StudentCount, &count);

        //Emit an event after successful registration.
        StudentRegisteredEvent {
            student_id: count,
            wallet: student_wallet,
        }
        .publish(env);

        count
    }

    pub fn get_student(env: &Env, student_id: u64) -> StudentDetails {
        env.storage()
            .persistent()
            .get(&DataKey::Student(student_id))
            .unwrap()
    }

    //FUNCTION to Returns all payment records belonging to a student.
    pub fn get_student_payments(env: &Env, student_id: u64) -> Vec<Payment> {
        env.storage()
            .persistent()
            .get(&DataKey::StudentPayments(student_id))
            .unwrap()
    }
     //FUNCTION Allows the admin to update a student's class.
    pub fn update_student_class(
        env: &Env,
        student_id: u64,
        new_class: Class,
    ) {
        Self::require_admin(env);

        let mut student = Self::get_student(env, student_id);

        student.class_name = new_class;

        env.storage()
            .persistent()
            .set(&DataKey::Student(student_id), &student);
    }
    // FUNCTION Allows the admin to deactivate a student.
    // The student remains in storage but can no longer make payments.
    pub fn deactivate_student(env: &Env, student_id: u64) {
        Self::require_admin(env);

        let mut student = Self::get_student(env, student_id);

        student.is_registered = false;

        env.storage()
            .persistent()
            .set(&DataKey::Student(student_id), &student);
    }
pub fn make_payment(env: &Env, student_id: u64, amount: i128) -> Result<(), ContractError> {
        if amount <= 0 {
            return Err(ContractError::InsufficientFunds);
        }

        let mut student: StudentDetails = Self::get_student(env, student_id);

        //VALIDATION to Prevent payments from deactivated students.
        if !student.is_registered {
            return Err(ContractError::StudentNotRegistered);
        }

        student.wallet_address.require_auth();

        let school_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();

        let token_address = env.storage().instance().get(&DataKey::Token).unwrap();

        let token_client = token::Client::new(env, &token_address);

        token_client.transfer(&student.wallet_address, &school_admin, &amount);

        student.total_paid += amount;

        let mut payments: Vec<Payment> = env
            .storage()
            .persistent()
            .get(&DataKey::StudentPayments(student_id))
            .unwrap();
       
        let payment = Payment {
            student_id,
            amount,
            timestamp: env.ledger().timestamp(),
        };

        payments.push_back(payment);

        env.storage()
            .persistent()
            .set(&DataKey::StudentPayments(student_id), &payments);

        env.storage()
            .persistent()
            .set(&DataKey::Student(student_id), &student);
 
        PaymentEvent {
            wallet_address: student.wallet_address,
            student_id,
            amount: amount.try_into().unwrap(),
        }
        .publish(env);

        Ok(())
    }
}
