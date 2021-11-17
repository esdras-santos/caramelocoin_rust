use sha2::{Sha256, Digest};
use crate::blockchain::block;
use byteorder::{BigEndian, ReadBytesExt};

const ONE_DAY: u32 = 86400;

struct ProofOfWork{
    block: block::Block,
}

impl ProofOfWork{
    pub fn new_bits(&self, prev_bits: &Vec<u8>, time_differential:&mut u64) -> Vec<u8>{
        if time_differential > (86400 * 4){
            time_differential = (86400 * 4);
        }
        if time_differential < (86400 / 4){
            time_differential = (86400 / 4);
        }

        let new_target = Self::bits_to_target(prev_bits) * (time_differential / 86400);
        Self::target_to_bits(&new_target)
    }

    //this function will be implemented at the end of the blockchain module
    pub fn get_bits(height: &u64, last_hash: &Vec<u8>) -> Vec<u8>{}

    pub fn target_to_bits(target: &usize) -> Vec<u8>{
        let exponent: usize;
        let mut coefficient: Vec<u8> = Vec::new();
        let raw_bytes: Vec<u8> = crate::blockchain::transaction::to_bytes(&target);
        if raw_bytes[0] > 0x7f{
            exponent = &raw_bytes.len() + 1;
            coefficient = vec![0x00];
            coefficient.append(&mut Vec::from(&raw_bytes[..2]));
        } else {
            exponent = raw_bytes.len();
            coefficient = Vec::from(&raw_bytes[..3]);
        }

        coefficient.append(&mut Vec::from(exponent.to_be_bytes()));
        coefficient

    }

    pub fn init_data(&self, nonce: &u128) -> Vec<u8>{
        self.block.nonce = crate::blockchain::transaction::to_bytes(nonce);
        let data = self.block.serialize();
        Vec::from(data.as_bytes())
    }

    pub fn run(&self) -> u128{
        let mut hash: Vec<u8> = Vec::new();
        let mut hasher = Sha256::new();
        let mut nonce: u128 = 0;
        let target = self.block.target();

        while nonce < u128::MAX{
            let data = self.init_data(&nonce);
            hasher.update(data);
            let h = hasher.finalize();
            hash = Vec::from(&h[..]);

        }

        return nonce
    }

    pub fn bits_to_target(bits: &Vec<u8>) -> usize{
        let exponent: usize = usize::from_be_bytes(bits[3]); 

        let coefficient: usize = usize::from_be_bytes(bits[..3]);
        let base: usize = 256;
        let n: usize = 3;
        return coefficient * base.pow(exponent - n); 
    }
    
}
