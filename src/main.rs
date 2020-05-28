mod blockchainlib;

use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 500,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 10,
                },
            ],
        }],
        difficulty,
    );

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 500,
                }],
            },

            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 5,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 6,
                    },
                ],
            },
        ],
        difficulty,
    );

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    println!("Last hash {:x?}", last_hash);

    blockchain
        .update_with_block(block)
        .expect("Failed to add block");
}
