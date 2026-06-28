use solana_faucet_tool::*;

fn main() {
    //config rpc client

    let mut decision = String::new();
    let mut pubkey = Pubkey::default();
    let mut ammount_in_lamports: u64 = 0;

    println!("welcome to faucet tool!");

    println!("wich network you want to use?");
    println!("1 - Devnet");
    println!("2 - Localhost");
    println!("--------------------------- ");

    decision = get_insert_string();

    let rpc_client = match decision.as_str() {
        "1" => RpcClient::new("https://api.devnet.solana.com"),
        "2" => RpcClient::new("http://localhost:8899"),
        _ => {
            println!("Opção inválida");
            std::process::exit(1);
        }
    };

    //request ammount of lamports/sol from user

    println!(
        "how many lamports/sol you want recive? (1_000_000_00 = 1 sol 1_500_000_000 = 1.5 sol)"
    );
    ammount_in_lamports = get_insert_u64();

    println!("you choose {}", ammount_in_lamports);

    println!("Do you want to use an existing wallet or create a new one?");
    println!("1 - Create new account");
    println!("2 - Use existing account");
    println!("3 - Exit");
    println!("--------------------------- ");

    decision = get_insert_string();

    match decision.as_str() {
        "1" => {
            pubkey = create_new_wallet(ammount_in_lamports, &rpc_client);
        }
        "2" => {
            pubkey = use_existing_wallet(ammount_in_lamports, &rpc_client);
        }
        "3" => {
            std::process::exit(0);
        }
        _ => {
            println!("Opção inválida");
            std::process::exit(1);
        }
    }

    let balance = get_balance(pubkey, &rpc_client);
    println!("your balance is {} lamports", balance);

    airdrop_sol(&pubkey, ammount_in_lamports, &rpc_client);
}
