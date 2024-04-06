fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use borsh::BorshSerialize;
    use program::instruction::NotepadInstructionPayload;
    use solana_program::{instruction, pubkey, system_program};
    use solana_rpc_client::rpc_client;
    use solana_sdk::{signature::Signer, signer::keypair, transaction};
    use std::str::FromStr;

    // const RPC_ADDR: &str = "https://api.devnet.solana.com";
    const RPC_ADDR: &str = "https://neat-twilight-seed.solana-devnet.quiknode.pro/f5b58eab34b737385de4001f1eddd79b998e3e29/";

    /*
    笔记创建测试
    */
    #[test]
    fn note_create_test() {
        let program_id =
            pubkey::Pubkey::from_str("38gB1TY5CsjJua8dd7VdwzAAxNCV1n7sugAcvvPJ1Ukz").unwrap();
        let payer =
        keypair::Keypair::from_base58_string("5JowcAzn1Kg2sw4WPCtTRuwVW1XMMDKoLx1Qj3Q6D39yThpHXr3Fhj7wPmbE22jqKDMqKgm36rdTYKgv1wkHbnWJ"); //payer既是gas费支付方，又是笔记用户
        let note = keypair::Keypair::new(); //笔记对象
        println!("note pubkey in base58: {}", note.to_base58_string());

        let accounts = vec![
            instruction::AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: note.pubkey(),
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
            contents: String::from("Hello."),
            pubkey: payer.pubkey(),
        };
        payload.serialize(&mut data).unwrap();
        let ins = instruction::Instruction::new_with_bytes(program_id, &data, accounts);
        print!("{:?}", ins.accounts);

        let client = rpc_client::RpcClient::new(RPC_ADDR);
        let latest_blockhash = client.get_latest_blockhash().unwrap();

        let result =
            client.send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
                &vec![ins],
                Some(&payer.pubkey()),
                &[&payer, &note],
                latest_blockhash,
            ));

        assert!(result.is_ok());
        println!("tx: {}", result.unwrap());
    }

    /*
    笔记更新测试
    */
    #[test]
    fn note_update_test() {
        let program_id =
            pubkey::Pubkey::from_str("38gB1TY5CsjJua8dd7VdwzAAxNCV1n7sugAcvvPJ1Ukz").unwrap();
        let payer =
            keypair::Keypair::from_base58_string("5JowcAzn1Kg2sw4WPCtTRuwVW1XMMDKoLx1Qj3Q6D39yThpHXr3Fhj7wPmbE22jqKDMqKgm36rdTYKgv1wkHbnWJ");
        let note = keypair::Keypair::from_base58_string("C4454d6a3coUscSxVqweNid92U7tNQyABUzCFnC5BEqzuoXQNi61gzwRN3YiDCDAsXQ6wdDzenkXty9yZSn7XPu"); //笔记对象，需要跟创建时候的keypair匹配

        let accounts = vec![
            instruction::AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: note.pubkey(),
                is_signer: true,
                is_writable: true,
            },
        ];

        let mut data: Vec<u8> = Vec::new();
        let tag = 1u8;
        data.push(tag);

        let payload = NotepadInstructionPayload {
            contents: String::from("update my note."),
            pubkey: payer.pubkey(),
        };
        payload.serialize(&mut data).unwrap();
        let ins = instruction::Instruction::new_with_bytes(program_id, &data, accounts);
        print!("{:?}", ins.accounts);

        let client = rpc_client::RpcClient::new(RPC_ADDR);
        let latest_blockhash = client.get_latest_blockhash().unwrap();

        let result =
            client.send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
                &vec![ins],
                Some(&payer.pubkey()),
                &[&payer, &note],
                latest_blockhash,
            ));

        assert!(result.is_ok());
        println!("tx: {}", result.unwrap());
    }

    /*
    笔记删除测试
    */
    #[test]
    fn note_delete_test() {
        let program_id =
            pubkey::Pubkey::from_str("6kxMCGKMkX96nqL6L99uZCqXrvBMQuTAtWfs8dGG2smw").unwrap();
        let payer =
                keypair::Keypair::from_base58_string("5JowcAzn1Kg2sw4WPCtTRuwVW1XMMDKoLx1Qj3Q6D39yThpHXr3Fhj7wPmbE22jqKDMqKgm36rdTYKgv1wkHbnWJ");
        let note = keypair::Keypair::from_base58_string("5STQAqjChyFhXRbeS55xX4fJ5XRCYuyesBr58nP8Mnu7tusHKPfowQ6nDctkFCExK3NLD3hCL3v2ApUUvUonpYnV");

        let accounts = vec![
            instruction::AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: note.pubkey(),
                is_signer: true,
                is_writable: true,
            },
        ];

        let mut data: Vec<u8> = Vec::new();
        let tag = 2u8;
        data.push(tag);
        let ins = instruction::Instruction::new_with_bytes(program_id, &data, accounts);

        let client = rpc_client::RpcClient::new(RPC_ADDR);
        let latest_blockhash = client.get_latest_blockhash().unwrap();

        let result =
            client.send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
                &vec![ins],
                Some(&payer.pubkey()),
                &[&payer, &note],
                latest_blockhash,
            ));

        assert!(result.is_ok());
        println!("tx: {}", result.unwrap());
    }

    /*
    笔记输入异常测试
    */
    #[test]
    fn note_invalid_contents_len_test() {
        let program_id =
            pubkey::Pubkey::from_str("38gB1TY5CsjJua8dd7VdwzAAxNCV1n7sugAcvvPJ1Ukz").unwrap();
        let payer =
                    keypair::Keypair::from_base58_string("5JowcAzn1Kg2sw4WPCtTRuwVW1XMMDKoLx1Qj3Q6D39yThpHXr3Fhj7wPmbE22jqKDMqKgm36rdTYKgv1wkHbnWJ");
        let note = keypair::Keypair::from_base58_string("C4454d6a3coUscSxVqweNid92U7tNQyABUzCFnC5BEqzuoXQNi61gzwRN3YiDCDAsXQ6wdDzenkXty9yZSn7XPu");

        let accounts = vec![
            instruction::AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: note.pubkey(),
                is_signer: true,
                is_writable: true,
            },
        ];

        let mut data: Vec<u8> = Vec::new();
        let tag = 1u8;
        data.push(tag);

        let payload = NotepadInstructionPayload {
            contents: String::from(
                "An aircraft carrier is a warship that serves as a seagoing airbase, equipped with a full-length flight deck and facilities for carrying, arming, deploying, and recovering aircraft.[1] Typically, it is the capital ship of a fleet, as it allows a naval force to project air power worldwide without depending on local bases for staging aircraft operations. Carriers have evolved since their inception in the early twentieth century from wooden vessels used to deploy balloons to nuclear-powered warships that carry numerous fighters, strike aircraft, helicopters, and other types of aircraft. While heavier aircraft such as fixed-wing gunships and bombers have been launched from aircraft carriers, these aircraft have not landed on a carrier. By its diplomatic and tactical power, its mobility, its autonomy and the variety of its means, the aircraft carrier is often the centerpiece of modern combat fleets. ",
            ),
            pubkey: payer.pubkey(),
        };
        payload.serialize(&mut data).unwrap();
        let ins = instruction::Instruction::new_with_bytes(program_id, &data, accounts);
        print!("{:?}", ins.accounts);

        let client = rpc_client::RpcClient::new(RPC_ADDR);
        let latest_blockhash = client.get_latest_blockhash().unwrap();

        let result =
            client.send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
                &vec![ins],
                Some(&payer.pubkey()),
                &[&payer, &note],
                latest_blockhash,
            ));

        assert_eq!(result.is_err(), true);
        println!("error: {:?} ", result.unwrap_err());
    }

    /*
    笔记权限异常测试
    */
    #[test]
    fn note_invalid_pubkey_test() {
        let program_id =
            pubkey::Pubkey::from_str("38gB1TY5CsjJua8dd7VdwzAAxNCV1n7sugAcvvPJ1Ukz").unwrap();
        let payer =
                keypair::Keypair::from_base58_string("29dUnwyQgtGBgNFHNkS35bDwDBnbWEAPVSzFSvLi9VHb8St3MHZpP3SKcXphXDKSBiAboAYk68Si4FhpfXunNf3v");
        let note = keypair::Keypair::from_base58_string("C4454d6a3coUscSxVqweNid92U7tNQyABUzCFnC5BEqzuoXQNi61gzwRN3YiDCDAsXQ6wdDzenkXty9yZSn7XPu");

        let accounts = vec![
            instruction::AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            instruction::AccountMeta {
                pubkey: note.pubkey(),
                is_signer: true,
                is_writable: true,
            },
        ];

        let mut data: Vec<u8> = Vec::new();
        let tag = 1u8;
        data.push(tag);

        let payload = NotepadInstructionPayload {
            contents: String::from("An aircraft carrier"),
            pubkey: payer.pubkey(),
        };
        payload.serialize(&mut data).unwrap();
        let ins = instruction::Instruction::new_with_bytes(program_id, &data, accounts);
        print!("{:?}", ins.accounts);

        let client = rpc_client::RpcClient::new(RPC_ADDR);
        let latest_blockhash = client.get_latest_blockhash().unwrap();

        let result =
            client.send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
                &vec![ins],
                Some(&payer.pubkey()),
                &[&payer, &note],
                latest_blockhash,
            ));

        assert_eq!(result.is_err(), true);
        println!("error: {:?} ", result.unwrap_err());
    }
}
