use cosmrs::{
    bank::MsgSend,
    crypto::secp256k1,
    tx::{self, Fee, Msg, SignDoc, SignerInfo, Tx},
    AccountId, Coin,
};
use hex::decode;
use k256::ecdsa::VerifyingKey;
use rand_core::OsRng;
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryFrom;
// impl std::fmt::Debug for secp256k1::SigningKey {
//     fn fmt(&self) -> String {
//         format!("{:?}", self)
//     }
// }

fn main() {
    let mut file = File::open("seed.txt").expect("cant open file");
    let mut key = String::new();
    file.read_to_string(&mut key)
        .expect("Oops!, cannot read file...");
    // println!("{:#?}", &key.as_bytes());
    let bytes: [u8, 32] = key.as_bytes();
    // let s = String::from("hello0").unwrap();
    // let pkey = &decode(bytes).unwrap();
    println!("{:#?}", &bytes);
    // let sender_private_key = k256::ecdsa::SigningKey::try_from(key.as_bytes());
    // println!("privkey {:#?}", sender_private_key);
    let sender_private_key = secp256k1::SigningKey::from_bytes(pkey);
    match sender_private_key {
        Ok(res) => {
            // let sender_public_key = res.public_key();
            // let sender_account_id = sender_public_key.account_id("cosmos").unwrap();
            // println!(
            //     "pubkey {:#?}\naccountid{:#?}",
            //     sender_public_key, sender_account_id
            // );
        }
        Err(err) => println!("{:#?}", err),
    }

    // println!("privkey {:#?}", sender_private_key);
}
