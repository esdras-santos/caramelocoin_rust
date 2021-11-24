use sha2::{Sha256, Digest};
use serde_derive::{Deserialize, Serialize};
use ring::{
    rand,
    signature::{self, KeyPair},
};
use crate::wallet::wallet::Wallet;
use crate::blockchain::blockchain::Blockchain;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Transaction{
    pub sig: Option<Vec<u8>>,
    nonce: u64,
    pub pubkey: Vec<u8>,
    pub recipient: Vec<u8>,
    pub value: u64
}
    
impl Transaction{
    pub fn new_transaction(from: &Wallet, to: &Vec<u8>, amount: u64, chain: &Blockchain) -> Self{
        let (balance, nonce) = chain.acc.balance_and_nonce(&from.address());
        if balance < amount+1 {
            panic!("not enough balance!")
        }
        let tx = Transaction{
            sig: None,
            nonce: nonce+1,
            pubkey: from.key_pair.public_key().as_ref().to_vec(),
            recipient: Wallet::address_to_pkh(to),
            //amount plus fee
            value: amount+1
        };
        tx.sign(from, chain);
        tx
    }
    pub fn id(&self) -> Vec<u8>{
        let mut data: Vec<u8> = Vec::new();
        data.append(&mut Self::to_bytes(&self.nonce.clone()));
        data.append(&mut self.pubkey.clone());
        data.append(&mut self.recipient.clone());
        data.append(&mut Self::to_bytes(&self.value.clone()));

        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let result: Vec<u8> = Vec::from(&hash[..]);
        result
    }

    //FIX THIS FUNCTON FIX THIS FUNCTON FIX THIS FUNCTON
    pub fn coinbase_tx(w: &Wallet) -> Self{
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
    
    pub fn to_bytes(input: &u64) -> Vec<u8>{
        let mut bytes:Vec<u8> = Vec::new();
        bytes.extend(&input.to_be_bytes());
        bytes
    }

    pub fn sign(&self, w: &Wallet, chain: &Blockchain) {
        if self.is_coinbase() {
            return
        }

        let (balance, _) = chain.acc.balance_and_nonce(&w.address());
        if balance >= self.value {
            let sig = w.key_pair.sign(&self.id());
            self.sig = Some(Vec::from(sig.as_ref()));
        } else {
            panic!("not enough founds!")
        }
    }

    pub fn verify_signature(txid: Vec<u8>, pubkey: Vec<u8>, sig: Vec<u8>) -> bool{
        let peer_public_key = 
        signature::UnparsedPublicKey::new(&signature::ED25519, pubkey);
        let result = match peer_public_key.verify(&txid, &sig){
            Ok(_) => true,
            Err(e) => false
        };
        result
    }
    
}

    
    

