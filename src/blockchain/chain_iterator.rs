use leveldb::options::{Options, WriteOptions, ReadOptions};
use leveldb::db::Database;
use std::str;
use crate::blockchain::block::Block;

pub struct BlockchanIterator{
    pub current_hash: Vec<u8>,
    database: Database
}

impl BlockchanIterator{
    
    pub fn next(self) -> Block{
        let read_ops = ReadOptions::new();
        let block = match self.database.get(&read_ops, &self.current_hash){
            Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                Ok(v) => crate::blockchain::block::Block::deserialize(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            } },
            Err(e) => { panic!("failed to read from database: {:?}", e) }
        };
        self.current_hash = block.prev_block;
        return block
    }
    
}