use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use ring::{
    rand,
    signature::{self, KeyPair},
};

const checksum_legth: usize = 4;
const version: Vec<u8> = vec![0x00];
const wallet_path: String = "./tmp/wallet.data".to_string();

pub struct Wallet{
    pub key_pair: signature::Ed25519KeyPair
}
impl Wallet{

    pub fn address(&self) -> Vec<u8>{
        let pub_hash = self.publickey_hash();
        let mut versioned_hash = version;
        versioned_hash.append(&mut pub_hash);
        let checksum = Self::checksum(versioned_hash);
        versioned_hash.append(&mut checksum);
        return versioned_hash;
    }

    pub fn address_to_pkh(address: &Vec<u8>) -> Vec<u8>{
        return Vec::from(&address[1..address.len()-checksum_legth]);
    } 

    pub fn pk_to_pkh(pubkey: &Vec<u8>) -> Vec<u8>{
        let mut hasher = Sha256::new();
        hasher.update(pubkey);
        let hash = hasher.finalize();
        let pub_hash: Vec<u8> = Vec::from(&hash[..]);

        let hasher160 = Ripemd160::new();
        hasher160.update(pub_hash);
        let hash160 = hasher160.finalize();
        Vec::from(&hash160[..])
    }

    pub fn pkh_to_address(pkh: Vec<u8>) -> Vec<u8>{
        let mut versioned_hash = version;
        versioned_hash.append(&mut pkh);
        let checksum = Self::checksum(versioned_hash);
        versioned_hash.append(&mut checksum);
        return versioned_hash;
    }

    pub fn validate_address(address: &Vec<u8>) -> bool{
        let checksum = Vec::from(&address[address.len()-checksum_legth..][..]);
        let version = vec![address[0]];
        let pubkey_hash = Vec::from(&address[1..address.len()-checksum_legth][..]);
        version.append(&mut pubkey_hash);
        let target_checksum = Self::checksum(version);
        return checksum == target_checksum;
    }

    fn new_key_pair() -> signature::Ed25519KeyPair{
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = match signature::Ed25519KeyPair::generate_pkcs8(&rng){
            Ok(d) => {d},
            Err(e) => {panic!("error generating keypair document {:?}", e)}
        };
        Self::save_file(pkcs8_bytes);

        let key_pair = match signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()){
            Ok(kp) => {kp},
            Err(e) => {panic!("error generating keypair {:?}", e)}
        };
        key_pair
    }

    pub fn make_wallet() -> Self{
        
        let kp = Self::new_key_pair();
        Wallet{
            key_pair: kp
        }
    }

    pub fn publickey_hash(&self) -> Vec<u8>{
        let mut hasher = Sha256::new();
        hasher.update(self.key_pair.public_key().as_ref());
        let hash = hasher.finalize();
        let pub_hash: Vec<u8> = Vec::from(&hash[..]);

        let hasher160 = Ripemd160::new();
        hasher160.update(pub_hash);
        let hash160 = hasher160.finalize();
        Vec::from(&hash160[..])
    }

    pub fn checksum(payload: Vec<u8>) -> Vec<u8>{
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let mut hash = hasher.finalize();
        let firsthash: Vec<u8> = Vec::from(&hash[..]);
        hasher.update(firsthash);
        hash = hasher.finalize();
        let secondhash: Vec<u8> = Vec::from(&hash[..]);
        Vec::from(&secondhash[..checksum_legth])
    }

    pub fn load_file() -> Self {
        if !Path::new(&wallet_path).exists() {
            panic!("wallet does not exists")
        }
        let mut con = String::new();
        let mut contents = match File::open(&wallet_path){
            Ok(f) => {
                match f.read_to_string(&mut con){
                    Ok(u) => {con},
                    Err(e) => {panic!("error reading file {:?}", e)}
                };
            },
            Err(e) => {panic!("error opening file {:?}", e)}
        }; 
        let key_pair = match signature::Ed25519KeyPair::from_pkcs8(contents.as_bytes()){
            Ok(kp) => {kp},
            Err(e) => {panic!("error generating keypair {:?}", e)}
        };
        Wallet{
            key_pair: key_pair,
        }
    }

    fn save_file(w: ring::pkcs8::Document) {
        let mut file = File::create(&wallet_path);
        match file.unwrap().write_all(w.as_ref()){
            _ => {},
            Err(e) => {panic!("error saving wallet file {:?}",e)}
        };
    }
}