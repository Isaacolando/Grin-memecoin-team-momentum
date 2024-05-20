/*#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
*/
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
