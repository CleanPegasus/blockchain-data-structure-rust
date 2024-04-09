use sha256::digest;

use super::block::Block;

#[derive(Debug)]
pub struct Blockchain {
  pub blocks: Vec<Block>
}

impl Blockchain {
  pub fn new() -> Self {
    let genesis_block = Block::genesis_block();
    Self {
      blocks: vec![genesis_block]
    }
  }

  pub fn add_new_block(&mut self, new_block: Block) -> bool {
    let last_block = self.blocks.last().unwrap();
    if Blockchain::verify_block(last_block, &new_block) {
      self.blocks.push(new_block);
      return true
    } else {
      return false
    }
  }

  pub fn verify_block(last_block: &Block, new_block: &Block) -> bool {
    let hash = digest(format!(
      "{}{}{}{}{}",
      new_block.id,
      new_block.nonce,
      new_block.data,
      new_block.timestamp,
      last_block.hash
    ));

    if hash == new_block.hash { return true };
    return false;
  }

  pub fn validate_chain(self) -> bool {
    for (index, _) in self.blocks.iter().enumerate() {
      if index == 0 {continue;}
      let current_block = self.blocks[index].clone();
      let previous_block = self.blocks[index - 1].clone();
      if Blockchain::verify_block(&previous_block, &current_block) {
        continue;
      } else {
        return false;
      }
    };
    return true
  }
}