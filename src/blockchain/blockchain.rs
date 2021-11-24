extern crate leveldb;
extern crate tempdir;

use std::str;
use crate::wallet::wallet::Wallet;
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::chain_iterator::BlockchanIterator;
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
    pub acc: AccDB
}

impl Blockchain{
    pub fn get_blockchain_instance(last_hash: Vec<u8>, db: &Database, acc: &AccDB) -> Self{

    }

    pub fn add_block(&self, block: &Block){
        let read_ops = ReadOptions::new();
        let write_ops = WriteOptions::new();
        
        match self.bcdb.get(&read_ops, &block.hash()){
            Ok(_) => return,
            _ =>  {} 
        }

        match self.bcdb.put(&write_ops, &block.hash(), &block.serialize().as_bytes()[..]){
            Ok(_) => { () },
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };
        
        let last_hash = match self.bcdb.get(&read_ops, &"lh".as_bytes()){
            Ok(h) => {h},
            Err(e) =>  {println!("Error on getting last hash {:?}", e)}
        };

        let last_block = match self.bcdb.get(&read_ops, &last_hash){
            Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                Ok(v) => crate::blockchain::block::Block::deserialize(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            } },
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };

        if u64::from_le_bytes(Self::clone_into_array(&block.height[..])) > u64::from_le_bytes(Self::clone_into_array(&last_block.height[..])){
            match self.bcdb.put(&write_ops, &"lh".as_bytes(), &block.hash()){
                Ok(_) => { () },
                Err(e) => { panic!("failed to write to database: {:?}", e) }
            };
            self.last_hash = block.hash()
        }

        self.acc.update_balance(&block)
    }

    pub fn get_block(&self, block_hash: &Vec<u8>) -> Block{
        let read_ops = ReadOptions::new();
        let block = match self.bcdb.get(&read_ops, block_hash){
            Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                Ok(v) => crate::blockchain::block::Block::deserialize(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            } },
            Err(e) => { panic!("failed to read from database: {:?}", e) }
        };
        block
    }

    //pub fn get_block_hashes(&self) -> Vec<Vec<u8>>{

    //} 

    pub fn iterator(&self) -> BlockchanIterator{
        BlockchanIterator{
            current_hash: self.last_hash,
            database: self.bcdb
        } 
    }

    pub fn get_best_height(&self) -> u64{
        let read_ops = ReadOptions::new();
        let last_hash = match self.bcdb.get(&read_ops, &"lh".as_bytes()){
            Ok(h) => {h},
            Err(e) =>  {println!("Error on getting last hash {:?}", e)}
        };

        let last_block = match self.bcdb.get(&read_ops, &last_hash){
            Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                Ok(v) => crate::blockchain::block::Block::deserialize(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            } },
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };

        return u64::from_le_bytes(Self::clone_into_array(&last_block.height[..]))
    }

    pub fn get_last_hash(&self) -> Vec<u8>{
        let read_ops = ReadOptions::new();
        let last_hash = match self.bcdb.get(&read_ops, &"lh".as_bytes()){
            Ok(h) => {h},
            Err(e) =>  {println!("Error on getting last hash {:?}", e)}
        };

        let last_block = match self.bcdb.get(&read_ops, &last_hash){
            Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                Ok(v) => crate::blockchain::block::Block::deserialize(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            } },
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };
        last_block.hash()
    }

    // pub fn mine_block(&self, txs: &Vec<Transaction>) -> Block{

    // }

    pub fn db_exists(path: &str) -> bool{
        Path::new(path).exists()
    }

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
        
        
        let acc = AccDB::init_accounts(&w.address());

        Blockchain{
            last_hash: last_hash,
            bcdb: database,
            acc: acc
        }
    }

    pub fn continue_blockchain(db_path: &str) -> Self{
        if Self::db_exists(db_path) == false{
            panic!("no existing blockchain found, create one!")
        }
        let path = Path::new(db_path);
        let mut options = Options::new();
        options.create_if_missing = true;

        let database = match Database::open(&path, &options){
            Ok(db) => { db },
            Err(e) => { panic!("failed to open database: {:?}", e) }
        };
        let read_ops = ReadOptions::new();

        let last_hash = match database.get(&read_ops, &"lh".as_bytes()){
            Ok(h) => {h},
            Err(e) =>  {println!("Error on getting last hash {:?}", e)}
        };

        let acc = AccDB::get_accounts();
        let chain = Self::get_blockchain_instance(last_hash: Vec<u8>);
        return chain
    }

    pub fn find_transaction(&self, id: Vec<u8>) -> Result<Transaction, Error>{
        let iter = self.iterator();

        loop{
            let block = iter.next();
            for tx in block.transactions{
                if tx.id() == id{
                    Ok(tx)
                }
            }
            if block.prev_block.len() == 0{
                break
            }
        }
        return Err(_)
    }

    pub fn verify_transaction(&self, tx: &Transaction) -> bool{
        if tx.is_coinbase() {
            return true;
        }

        let (balance, _) = self.acc.balance_and_nonce(&Wallet::pkh_to_address(Wallet::pk_to_pkh(&tx.pubkey)));
        let id = tx.id();
        if !Transaction::verify_signature(id, tx.pubkey, tx.sig.unwrap()){
            println!("\ninvalid signature");
            return false
        } else if balance < tx.value {
            return false
        }
        return true
    }
    fn clone_into_array<A, T>(slice: &[T]) -> A
    where
        A: Default + AsMut<[T]>,
        T: Clone,
    {
        let mut a = A::default();
        <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
        a
    }
}