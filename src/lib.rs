pub use solana_client::rpc_client::RpcClient;
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

pub fn get_insert_u64() -> u64 {
    loop {
        let input = get_insert_string();
        match input.parse::<u64>() {
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
    signature::{Keypair, Signer},
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

pub fn airdrop_sol(pubkey: &Pubkey, ammount_in_lamports: u64, rpc_client: &RpcClient) {
    let sig = rpc_client
        .request_airdrop(pubkey, ammount_in_lamports)
        .unwrap();
    let confirmed = rpc_client.confirm_transaction(&sig).unwrap(); //dizer se a transação foi confirmada ou não 
    println!("Transaction confirmed: {:?}", confirmed);
    if confirmed {
        println!("Airdrop successful!");
    } else {
        println!("Airdrop failed!");
    }
}

pub fn get_balance(pubkey: Pubkey, rpc_client: &RpcClient) -> u64 {
    rpc_client.get_balance(&pubkey).unwrap()
}

pub fn create_new_wallet(_ammount_in_lamports: u64, _rpc_client: &RpcClient) -> Pubkey {
    let keypair = Keypair::new();
    println!("New wallet created: {:?}", keypair.pubkey());

    return keypair.pubkey();
}

pub fn use_existing_wallet(_ammount_in_lamports: u64, rpc_client: &RpcClient) -> Pubkey {
    println!("insert your public key to recive the {_ammount_in_lamports}", _ammount_in_lamports = _ammount_in_lamports);

    let pubkey = get_insert_string();
    let pubkey_addres = Pubkey::from_str(&pubkey).unwrap();

    let balance = get_balance(pubkey_addres, rpc_client);
    println!("your balance is {balance} lamports");

    return pubkey_addres;
}
