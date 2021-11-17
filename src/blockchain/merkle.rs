use sha2::{Sha256, Digest};


pub struct MerkleTree{
    pub root_node: MerkleNode,
}

impl MerkleTree{
    pub fn new(data: &mut Vec<Vec<u8>>) -> Self{
        let mut nodes: Vec<MerkleNode> = Vec::new();
        if data.len() % 2 != 0{
            data.push(data[data.len() - 1]);
        }

        for d in data{
            let node = MerkleNode::new(None, None, Some(d.to_vec()));
            nodes.push(node);
        }
        let mut i: usize = 0;
        while i < data.len()/2{
            let level: Vec<MerkleNode>;
            let mut j: usize = 0;
            while j < nodes.len(){
                let node = MerkleNode::new(Some(nodes[j]), Some(nodes[j+1]), None);
                level.push(node);
                j += 2;
            }
            nodes = level;
            i += 1;
        }

        MerkleTree{
            root_node: nodes[0]
        }
    }
}

pub struct MerkleNode{
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
    pub data: Option<Vec<u8>>
}

impl MerkleNode{
    pub fn new(left: Option<Self>,right: Option<Self>, data: Option<Vec<u8>>) -> Self{
        let node: MerkleNode;
        if left.is_none() && right.is_none(){
            let mut hasher = Sha256::new();
            hasher.update(data.unwrap());
            let hash = hasher.finalize();
            node.data = Some(Vec::from(&hash[..]));
        } else {
            let prev_hashes: Vec<u8>;
            prev_hashes.append(&mut left.unwrap().data.unwrap());
            prev_hashes.append(&mut right.unwrap().data.unwrap());
            let mut hasher = Sha256::new();
            hasher.update(prev_hashes);
            let hash = hasher.finalize();
            node.data = Some(Vec::from(&hash[..]));
        }
        node.right = Some(Box::new(right.unwrap()));
        node.left = Some(Box::new(left.unwrap()));
        node
    }
}