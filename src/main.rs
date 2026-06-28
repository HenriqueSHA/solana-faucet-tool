use solana_faucet_tool::*;

fn main() {
    println!("welcome to faucet tool!");

    println!("wich network you want to use?");
    println!("1 - Devnet");
    println!("2 - Localhost");
    println!("--------------------------- ");

    let network_decision = get_insert_string();

    let rpc_client = match network_decision.as_str() {
        "1" => RpcClient::new_with_commitment("https://api.devnet.solana.com", CommitmentConfig::confirmed()),
        "2" => RpcClient::new_with_commitment("http://localhost:8899", CommitmentConfig::confirmed()),
        _ => {
            println!("Opção inválida");
            std::process::exit(1);
        }
    };

    //request ammount of lamports/sol from user
    println!(
        "how many lamports/sol you want recive? (1_000_000_00 = 1 sol 1_500_000_000 = 1.5 sol)"
    );
    let ammount_in_lamports = get_insert_u64();

    println!("you choose {}", ammount_in_lamports);

    println!("Do you want to use an existing wallet or create a new one?");
    println!("1 - Create new account");
    println!("2 - Use existing account");
    println!("3 - Exit");
    println!("--------------------------- ");

    let wallet_decision = get_insert_string();

    let pubkey = match wallet_decision.as_str() {
        "1" => create_new_wallet(ammount_in_lamports, &rpc_client),
        "2" => use_existing_wallet(ammount_in_lamports, &rpc_client),
        "3" => {
            std::process::exit(0);
        }
        _ => {
            println!("Opção inválida");
            std::process::exit(1);
        }
    };

    let balance_before = get_balance(pubkey, &rpc_client);
    println!("your balance before airdrop is {} lamports", balance_before);

    println!("Requesting airdrop...");
    airdrop_sol(&pubkey, ammount_in_lamports, &rpc_client);

    let balance_after = get_balance(pubkey, &rpc_client);
    println!("your balance after airdrop is {} lamports", balance_after);
}
