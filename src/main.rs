use block::Block;
use blockchain::Blockchain;

mod block;
mod blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    mine_and_add_blocks(&mut blockchain, "Block_1".to_string());
    mine_and_add_blocks(&mut blockchain, "Block_2".to_string());
    mine_and_add_blocks(&mut blockchain, "Block_3".to_string());
    mine_and_add_blocks(&mut blockchain, "Block_4".to_string());
    
    let chain_validity = blockchain.validate_chain();
    println!("The chain validity is {}", chain_validity);

}


fn mine_and_add_blocks(blockchain: &mut Blockchain, data: String) {
    let last_block = blockchain.blocks.last().unwrap();
    let block_1 = Block::new(&last_block, "Hello");
    blockchain.add_new_block(block_1);
}

