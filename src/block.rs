use chrono::Utc;
use sha256::digest;

#[derive(Clone, Debug)]
pub struct Block {
    pub id: u32,
    pub nonce: u64,
    pub data: String,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(last_block: &Block, data: &str) -> Self {
        let block_id = last_block.id + 1;
        let mut nonce = 0;

        loop {
            let timestamp = Utc::now().timestamp();
            let hash = digest(format!(
                "{}{}{}{}{}",
                block_id,
                nonce,
                data.to_string(),
                timestamp,
                last_block.hash
            ));
            if hash.starts_with("0000") {
                return Self {
                    id: block_id,
                    nonce,
                    data: data.to_string(),
                    timestamp: timestamp,
                    previous_hash: last_block.clone().hash,
                    hash: hash,
                };
            }
			nonce += 1;
        }
    }

    pub fn genesis_block() -> Self {
        let genesis_id = 0;
        let mut nonce = 0;
        let previous_hash =
            "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let genesis_block_data = "Genesis Block".to_string();
        loop {
            let timestamp = Utc::now().timestamp();
            let hash = digest(format!(
                "{}{}{}{}{}",
                genesis_id,
                nonce,
                genesis_block_data.to_string(),
                timestamp,
                previous_hash
            ));

            if hash.starts_with("0000") {
                return Self {
                    id: genesis_id,
                    nonce,
                    data: genesis_block_data,
                    timestamp: timestamp,
                    previous_hash: previous_hash,
                    hash: hash,
                };
            }
			nonce += 1;
        }
    }
}

