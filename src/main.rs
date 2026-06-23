use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    system_transaction,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{

    //configurando o cliente para a devnet

    let rpc_url = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(rpc_url);

    //gerando keypair nova

    println!("welcome to faucet tool!");

    println!("how many lamports/sol you want recive? (1_000_000_00 = 1 sol 1_500_000_000 = 1.5 sol)");
    get_amount_in_lamports();

    println!("you choose {}", amount_in_lamports);

    println!("Do you want to use an existing wallet or create a new one?");
    println!("1 - Create new account");
    println!("2 - Use existing account");
    println!("3 - Exit");
    println!("--------------------------- ");

    let decision = get_user_input();

    match decision {
        1 => create_keypair(amount_in_lamports, rpc_client);
        2 => use_existing_keypair(amount_in_lamports, rpc_client);
        3 => exit();
        _ => println!("Opção inválida")
    }

    

    Ok(())
}
