use sha2::{Sha256, Digest};
// use serde::Deserialize;
// use serde_json::json;
use serde_derive::{Deserialize, Serialize};
// use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct Transaction{
    sig: Vec<u8>,
    nonce: u64,
    pubkey: Vec<u8>,
    recipient: Vec<u8>,
    value: u64
}
    
impl Transaction{
    pub fn new(s: Vec<u8>, n: u64, pk: Vec<u8>, r: Vec<u8>, v: u64) -> Self{
        Transaction{
            //the signature need to be genereted by a the function sign
            sig: s,
            //the nonce needs to be taken from the balance database
            nonce: n,
            pubkey: pk,
            recipient: r,
            //the value needs to be checked in the balance database
            value: v
        }
    }
    pub fn id(&self) -> Vec<u8>{
        let mut data: Vec<u8> = Vec::new();
        data.append(&mut to_bytes(&self.nonce.clone()));
        data.append(&mut self.pubkey.clone());
        data.append(&mut self.recipient.clone());
        data.append(&mut to_bytes(&self.value.clone()));

        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let result: Vec<u8> = Vec::from(&hash[..]);
        result
    }

    pub fn coinbase_tx() -> Self{
        Transaction{
            nonce: 0,
            pubkey: vec![0x0000000000000000000000000000000000000001],
            sig: vec![0x00000001],
            //the address of the recipient needs to be taken from a wallet
            recipient: vec![0x01],
            value: 50
        }
    }

    pub fn is_coinbase(&self) -> bool{
        if self.nonce == 0{
            if self.pubkey == vec![0x0000000000000000000000000000000000000001]{
                if self.sig == vec![0x00000001]{
                    return true;
                }
            }
        }
        false
    }

    pub fn serialize(&self) -> String{
        let v = serde_json::to_string(&self).unwrap();
        return v;
    }

    pub fn deserialize(s: &str) -> Transaction{
        let tx: Transaction = serde_json::from_str(s).unwrap();
        tx
    }    
}
pub fn to_bytes(input: &u64) -> Vec<u8>{
    let mut bytes:Vec<u8> = Vec::new();
    bytes.extend(&input.to_be_bytes());
    bytes
}
    
    

