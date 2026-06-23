use solana_sdk::{pubkey::Pubkey, signature::{Keypair, Signer}};


fn create_keypair(amount_in_lamports: u64, rpc_client: RpcClient){

    let keypair = Keypair::new();
    let public_key = keypair.pubkey();

    println!("Nova conta criada");
    println!("Endereço público: {}", public_key);


    match rpc_client.request_airdrop(&public_key, amount_in_lamports) {
        Ok(_) => println!("{} lamports enviados para {}", amount_in_lamports, public_key),
        Err(e) => println!("Erro ao enviar lamports: {}", e),
    }

    return (keypair, public_key, amount_in_lamports);
}

fn use_existing_keypair(amount_in_lamports: u64, rpc_client: RpcClient){

    println!("insert your pubkey: ")
    let public_key = get_user_input();


    match rpc_client.request_airdrop(&public_key, amount_in_lamports) {
        Ok(_) => println!("{} lamports enviados para {}", amount_in_lamports, public_key),
        Err(e) => println!("Erro ao enviar lamports: {}", e),
    }
    
}

fn get_user_input() -> u8 {
    println!("Digite sua opção: ");

    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    
    input.trim().parse().expect("Erro ao converter entrada para número")  
    
    return input;
}

fn get_public_key() -> Pubkey {
    println!("Digite sua chave pública: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    Pubkey::try_from(input.trim()).expect("Erro ao converter entrada para Pubkey")

    return public_key;
}

fn get_amount_in_lamports() -> u64 {
    println!("Digite a quantidade de lamports/sol você deseja receber: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    input.trim().parse().expect("Erro ao converter entrada para número")

    return amount_in_lamports;
}

fn exit() {
    std::process::exit(0);
}
