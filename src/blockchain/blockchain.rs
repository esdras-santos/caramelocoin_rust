use crate::wallet::wallet::Wallet;
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;

pub struct Blockchain{
    last_hash: Vec<u8>
    //blockchain db
    //accounts db
}

impl Blockchain{
    //this function needs to receive the lasthash, blockchain db and accounts db
    pub fn get_blockchain_instance(last_hash: Vec<u8>) -> Self{

    }

    pub fn add_block(&self, block: &Block){

    }

    pub fn get_block(&self, block_hash: &Vec<u8>) -> Block{

    }

    pub fn get_block_hashes(&self) -> Vec<Vec<u8>>{

    } 

    pub fn get_best_height(&self) -> u64{

    }

    pub fn get_last_hash(&self) -> Vec<u8>{

    }

    pub fn mine_block(&self, txs: &Vec<Transaction>) -> Block{

    }

    pub fn db_exists(path: &str) -> bool{

    }

    pub fn init_blockchain(w: &Wallet, dbpath: &str) -> Self{

    }

    pub fn continue_blockchain(db_path: &str) -> Self{

    }

    pub fn find_transaction(&self, id: Vec<u8>) -> Transaction{

    }

    pub fn verify_transaction(&self, tx: &Transaction) -> bool{

    }
}