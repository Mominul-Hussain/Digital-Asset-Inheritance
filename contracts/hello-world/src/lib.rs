#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Symbol, symbol_short};

// Structure to store inheritance information
#[contracttype]
#[derive(Clone)]
pub struct Inheritance {
    pub owner: Address,
    pub heir: Address,
    pub asset_amount: i128,
    pub deadline: u64,
    pub is_claimed: bool,
}

// Mapping owner address to their inheritance setup
#[contracttype]
pub enum InheritanceBook {
    Record(Address)
}

// Counter for total inheritances
const TOTAL_INHERITANCES: Symbol = symbol_short!("TOTAL");

#[contract]
pub struct InheritanceContract;

#[contractimpl]
impl InheritanceContract {
    
    // Function to set up inheritance by the owner
    pub fn setup_inheritance(
        env: Env, 
        owner: Address, 
        heir: Address, 
        asset_amount: i128, 
        deadline: u64
    ) {
        // Verify the owner is calling this function
        owner.require_auth();
        
        // Get current timestamp
        let current_time = env.ledger().timestamp();
        
        // Ensure deadline is in the future
        if deadline <= current_time {
            log!(&env, "Deadline must be in the future");
            panic!("Deadline must be in the future");
        }
        
        // Create inheritance record
        let inheritance = Inheritance {
            owner: owner.clone(),
            heir: heir.clone(),
            asset_amount,
            deadline,
            is_claimed: false,
        };
        
        // Store the inheritance record
        env.storage().instance().set(&InheritanceBook::Record(owner.clone()), &inheritance);
        
        // Update total count
        let mut total: u64 = env.storage().instance().get(&TOTAL_INHERITANCES).unwrap_or(0);
        total += 1;
        env.storage().instance().set(&TOTAL_INHERITANCES, &total);
        
        env.storage().instance().extend_ttl(10000, 10000);
        
        log!(&env, "Inheritance setup successfully for heir");
    }
    
    // Function for heir to claim inheritance after deadline
    pub fn claim_inheritance(env: Env, owner: Address, heir: Address) {
        // Verify the heir is calling this function
        heir.require_auth();
        
        // Get the inheritance record
        let mut inheritance = Self::view_inheritance(env.clone(), owner.clone());
        
        // Verify heir address matches
        if inheritance.heir != heir {
            log!(&env, "Unauthorized: You are not the designated heir");
            panic!("Unauthorized heir");
        }
        
        // Check if already claimed
        if inheritance.is_claimed {
            log!(&env, "Inheritance already claimed");
            panic!("Already claimed");
        }
        
        // Check if deadline has passed
        let current_time = env.ledger().timestamp();
        if current_time < inheritance.deadline {
            log!(&env, "Deadline not reached yet");
            panic!("Cannot claim before deadline");
        }
        
        // Mark as claimed
        inheritance.is_claimed = true;
        env.storage().instance().set(&InheritanceBook::Record(owner.clone()), &inheritance);
        
        env.storage().instance().extend_ttl(10000, 10000);
        
        log!(&env, "Inheritance claimed successfully by heir");
    }
    
    // Function to view inheritance details
    pub fn view_inheritance(env: Env, owner: Address) -> Inheritance {
        let key = InheritanceBook::Record(owner.clone());
        
        env.storage().instance().get(&key).unwrap_or(Inheritance {
            owner: owner.clone(),
            heir: owner.clone(),
            asset_amount: 0,
            deadline: 0,
            is_claimed: false,
        })
    }
    
    // Function to get total number of inheritances
    pub fn get_total_inheritances(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_INHERITANCES).unwrap_or(0)
    }
}