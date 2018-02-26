extern crate secp256k1;
extern crate bitcoin;
extern crate rand;

// use std::str;
use std::string::String;
use bitcoin::util::address::Address;
use bitcoin::util::address::Privkey;
use bitcoin::util::base58::ToBase58;
use bitcoin::network::constants::Network;
use secp256k1::Secp256k1;


pub fn generate_address(nwork: &str) -> (String, String) {
    let network = match nwork {
        "bitcoin" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        _ => Network::Bitcoin,
    };

    let s = Secp256k1::new();
    let (sk, pk) = s.generate_keypair(&mut rand::thread_rng()).unwrap();
    let addr = Address::from_key(network, &pk, true);
    // let uncomr_addr = Address::from_key(network, &pk, false);
    let private_key = Privkey::from_key(network, sk, true);
    let comp_addr = addr.to_base58check();
    let comp_priv = private_key.to_base58check();
    // println!("{:?}", uncomr_addr);
    (comp_addr, comp_priv)
}
