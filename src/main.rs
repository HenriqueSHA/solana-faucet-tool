use solana_faucet_tool::*;

fn main() {
    //config rpc client

    let mut decision = String::new();

    let mut ammount_in_lamports = String::new();

    println!("welcome to faucet tool!");

    println!("wich network you want to use?");
    println!("1 - Devnet");
    println!("2 - Localhost");
    println!("--------------------------- ");
    io::stdin()
        .read_line(&mut decision)
        .expect("Failed to read line");
    let decision = decision.trim();

    let rpc_client = match decision {
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
    io::stdin()
        .read_line(&mut ammount_in_lamports)
        .expect("Failed to read line");
    let mut ammount_in_lamports = ammount_in_lamports.trim();

    println!("you choose {}", ammount_in_lamports);

    println!("Do you want to use an existing wallet or create a new one?");
    println!("1 - Create new account");
    println!("2 - Use existing account");
    println!("3 - Exit");
    println!("--------------------------- ");

    match decision {
        "1" => {
            create_new_wallet(ammount_in_lamports, rpc_client);
        }
        "2" => {
            use_existing_wallet(ammount_in_lamports, rpc_client);
        }
        "3" => {
            std::process::exit(0);
        }
        _ => {
            println!("Opção inválida");
            std::process::exit(1);
        }
    }
}
