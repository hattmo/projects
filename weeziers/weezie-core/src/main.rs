#![cfg_attr(feature = "shellcode", no_main)]
#![cfg_attr(feature = "shellcode", no_std)]
#![cfg_attr(feature = "shellcode", feature(lang_items))]

#[cfg(feature = "shellcode")]
mod entry;
#[cfg(feature = "shellcode")]
use std::prelude::*;

fn main() {
    use ed25519_dalek::{Keypair, Signer, Verifier};
    use rand::thread_rng;

    let mut csprng = thread_rng();
    let keypair = Keypair::generate(&mut csprng);
    let sig = keypair.sign("hello world".as_bytes());
    let sig_bytes = sig.to_bytes();
    println!("Signature: {:?} len: {}", sig_bytes, sig_bytes.len());
    if keypair.verify("hello world".as_bytes(), &sig).is_ok() {
        println!("Signature verified!");
    } else {
        println!("Signature not verified!");
    };
    serde_json::to_writer(std::io::stdout(), sig.to_bytes().as_ref()).unwrap();
}
