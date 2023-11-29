use hex;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
    parent: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new(hash: Vec<u8>) -> Self {
        MerkleNode {
            hash,
            left: None,
            right: None,
            parent: None,
        }
    }
}

struct MerkleTree {
    root: Option<MerkleNode>,
}

impl MerkleTree {
    fn new() -> Self {
        MerkleTree { root: None }
    }

    // Create/Construct the tree
    fn create_tree(&mut self, data: Vec<u8>) {
        let mut nodes = Vec::new();
        // criando os primeiros hashs
        for number in &data {
            let mut hasher = Sha256::new();
            hasher.update(&[*number]);
            let hashed: Vec<u8> = hasher.finalize().to_vec();
            let merkle_node = MerkleNode::new(hashed);
            nodes.push(merkle_node);
        }

        let hash_tree_data: Vec<MerkleNode> = self.create_hash_tree(nodes);
        if let Some(first_node) = hash_tree_data.into_iter().next() {
            self.root = Some(first_node);
        }
    }

    fn create_hash_tree(&mut self, nodes: Vec<MerkleNode>) -> Vec<MerkleNode> {
        if nodes.len() <= 1 {
            return nodes;
        }

        let mut temp_nodes: Vec<MerkleNode> = Vec::new();

        for chunk in nodes.chunks(2) {
            match chunk {
                [left, right] => {
                    let mut hasher = Sha256::new();
                    hasher.update(&left.hash);
                    hasher.update(&right.hash);
                    let hash: Vec<u8> = hasher.finalize().to_vec();

                    let merkle_node: MerkleNode = MerkleNode {
                        hash: (hash),
                        left: Some(Box::new(left.clone())),
                        right: Some(Box::new(right.clone())),
                        parent: None,
                    };

                    temp_nodes.push(merkle_node);
                    println!(
                        "left hash {} right hash {}",
                        hex::encode(&left.hash),
                        hex::encode(&right.hash)
                    );
                }
                [left] => {
                    let mut hasher = Sha256::new();
                    hasher.update(&left.hash);
                    hasher.update(&left.hash);
                    let hash: Vec<u8> = hasher.finalize().to_vec();

                    let merkle_node: MerkleNode = MerkleNode {
                        hash: (hash),
                        left: Some(Box::new(left.clone())),
                        right: Some(Box::new(left.clone())),
                        parent: None,
                    };
                    temp_nodes.push(merkle_node);
                    println!("left hash {} right hash None", hex::encode(&left.hash));
                }
                _ => {}
            }
        }
        return self.create_hash_tree(temp_nodes);
    }
}

pub fn run_demo() {
    let mut merkle_tree = MerkleTree::new();
    merkle_tree.create_tree([1, 23, 3, 4, 23, 1, 54].to_vec()); // original
}
