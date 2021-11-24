use sha2::{Sha256, Digest};
use num_bigint::{BigUint,ToBigUint};
use byteorder::{BigEndian, ReadBytesExt};
use std::convert::AsMut;
use num::pow::pow;
use crate::blockchain::block;
use crate::blockchain::transaction;
use crate::blockchain::blockchain::Blockchain;

const ONE_DAY: u32 = 86400;
const BLOCKS_PER_DAY: u64 = 1440;

pub struct ProofOfWork{
    block: block::Block,
}

impl ProofOfWork{
    pub fn new_bits(&self, prev_bits: &Vec<u8>, time_differential:&u64) -> Vec<u8>{
        let mut td: u128;
        if time_differential > &(86400 * 4){
            td = 86400 * 4;
        }
        if time_differential < &(86400 / 4){
            td = 86400 / 4;
        }

        let new_target = Self::bits_to_target(prev_bits) * (td / 86400);
        Self::target_to_bits(&new_target)
    }

    //this function will be implemented at the end of the blockchain module
    pub fn get_bits(height: &u64, last_hash: &Vec<u8>, chain: &Blockchain) -> Vec<u8>{
        let pow: ProofOfWork;
        if height == &0{
            vec![0x01,0xff,0xff,0xff]
        } else if height % BLOCKS_PER_DAY == 0 {
            let last_block = chain.get_block(&chain.last_hash.unwrap());
            return pow.new_bits(&last_block.bits, &Self::get_time_difference(chain));
        } else {
            let last_block = chain.get_block(&chain.last_hash.unwrap());
            last_block.bits
        }
    }

    pub fn target_to_bits(target: &BigUint) -> Vec<u8>{
        let exponent: usize;
        let mut coefficient: Vec<u8> = Vec::new();
        let raw_bytes: Vec<u8> = target.to_bytes_be();
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

    pub fn init_data(&self, nonce: &u64) -> Vec<u8>{
        self.block.nonce = Some(crate::blockchain::transaction::Transaction::to_bytes(nonce));
        let data = self.block.serialize();
        Vec::from(data.as_bytes())
    }

    pub fn run(&self) -> u64{
        let mut hash: Vec<u8> = Vec::new();
        let mut hasher = Sha256::new();
        let mut nonce: u64 = 0;
        let target = self.block.target();
        let int_hash: BigUint;
        let mut hash_string = String::new();

        while nonce < u64::MAX{
            let data = self.init_data(&nonce);
            hasher.update(data);
            let h = hasher.finalize();
            hash = Vec::from(&h[..]);
            for byte in hash{
                write!(&mut hash_string, "{:X}", byte).expect("Unable to write");
            }
            print!("\r{}",hash_string);
            int_hash = BigUint::from_bytes_be(&hash);

            if int_hash < target{
                break
            } else{
                nonce+=1
            }
        }

        return nonce
    }

    pub fn bits_to_target(bits: &Vec<u8>) -> BigUint{
        let exponent: usize = usize::from(bits[3]); 

        let coefficient: usize = usize::from_be_bytes(Self::clone_into_array(&bits[..3]));
        let base: usize = 256;
        let n: usize = 3;
        return (coefficient * pow(base, exponent - n)).to_biguint().unwrap(); 
    }

    pub fn get_time_difference(chain: &Blockchain) -> u64{
        let iter = chain.iterator();
        let i: u64 = 0;
        while i < BLOCKS_PER_DAY{
            iter.next();
            i+=1;
        }
        let first_block = chain.get_block(&iter.current_hash);
        let last_block = chain.get_block(&iter.current_hash);
        u64::from_le_bytes(Self::clone_into_array(&last_block.timestamp[..])) - u64::from_le_bytes(Self::clone_into_array(&first_block.timestamp[..]))

    }

    pub fn validate(&self) -> bool{
        let int_hash: BigUint;
        let mut hasher = Sha256::new();

        let data = self.block.serialize();
        hasher.update(data);
        let h = hasher.finalize();
        let hash = Vec::from(&h[..]);
        
        return int_hash < self.block.target()
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
