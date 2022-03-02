use anyhow::anyhow;
use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::read_keypair_file;

fn main() {
    println!("-----");
    println!("Let's say hello to a Solana account...");
    // start by getting the rpc url and account from the cli config file
    // https://docs.rs/solana-cli-config/latest/solana_cli_config/
    let config_file = solana_cli_config::CONFIG_FILE
        .as_ref()
        .ok_or_else(|| anyhow!("unable to get config file path"))
        .unwrap();
    let cli_config = Config::load(config_file)
        // TODO: what's the equive of ok_or_else here?
        .unwrap();

    // TODO: read https://docs.solana.com/developing/clients/jsonrpc-api#configuring-state-commitment
    let client = RpcClient::new(&cli_config.json_rpc_url);

    println!(
        "Connection to cluster established: {:?} ({:?})",
        cli_config.json_rpc_url,
        client.get_version().unwrap()
    );

    let payer_key = read_keypair_file(cli_config.keypair_path).unwrap();
    let payer_balance = client.get_balance(&payer_key.pubkey()).unwrap();
    // // get payer keypair from config.keypair_path
    println!(
        "Using account {:?} containing {:?} SOL to pay for fees",
        payer_key.pubkey(),
        payer_balance
    );
    // TODO: get programid from ../../dist/program/helloworld-keypair.json (created by ts client for now)
    let program_key = read_keypair_file("../../dist/program/helloworld-keypair.json").unwrap();
    let program_account = client.get_account(&program_key.pubkey()).unwrap();
    //let program_id = "5gBQEUwAnt8qowhMP1ZmXMnTLnRbz4CUzwVCzXo1CWdz";
    println!(
        "Using program {:?}, executable: {:?}, owned by: {:?}",
        program_key.pubkey(),
        program_account.executable,
        program_account.owner
    );
    // // get greeting account - key derived from "hello" so its easy to find.
    let greeted =
        Pubkey::create_with_seed(&payer_key.pubkey(), &"hello", &program_key.pubkey()).unwrap();
    println!("Saying hello to {:?}", greeted);
    // println!("FoSMxh52TDgDAjkpnCSDuDbLwxmAugvzof1T6nhmjwvS has been greeted 14 time(s)");
    println!("Success");
    println!("");
}
