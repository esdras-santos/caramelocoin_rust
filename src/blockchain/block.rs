use sha2::{Sha256, Digest};
use serde_derive::{Deserialize, Serialize};
use num_bigint::{BigUint,ToBigUint};
use num::pow::pow;
use std::time::{UNIX_EPOCH, SystemTime};
use crate::blockchain::transaction::Transaction;
use crate::blockchain::merkle;
use crate::blockchain::proof::ProofOfWork;

#[derive(Deserialize, Serialize)]
pub struct Block{
    version: Vec<u8>,
    prev_block: Vec<u8>,
    merkle_root: Option<Vec<u8>>,
    timestamp: Vec<u8>,
    bits: Vec<u8>,
    pub nonce: Option<Vec<u8>>,
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

    pub fn create_block(txs: &Vec<Transaction>, prev_hash: &Vec<u8>, height: &u64) -> Self{
         let bits =  ProofOfWork::get_bits(&height, &prev_hash);
         let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs() * 1000,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        let block = Self{
            version: vec![0x00000001],
            prev_block: prev_hash.to_vec(),
            merkle_root: None,
            timestamp: Transaction::to_bytes(&ts),
            bits: bits,
            nonce: None,
            height: Transaction::to_bytes(height),
            transactions: txs.to_vec(),
        }; 
        block.merkle_root = Some(block.hash_transactions());
        let pow = Self::new_proof(block);
        let nonce = pow.run();
        block.nonce = Some(Transaction::to_bytes(&nonce));
        block
    }

    pub fn genesis(coinbase: &Transaction) -> Self{
        return Self::create_block(vec![coinbase], vec![0x00], &0);
    }

    pub fn serialize(&self) -> String{
        let v = serde_json::to_string(&self).unwrap();
        return v;
    }

    pub fn deserialize(s: &str) -> Self{
        let block: Block = serde_json::from_str(s).unwrap();
        block
    }

    pub fn hash(&self) -> Vec<u8>{
        let data: Vec<u8>;
        data.append(&mut self.version);
        data.append(&mut self.prev_block);
        data.append(&mut self.merkle_root.unwrap());
        data.append(&mut self.bits);
        data.append(&mut self.nonce.unwrap());
        data.append(&mut self.height);

        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        Vec::from(&hash[..])
    }

    pub fn new_proof(block: Block) -> ProofOfWork{
        ProofOfWork{
            block: block
        }
    }

    pub fn difficulty(&self) -> BigUint{
        let lowest: BigUint = (65535 * pow(256, 26)).to_biguint().unwrap();
        return lowest / self.target() 
    }

    pub fn target(&self) -> BigUint{
        let target = ProofOfWork::bits_to_target(&self.bits);
        target
    }


}