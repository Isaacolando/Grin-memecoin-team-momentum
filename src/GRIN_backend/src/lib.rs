
/* 
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::{init, update, query};
use ic_cdk_macros;
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Default)]
struct State {
    total_supply: u64,
    balances: HashMap<ic_cdk::export::Principal, u64>,
    name: String,
    symbol: String,
}

static mut STATE: Option<State> = None;

#[init]
fn init(name: String, symbol: String, total_supply: u64) {
    let caller = ic_cdk::caller();
    unsafe {
        STATE = Some(State {
            total_supply,
            balances: {
                let mut balances = HashMap::new();
                balances.insert(caller, total_supply);
                balances
            },
            name,
            symbol,
        });
    }
}

#[update]
fn transfer(to: ic_cdk::export::Principal, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    unsafe {
        let state = STATE.as_mut().unwrap();
        let caller_balance = state.balances.entry(caller).or_insert(0);
        if *caller_balance < amount {
            return Err("Insufficient balance".into());
        }
        *caller_balance -= amount;
        let recipient_balance = state.balances.entry(to).or_insert(0);
        *recipient_balance += amount;
        Ok(())
    }
}

#[query]
fn balance_of(owner: ic_cdk::export::Principal) -> u64 {
    unsafe {
        let state = STATE.as_ref().unwrap();
        *state.balances.get(&owner).unwrap_or(&0)
    }
}

#[query]
fn total_supply() -> u64 {
    unsafe { STATE.as_ref().unwrap().total_supply }
}

#[update]
fn airdrop(recipients: Vec<(ic_cdk::export::Principal, u64)>) -> Result<(), String> {
    let caller = ic_cdk::caller();
    unsafe {
        let state = STATE.as_mut().unwrap();
        if state.balances.get(&caller).unwrap_or(&0) < &recipients.iter().map(|(_, amount)| amount).sum() {
            return Err("Insufficient balance for airdrop".into());
        }
        for (recipient, amount) in recipients {
            let recipient_balance = state.balances.entry(recipient).or_insert(0);
            *recipient_balance += amount;
        }
    }
    Ok(())
}
*/
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::{init, update, query, pre_upgrade, post_upgrade};
use ic_cdk::storage;
use std::collections::HashMap;
use serde::Serialize;

// Define the state structure
#[derive(CandidType, Deserialize, Default, Serialize)]
struct State {
    total_supply: u64,
    balances: HashMap<Principal, u64>,
    name: String,
    symbol: String,
    admins: Vec<Principal>,
}

// Use storage for persistent state
#[init]
fn init(name: String, symbol: String, total_supply: u64) {
    let caller = ic_cdk::caller();
    let initial_state = State {
        total_supply,
        balances: {
            let mut balances = HashMap::new();
            balances.insert(caller, total_supply);
            balances
        },
        name,
        symbol,
        admins: vec![caller], // Set the caller as the initial admin
    };
    storage::stable_save((initial_state,)).unwrap();
}

#[update]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let mut state: State = storage::stable_restore().unwrap();
    let caller_balance = state.balances.entry(caller).or_insert(0);

    if *caller_balance < amount {
        return Err("Insufficient balance".into());
    }
    *caller_balance -= amount;
    let recipient_balance = state.balances.entry(to).or_insert(0);
    *recipient_balance += amount;

    storage::stable_save((state,)).unwrap();
    Ok(())
}

#[query]
fn balance_of(owner: Principal) -> u64 {
    let state: State = storage::stable_restore().unwrap();
    *state.balances.get(&owner).unwrap_or(&0)
}

#[query]
fn total_supply() -> u64 {
    let state: State = storage::stable_restore().unwrap();
    state.total_supply
}

#[update]
fn airdrop(recipients: Vec<(Principal, u64)>) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let mut state: State = storage::stable_restore().unwrap();
    let total_airdrop_amount: u64 = recipients.iter().map(|(_, amount)| amount).sum();

    let caller_balance = state.balances.entry(caller).or_insert(0);
    if *caller_balance < total_airdrop_amount {
        return Err("Insufficient balance for airdrop".into());
    }

    for (recipient, amount) in recipients {
        let recipient_balance = state.balances.entry(recipient).or_insert(0);
        *recipient_balance += amount;
    }

    storage::stable_save((state,)).unwrap();
    Ok(())
}

// Governance: Adding admins
#[update]
fn add_admin(new_admin: Principal) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let mut state: State = storage::stable_restore().unwrap();

    if !state.admins.contains(&caller) {
        return Err("Only admins can add new admins".into());
    }

    if !state.admins.contains(&new_admin) {
        state.admins.push(new_admin);
    }

    storage::stable_save((state,)).unwrap();
    Ok(())
}

#[query]
fn is_admin(user: Principal) -> bool {
    let state: State = storage::stable_restore().unwrap();
    state.admins.contains(&user)
}

// Pre-upgrade and post-upgrade hooks for smooth upgrades
#[pre_upgrade]
fn pre_upgrade() {
    let state: State = storage::stable_restore().unwrap();
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (state,): (State,) = storage::stable_restore().unwrap();
    storage::stable_save((state,)).unwrap();
}

