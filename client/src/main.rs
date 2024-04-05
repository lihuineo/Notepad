// const RPC_ADDR: &str = "https://api.devnet.solana.com";
fn main() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use borsh::BorshSerialize;
    use program::instruction::NotepadInstructionPayload;
    use solana_program::{instruction, pubkey, system_program};
    use solana_rpc_client::rpc_client;
    use solana_sdk::{signature::Signer, signer::keypair, transaction};
    use std::str::FromStr;

    const RPC_ADDR: &str = "https://neat-twilight-seed.solana-devnet.quiknode.pro/f5b58eab34b737385de4001f1eddd79b998e3e29/";
    // const RPC_ADDR: &str = "https://api.devnet.solana.com";

    #[test]
    fn note_create_test() {
        let program_id =
            pubkey::Pubkey::from_str("6yAWkkNFf51mNKBANvGekWv6SXx7KwLuGgTrHdHQ27b5").unwrap();
        let payer =
        keypair::Keypair::from_base58_string("5JowcAzn1Kg2sw4WPCtTRuwVW1XMMDKoLx1Qj3Q6D39yThpHXr3Fhj7wPmbE22jqKDMqKgm36rdTYKgv1wkHbnWJ");
        let user = keypair::Keypair::new();
        println!("user pubkey: {}", user.pubkey().to_string());

        let accounts = vec![
            instruction::AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: user.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: system_program::ID,
                is_signer: false,
                is_writable: true,
            },
        ];

        let mut data: Vec<u8> = Vec::new();
        let tag = 0u8;
        data.push(tag);

        let payload = NotepadInstructionPayload {
            title: String::from("Hi new world."),
            body: String::from("This is my first note."),
            pubkey: user.pubkey(),
        };
        payload.serialize(&mut data).unwrap();
        let ins = instruction::Instruction::new_with_bytes(program_id, &data, accounts);
        print!("{:?}", ins.accounts);

        let client = rpc_client::RpcClient::new(RPC_ADDR);
        let latest_blockhash = client.get_latest_blockhash().unwrap();

        let tx = client
            .send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
                &vec![ins],
                Some(&payer.pubkey()),
                &[&payer, &user],
                latest_blockhash,
            ))
            .unwrap();

        println!("tx:{}", tx);
    }
}
