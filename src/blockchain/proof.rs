use sha2::{Sha256, Digest};
use crate::blockchain::block;
use num_bigint::BigUint;

const ONE_DAY: u32 = 86400;

struct ProofOfWork{
    block: block::Block,
}

// impl ProofOfWork{
//     pub fn new_bits(&self, prev_bits: Vec<u8>, time_differential:&mut u64) -> Vec<u8>{
//         if time_differential > (ONE_DAY * 4){
//             time_differential = (ONE_DAY * 4);
//         }
//         if time_differential < (ONE_DAY / 4){
//             time_differential = (ONE_DAY / 4);
//         }



//     }

//     pub fn target_to_bits(target: BigUint) -> Vec<u8>{
        
//     }

//     pub fn bits_to_target(bits: Vec<u8>) -> BigUint{
//         let exponent: BigUint = 0;
//         let coefficient: BigUint = 0;
//     }
    
// }
