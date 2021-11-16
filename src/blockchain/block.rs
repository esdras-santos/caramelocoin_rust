use sha2::{Sha256, Digest};
use crate::blockchain::transaction::Transaction;
use crate::blockchain::merkle;

pub struct Block{
    version: Vec<u8>,
    prev_block: Vec<u8>,
    merkle_root: Vec<u8>,
    timestamp: Vec<u8>,
    bits: Vec<u8>,
    nonce: Vec<u8>,
    height: Vec<u8>,
    transactions: Vec<Transaction>
}

impl Block{
    pub fn hash_transactions(&self) -> Vec<u8>{
        let tx_hashes: Vec<Vec<u8>>;
        for tx in self.transactions{
            let txb = tx.serialize();
            tx_hashes.push(Vec::from(txb.as_bytes()));
        }
        merkle::MerkleTree::new(tx_hashes).root_node.data.unwrap()
    }
    
    pub fn teste(){
        println!("teste")
    }

    // pub fn create_block(txs: Vec<Transaction>, prev_hash: Vec<u8>, height: u64) -> Self{
        
    // }
}