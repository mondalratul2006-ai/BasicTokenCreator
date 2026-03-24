#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, symbol_short};

// Storage keys
const ADMIN: Symbol = symbol_short!("ADMIN");
const TOTAL_SUPPLY: Symbol = symbol_short!("SUPPLY");

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {

    // Initialize contract with admin
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOTAL_SUPPLY, &0i128);
    }

    // Mint tokens (only admin)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let balance = Self::balance(env.clone(), to.clone());
        env.storage().instance().set(&to, &(balance + amount));

        let total: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap();
        env.storage().instance().set(&TOTAL_SUPPLY, &(total + amount));
    }

    // Transfer tokens
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage().instance().set(&from, &(from_balance - amount));
        env.storage().instance().set(&to, &(to_balance + amount));
    }

    // Check balance
    pub fn balance(env: Env, user: Address) -> i128 {
        env.storage().instance().get(&user).unwrap_or(0)
    }

    // Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_SUPPLY).unwrap_or(0)
    }
}