extern crate secp256k1;
extern crate bitcoin;
extern crate rand;
extern crate base58;

use std::io;
use std::env;
mod address;
mod transaction;

fn options() -> i32 {
    loop {
        println!("\n1) Generate a new address");
        println!("2) List all addresses");
        println!("3) List unused addresses");
        println!("4) Create a transaction");
        println!("0) Exit\n");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(num) => {
                if num > -1 && num < 5 {
                    return num
                } else {
                    println!("\nEnter a valid option!\n");
                    -1
                }
            },
            Err(_) => {
                println!("\nEnter a valid option!\n");
                -1
            },
        };
    }
}


fn main() {
    // let r = address::generate_address("testnet");
    // Collect any command line arguments
    // let args: Vec<String> = env::args().collect();
    // let argmnt = args.get(1);
    //
    // let network = match argmnt {
    //     Some(t) => t,
    //     None => "bitcoin",
    // };
    //
    // if network == "testnet" {
    //     println!("Running in testnet mode...");
    // }
    //
    // println!("Welcome to Rusty Wallet!");
    // loop {
    //     let r = options();
    //     match r {
    //         0 => {
    //             println!("\nThank you for using Rusty Wallet! Have a nice day!\n");
    //             break
    //         },
    //         // Generate a new address...
    //         1 => {
    //             let (addr, private_key) = address::generate_address(network);
    //             println!("\nGenerating new address....");
    //             println!("New address generated!\nPublic address: {}\nPrivate key: {}", addr, private_key);
    //             continue
    //         },
    //         2 => continue,
    //         3 => continue,
    //         4 => continue,
    //         _ => continue,
    //     }
    // }

    transaction::transaction();

}
