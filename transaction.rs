extern crate secp256k1;
extern crate bitcoin;
extern crate rand;
extern crate base58;

use std::str;
use std::string::String;
use bitcoin::util::address::Address;
use bitcoin::util::address::Privkey;
// use bitcoin::util::base58::ToBase58;
// use bitcoin::util::base58::FromBase58;
use bitcoin::network::constants::Network;
use secp256k1::Secp256k1;
use bitcoin::util::hash::{Hash160, Sha256dHash};
use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::blockdata::opcodes::All;
use base58::{ToBase58, FromBase58};
use bitcoin::blockdata::transaction::{TxIn, TxOut, Transaction, SigHashType};

pub fn transaction() {
    let network = Network::Bitcoin;
    let sample = "1NWrtbb8eF2M8y23FCSRe25VwWmDXSmPYx";
    let satoshi = 129_900_000;
    let prev_index = 0;
    let prev_tx = Sha256dHash::from_hex("526b548e30fe7ce9a0a861a159cf8e6e5cba5a31ff7756d4a819a4736d2bb4e0").unwrap();
    let sighash = SigHashType::All;

    let version = 1;
    let lock_time = 0;

    let s = Secp256k1::new();
    // let hsh = "ec00599f35c26479f74728a5a730e81b664c77c9";
    // let arra = sample.as_bytes();
    // let res = Address::from_base58check(sample);

    // let v: Vec<u8> = FromBase58::from_base58(sample).unwrap();
    let v = sample.from_base58().unwrap();
    let ss = &v[1..21];

    let b = Builder::new().push_opcode(All::OP_DUP).push_opcode(All::OP_HASH160).push_slice(ss).push_opcode(All::OP_EQUALVERIFY)
        .push_opcode(All::OP_CHECKSIG);
    let script = b.into_script();
    println!("{:?}", script);

    let txout = TxOut {
        value: satoshi,
        script_pubkey: script
    };

    println!("{:?}", txout);

    let mut txin = TxIn {
        prev_hash: prev_tx,
        prev_index: prev_index,
        sequence: 0xFFFFFFFF,
        script_sig: Script::new(),
    };

    println!("{:?}", txin);


    let mut transaction = Transaction {
        version: version,
        lock_time: lock_time,
        input: vec![txin],
        output: vec![txout],
        witness: vec![],
    };

    println!("{:?}", transaction);

    println!("--------------");
    println!("{:?}", transaction.txid());


    //let signature = transaction.signature_hash(prev_index, &script, sighash);


    // println!("{:?}", &v[1..21]);
    // let sdsd = String::from_utf8(v[1..]);
    // let something = str::from_utf8(&v).unwrap();
    // println!("{:?}", something);

    // let soe = "007EAA972FB63888A59267E973FF8A9D86950CB6900E3BDCA3";
    // let wewe = Address::from_base58(&soe.as_bytes());

    // println!("{:?}", wewe);

    // let something = Hash160::from_data(&sample.as_bytes());

    // println!("{:?}", something);

    // let addr = Address::from_key(network, &pk, true);

}
