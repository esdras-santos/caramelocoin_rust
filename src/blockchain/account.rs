extern crate leveldb;

use std::str;
use serde_derive::{Deserialize, Serialize};
use leveldb::db::Database;
use leveldb::options::{Options, WriteOptions, ReadOptions};
use std::path::Path;
use crate::wallet::wallet::Wallet;
use crate::blockchain::block::Block;


#[derive(Deserialize, Serialize, Clone, Debug)]
struct Account{
    balance: u64,
    nonce: u64
}
impl Account{
    pub fn serialize(&self) -> String{
        let v = serde_json::to_string(&self).unwrap();
        return v;
    }

    pub fn deserialize(s: &str) -> Self{
        let acc: Account = serde_json::from_str(s).unwrap();
        acc
    }  
}

pub struct AccDB{
    acc: Database,
} 
impl AccDB{
    pub fn balance_and_nonce(&self, address: &Vec<u8>) -> (u64, u64) {
        let read_ops = ReadOptions::new();
        let acc = match self.acc.get(&read_ops, &Wallet::address_to_pkh(address)){
            Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                Ok(v) => crate::blockchain::account::Account::deserialize(v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            } },
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };
        return (acc.balance, acc.nonce);
    }

    pub fn update_balance(&self, b: &Block){
        let read_ops = ReadOptions::new();
        let write_ops = WriteOptions::new();
        for tx in b.transactions{
            if tx.is_coinbase() == true {  
                let recipient = match self.acc.get(&read_ops, &tx.recipient){
                    Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                        Ok(v) => crate::blockchain::account::Account::deserialize(v),
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    } },
                    Err(e) => { panic!("failed to read to database: {:?}", e) }
                };
                recipient.balance += tx.value;
                
                match self.acc.put(&write_ops, &tx.recipient, &recipient.serialize().as_bytes()[..]){
                    Ok(_) => { () },
                    Err(e) => { panic!("failed to write to database: {:?}", e) }
                };
            } else {
                match self.acc.get(&read_ops, &tx.recipient){
                    Ok(d) => {
                        let recipient = match self.acc.get(&read_ops, &tx.recipient){
                            Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                                Ok(v) => crate::blockchain::account::Account::deserialize(v),
                                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                            } },
                            Err(e) => { panic!("failed to read to database: {:?}", e) }
                        };
                        recipient.balance = recipient.balance + tx.value - 1;
                        match self.acc.put(&write_ops, &tx.recipient, &recipient.serialize().as_bytes()[..]){
                            Ok(_) => { () },
                            Err(e) => { panic!("failed to write to database: {:?}", e) }
                        };
                    },
                    Err(e) =>{
                        let acc = crate::blockchain::account::Account{
                            balance: tx.value,
                            nonce: 0,
                        };
                        match self.acc.put(&write_ops, &tx.recipient, &acc.serialize().as_bytes()[..]){
                            Ok(_) => { () },
                            Err(e) => { panic!("failed to write to database: {:?}", e) }
                        };
                    }
                }

                let sender = match self.acc.get(&read_ops, &Wallet::pk_to_pkh(&tx.pubkey)){
                    Ok(d) => { match str::from_utf8(&d.unwrap()[..]) {
                        Ok(v) => crate::blockchain::account::Account::deserialize(v),
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    } },
                    Err(e) => { panic!("failed to read to database: {:?}", e) }
                };
                sender.balance -= tx.value;
                sender.nonce += 1;
                match self.acc.put(&write_ops, &Wallet::pk_to_pkh(&tx.pubkey), &sender.serialize().as_bytes()[..]){
                    Ok(_) => { () },
                    Err(e) => { panic!("failed to write to database: {:?}", e) }
                };
            }
        }
    }

    pub fn get_accounts() -> Self{
        let path = Path::new("./tmp/accounts");
        let mut options = Options::new();
        options.create_if_missing = true;

        let database = match Database::open(&path, &options){
            Ok(db) => { db },
            Err(e) => { panic!("failed to open database: {:?}", e) }
        };
        AccDB{
            acc: database,
        }
    }

    pub fn init_accounts(address: &Vec<u8>) -> Self{
        let write_ops = WriteOptions::new();
        let path = Path::new("./tmp/accounts");
        let mut options = Options::new();
        options.create_if_missing = true;

        let database = match Database::open(&path, &options){
            Ok(db) => { db },
            Err(e) => { panic!("failed to open database: {:?}", e) }
        };
        let acc = crate::blockchain::account::Account{
            balance: 50,
            nonce: 0,
        };
        match database.put(&write_ops, &Wallet::address_to_pkh(address), &acc.serialize().as_bytes()[..]){
            Ok(_) => { () },
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };
        AccDB{
            acc: database
        }
    }
}


