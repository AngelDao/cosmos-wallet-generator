use cosmrs::{
    crypto::secp256k1,
};
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryFrom;
use bip39::{Mnemonic, MnemonicType, Language, Seed};
use bip32::{ExtendedPrivateKey, PrivateKey, XPrv};

fn main() {
    let mut file = File::open("mnemonic.txt").expect("cant open file");
    let mut key = String::new();
    file.read_to_string(&mut key)
        .expect("Oops!, cannot read file...");
    let bytes: &[u8] = &key.as_bytes();
    let mnemonic = Mnemonic::from_phrase(&key, Language::English).unwrap();
    println!("{:#?}", mnemonic);
    let seed = Seed::new(&mnemonic, "");
    println!("{:#?}", seed);
    let root_privk = XPrv::new(&seed).unwrap();
    // evmos "m/44'/60'/0'/0/0"
    // cosmos "m/44'/118'/0'/0/0"
    let privk = XPrv::derive_from_path(&seed, &"m/44'/60'/0'/0/0".parse().unwrap()).unwrap();
    println!("{:#?}", &privk.private_key());
    let bytes2 = privk.private_key().to_bytes();
    println!("bytes2 {:#?}", &bytes2);
    let sender_private_key = secp256k1::SigningKey::from_bytes(bytes2.as_slice()).unwrap();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id("evmos").unwrap();
    println!(
        "pubkey {:#?}\naccountid{:#?}",
        sender_public_key, sender_account_id
    );
}
