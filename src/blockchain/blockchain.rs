extern crate leveldb;
extern crate tempdir;

use std::str;
use crate::wallet::wallet::Wallet;
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::account::AccDB;
use leveldb::options::{Options, WriteOptions, ReadOptions};
use leveldb::db::Database;
use leveldb::error::Error;
use leveldb::util::FromU8;
use leveldb::iterator::Iterable;
use std::path::Path;

pub struct Blockchain{
    last_hash: Vec<u8>,
    bcdb: Database,
    accdb: AccDB
}

impl Blockchain{
    //this function needs to receive the lasthash, blockchain db and accounts db
    // pub fn get_blockchain_instance(last_hash: Vec<u8>) -> Self{

    // }

    // pub fn add_block(&self, block: &Block){

    // }

    // pub fn get_block(&self, block_hash: &Vec<u8>) -> Block{

    // }

    // pub fn get_block_hashes(&self) -> Vec<Vec<u8>>{

    // } 

    // pub fn get_best_height(&self) -> u64{

    // }

    // pub fn get_last_hash(&self) -> Vec<u8>{

    // }

    // pub fn mine_block(&self, txs: &Vec<Transaction>) -> Block{

    // }

    // pub fn db_exists(path: &str) -> bool{

    // }

    pub fn init_blockchain(w: &Wallet, dbpath: &str) -> Self{
        let path = Path::new(dbpath);
        let mut options = Options::new();
        options.create_if_missing = true;

        let database = match Database::open(&path, &options){
            Ok(db) => { db },
            Err(e) => { panic!("failed to open database: {:?}", e) }
        };
        

        let write_ops = WriteOptions::new();
        

        let cbtx = Transaction::coinbase_tx(w);
        let genesis = Block::genesis(&cbtx);

        match database.put(&write_ops, &genesis.hash(), &genesis.serialize().as_bytes()[..]){
            Ok(_) => { () },
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };
        
        match database.put(&write_ops, &"lh".as_bytes(), &genesis.serialize().as_bytes()[..]){
            Ok(_) => { () },
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };
        
        let last_hash = genesis.hash();
        
        let saddress = match str::from_utf8(&w.address()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let acc = AccDB::init_accounts(saddress);

        Blockchain{
            last_hash: last_hash,
            bcdb: database,
            accdb: acc
        }
    }

    // pub fn continue_blockchain(db_path: &str) -> Self{

    // }

    // pub fn find_transaction(&self, id: Vec<u8>) -> Transaction{

    // }

    // pub fn verify_transaction(&self, tx: &Transaction) -> bool{

    // }
}