mod programs;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use bs58;
    use std::io::{self, BufRead};
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{self, message::Message, pubkey::Pubkey, signature::{read_keypair_file, Keypair}, signer::Signer, system_instruction::transfer, system_program, transaction::Transaction};
    use crate::programs::turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs };

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("cant find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs {
            github: b"suite".to_vec()
        };
        let blockhash = rpc_client.get_latest_blockhash().expect("failed to get latest blockhash");
        let tx = Turbin3PrereqProgram::complete(&[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()), &[&signer], blockhash);

        let sig = rpc_client.send_and_confirm_transaction(&tx).expect("failed to send tx");
        println!("Success! https://explorer.solana.com/tx/{}/?cluster=devnet", sig);
    }

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); 
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap(); 
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:"); let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',') .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string(); 
        println!("{:?}", base58);
    }

    #[test] 
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()) 
        };
    } 

    #[test] 
    fn transfer_sol() {
        // Send 0.1 SOL to Turbin3 wallet
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("DxN7Tawr7eyzYYGgJEojQZnaLu6bDw4wDK8oiAzVUnZL").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(&[transfer(
            &keypair.pubkey(), &to_pubkey, 100_000_000
            )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
                "Sent 0.1 SOL! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
        );

        // Empty wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let message = Message::new_with_blockhash(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance,
            )], Some(&keypair.pubkey()), &recent_blockhash
        );

        let fee= rpc_client.get_fee_for_message(&message) .expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
&[transfer( &keypair.pubkey(), &to_pubkey, balance - fee,
            )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
                "Emptied wallet! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
        );
    }
}
