
mod blockchain;
use crate::blockchain::transaction;

fn main() {

    // let nonce = transaction::to_bytes(&10);
    // let value = transaction::to_bytes(&50);
    let tx = transaction::Transaction::new(vec![0x10],10,vec![0x10],vec![0x20],50);
    let txs = tx.serialize();
    println!("{}", txs);
    let txd = transaction::Transaction::deserialize(&txs);
    let txs2 = txd.serialize();
    println!("{}",txs2)
}





