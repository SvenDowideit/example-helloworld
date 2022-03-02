use anyhow::anyhow;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::transaction::Transaction;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

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
        payer_balance / 1000000000
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
    let greeted_balance = client.get_balance(&greeted).unwrap();
    println!(
        "Saying hello to {:?}: balance: {:?}",
        greeted,
        greeted_balance / 1000000000
    );

    // println!("FoSMxh52TDgDAjkpnCSDuDbLwxmAugvzof1T6nhmjwvS has been greeted 14 time(s)");

    // let lamports = 33;
    // let latest_blockhash = client.get_latest_blockhash().unwrap();
    // let tx = system_transaction::transfer(&payer_key, &greeted, lamports, latest_blockhash);
    // let signature = client.send_and_confirm_transaction(&tx).unwrap();
    // println!("Success: {:?}", signature);

    let g = GreetingAccount { counter: 0 };
    let instr = Instruction::new_with_borsh(
        program_key.pubkey(),
        &g,
        vec![AccountMeta::new(greeted, false)],
    );
    let blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[instr],
        Some(&payer_key.pubkey()),
        &[&payer_key],
        blockhash,
    );
    let signature = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Success: {:?}", signature);

    let greeted_account = client.get_account(&greeted).unwrap();
    let data = GreetingAccount::try_from_slice(&greeted_account.data).unwrap();
    println!("{:?} has been greeted {:?} time(s)!", greeted, data.counter);

    println!("");
}
