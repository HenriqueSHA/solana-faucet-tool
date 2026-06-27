pub use solana_client::rpc_client::RpcClient;
pub use std::error::Error;
pub use std::io;
use std::str::FromStr;

pub fn get_insert_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_insert_u8() -> u8 {
    loop {
        let input = get_insert_string();
        match input.parse::<u8>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer between 0 and 255:"),
        }
    }
}

pub fn get_insert_i8() -> i8 {
    loop {
        let input = get_insert_string();
        match input.parse::<i8>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer between -128 and 127:"),
        }
    }
}

pub fn get_insert_f64() -> f64 {
    loop {
        let input = get_insert_string();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid decimal number (e.g. 1.5):"),
        }
    }
}

pub use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};

pub mod system_transaction {
    use super::*;
    use solana_sdk::hash::Hash;

    pub fn transfer(
        from_keypair: &Keypair,
        to_pubkey: &Pubkey,
        lamports: u64,
        recent_blockhash: Hash,
    ) -> Transaction {
        let from_pubkey = from_keypair.pubkey();
        let instruction =
            solana_system_interface::instruction::transfer(&from_pubkey, to_pubkey, lamports);
        Transaction::new_signed_with_payer(
            &[instruction],
            Some(&from_pubkey),
            &[from_keypair],
            recent_blockhash,
        )
    }
}

pub fn get_balance(pubkey: String, rpc_client: RpcClient) -> u64 {
    let pubkey = Pubkey::new_from_string(&pubkey);
    let balance = rpc_client.get_balance(&pubkey).unwrap();
    balance
}

pub fn create_new_wallet(ammount_in_lamports: u64, rpc_client: RpcClient) {
    let keypair = Keypair::new();
    println!("New wallet created: {:?}", keypair.pubkey());
}

pub fn use_existing_wallet(ammount_in_lamports: u64, rpc_client: RpcClient) {
    println!(
        "insert your public key to recive the {ammount_in_lamports}",
        ammount_in_lamports
    );

    let pubkey = get_insert_string();

    let balance = rpc_client
        .get_balance(&pubkey)
        .expect("Failed to get balance");

    let balance = get_balance(pubkey, rpc_client);
    println!("your balance is {balance} lamports");
}
