#[cfg(test)]
mod client {
    use borsh::BorshSerialize;
    use notepad::instruction::NotepadInstructionPayload;
    use solana_program::{instruction, pubkey};
    use solana_rpc_client::rpc_client;
    use solana_sdk::{signature::Signer, signer::keypair, system_program, transaction};
    use std::str::FromStr;

    const RPC_ADDR: &str = "https://neat-twilight-seed.solana-devnet.quiknode.pro/f5b58eab34b737385de4001f1eddd79b998e3e29/";
    // const RPC_ADDR: &str = "https://api.devnet.solana.com";

    #[test]
    fn note_create_test() {
        let program_id =
            pubkey::Pubkey::from_str("CpfNZzf12jZsevQci24LkQXsSEKjdrGpXKmz4H4wDyLE").unwrap();
        let payer =
        keypair::Keypair::from_base58_string("5JowcAzn1Kg2sw4WPCtTRuwVW1XMMDKoLx1Qj3Q6D39yThpHXr3Fhj7wPmbE22jqKDMqKgm36rdTYKgv1wkHbnWJ");
        let payee = keypair::Keypair::from_base58_string("4WyNHzz6x3YfNw3TYQi784f7JXkzc6DgNkdAynitDq62qnFZcEqHHZkmCQp4MsD2F1HUKTPMbzTLNkUJzcUm2knQ");
        let accounts = vec![
            instruction::AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: payee.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: program_id,
                is_signer: false,
                is_writable: true,
            },
        ];

        let mut data: Vec<u8> = Vec::new();
        let tag = 0u8;
        data.push(tag);

        let payload = NotepadInstructionPayload {
            title: String::from("My"),
            body: String::from("This."),
            pubkey: pubkey::Pubkey::from_str("1eo33hEuvUBEhj69BU78M36YhuSruKVymKYz7AH7Zok")
                .unwrap(),
        };
        payload.serialize(&mut data).unwrap();

        let ins = instruction::Instruction::new_with_bytes(program_id, &data, accounts);

        let client = rpc_client::RpcClient::new(RPC_ADDR);
        let latest_blockhash = client.get_latest_blockhash().unwrap();

        let tx = client
            .send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
                &vec![ins],
                Some(&payer.pubkey()),
                &[&payer, &payee],
                latest_blockhash,
            ))
            .unwrap();
        println!("tx:{}", tx);
    }
}
