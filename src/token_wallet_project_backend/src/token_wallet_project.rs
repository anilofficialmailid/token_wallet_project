use ic_cdk::api::{call, time};
use ic_cdk::export::{candid::{CandidType, Deserialize}, Principal};
use ic_cdk::storage;
use ic_cdk_macros::{query, update};
use ic_cdk::println;

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct Wallet {
    pub balance: u64,
}

#[update]
fn send_tokens(to: Principal, amount: u64) -> Result<String, String> {
    let sender = ic_cdk::caller();
    println!("Sender: {:?}", sender);
    println!("Receiver: {:?}", to);
    println!("Amount: {:?}", amount);
    
    let mut sender_wallet = get_wallet(sender.clone());
    if sender_wallet.balance < amount {
        return Err("Insufficient funds".to_string());
    }
    sender_wallet.balance -= amount;
    println!("Sender's new balance: {:?}", sender_wallet.balance);

    set_wallet(sender.clone(), sender_wallet);

    let mut receiver_wallet = get_wallet(to.clone());
    receiver_wallet.balance += amount;
    println!("Receiver's new balance: {:?}", receiver_wallet.balance);

    set_wallet(to.clone(), receiver_wallet);

    Ok(format!("Sent {} tokens to {}", amount, to))
}

#[query]
fn balance_of(account: Principal) -> u64 {
    let account_wallet = get_wallet(account);
    account_wallet.balance
}

fn get_wallet(account: Principal) -> Wallet {
    storage::get_mut::<std::collections::HashMap<Principal, Wallet>>()
        .entry(account)
        .or_insert(Wallet::default())
        .clone()
}

fn set_wallet(account: Principal, wallet: Wallet) {
    storage::get_mut::<std::collections::HashMap<Principal, Wallet>>()
        .insert(account, wallet);
}
