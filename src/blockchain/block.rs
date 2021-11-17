use sha2::{Sha256, Digest};
use crate::blockchain::transaction::Transaction;
use crate::blockchain::merkle;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Block{
    version: Vec<u8>,
    prev_block: Vec<u8>,
    merkle_root: Vec<u8>,
    timestamp: Vec<u8>,
    bits: Vec<u8>,
    pub nonce: Vec<u8>,
    height: Vec<u8>,
    transactions: Vec<Transaction>
}

impl Block{
    pub fn hash_transactions(&self) -> Vec<u8>{
        let mut tx_hashes: Vec<Vec<u8>> = Vec::new();
        for tx in &self.transactions{
            let txb = tx.serialize();
            tx_hashes.push(Vec::from(txb.as_bytes()));
        }
        merkle::MerkleTree::new(&mut tx_hashes).root_node.data.unwrap()
    }

    // pub fn create_block(txs: Vec<Transaction>, prev_hash: Vec<u8>, height: u64) -> Self{
        
    // }
    pub fn serialize(&self) -> String{
        let v = serde_json::to_string(&self).unwrap();
        return v;
    }

    pub fn deserialize(s: &str) -> Block{
        let block: Block = serde_json::from_str(s).unwrap();
        block
    }

    pub fn target(&self) -> usize{
        
    }
}