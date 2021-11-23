
use sha2::{Sha256, Digest};

pub struct Wallet{
    private: Vec<u8>,
}
impl Wallet{

    pub fn address(&self) -> Vec<u8>{

    }

    pub fn address_to_pkh(address: &str) -> Vec<u8>{
        return [];
    } 

    pub fn pk_to_pkh(pubkey: Vec<u8>) -> Vec<u8>{

    }

    pub fn pkh_to_address(pkh: Vec<u8>) -> &str{
        return " ";
    }

    pub fn validate_address(address: String) -> bool{
        false
    }

    pub fn new_key_pair() -> Vec<u8>{

    }

    pub fn make_wallet() -> Self{
        let private = Self::new_key_pair();
        Wallet{
            private: private
        }
    }

    pub fn publickey_hash(&self) -> Vec<u8>{

    }

    pub fn checksum(payload: Vec<u8>) -> Vec<u8>{
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let mut hash = hasher.finalize();
        let firsthash: Vec<u8> = Vec::from(&hash[..]);
        hasher.update(firsthash);
        hash = hasher.finalize();
        let secondhash: Vec<u8> = Vec::from(&hash[..]);
        Vec::from(&secondhash[..4])
    }
}