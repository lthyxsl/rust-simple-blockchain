use core::block_chain;
use std::{thread, time::Duration};

fn main() {
    let mut block_chain = block_chain::BlockChain::new();
    println!("starting mining....");
    thread::sleep(Duration::from_secs(5));
    block_chain.add_block(String::from("a -> b: 5 btc"));
    println!("produce a block....");
    thread::sleep(Duration::from_secs(2));
    println!("starting mining....");
    thread::sleep(Duration::from_secs(5));
    block_chain.add_block(String::from("c -> d: 10 btc"));
    println!("produce a block....");
    for block in block_chain.blocks {
        println!("----------------------------");
        println!("{:#?}", block);
        println!();
    }
}
