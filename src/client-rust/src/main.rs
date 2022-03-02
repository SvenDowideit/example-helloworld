use anyhow::anyhow;
use solana_cli_config::Config;

fn main() {
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

    // TODO: connect

    println!(
        "Connection to cluster established: {:?} (connection info)",
        cli_config.json_rpc_url
    );

    // // get payer keypair from config.keypair_path
    // println!("Using account CG4hWYmpXGe9TE3BBM9kdMzEBhugEocqAMvHxpW9jF7a containing <SOL> SOL to pay for fees");
    // // get programid from ../../dist/program/helloworld-keypair.json (created by ts client for now)
    // println!("Using program 5gBQEUwAnt8qowhMP1ZmXMnTLnRbz4CUzwVCzXo1CWdz");
    // // get greeting account - key derived from "hello" so its easy to find.
    // println!("Saying hello to FoSMxh52TDgDAjkpnCSDuDbLwxmAugvzof1T6nhmjwvS");
    // println!("FoSMxh52TDgDAjkpnCSDuDbLwxmAugvzof1T6nhmjwvS has been greeted 14 time(s)");
    // println!("Success");
}
