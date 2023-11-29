use sha2::{Digest, Sha256};
pub struct MerkleTree {
    pub nodes: Vec<Vec<u8>>, // Armazena os hashes de todos os nós
}

impl MerkleTree {
    pub fn new(data: Vec<u8>) -> Self {
        let mut tree = MerkleTree { nodes: Vec::new() };
        tree.create_tree(data);
        tree
    }

    fn create_tree(&mut self, data: Vec<u8>) {
        let leaf_count = data.len().next_power_of_two();
        let total_nodes = 2 * leaf_count - 1;
        let mut nodes = vec![vec![0; 32]; total_nodes]; // Inicializa todos os nós com hashes vazios

        // println!("max capacity {}", 2 * leaf_count - 1);

        // Preenche os nós folha
        for (i, &datum) in data.iter().enumerate() {
            let mut hasher = Sha256::new();
            hasher.update(&[datum]);
            nodes[leaf_count - 1 + i] = hasher.finalize().to_vec();
        }

        // Constrói os nós internos
        for i in (0..leaf_count - 2).rev() {
            let left_hash = nodes[i * 2 + 1].clone();
            let right_hash = nodes[i * 2 + 2].clone();
            // println!(
            //     "index i {} hash left {} right {}",
            //     i,
            //     hex::encode(&left_hash),
            //     hex::encode(&right_hash)
            // );

            let mut hasher = Sha256::new();
            hasher.update(&left_hash);
            hasher.update(&right_hash);
            nodes[i] = hasher.finalize().to_vec();
        }

        // for node in &nodes {
        //     println!("conferindo arvore {}", hex::encode(node));
        // }

        self.nodes = nodes;
    }

    pub fn generate_proof(&self, data_index: u8) -> Vec<(Vec<u8>, bool)> {
        let mut proof = Vec::new();
        let leaf_count = (self.nodes.len() / 2).next_power_of_two();

        let mut node_index = leaf_count + data_index as usize - 1; // Índice do nó folha no vetor 'nodes'
        // println!("node index {}", node_index);

        // getting brother
        let sibling_index = if node_index % 2 == 0 {
            node_index - 1
        } else {
            node_index + 1
        };
        // println!("brother index {} node index {}", sibling_index, node_index);
        if sibling_index < self.nodes.len() {
            proof.push((self.nodes[sibling_index].clone(), node_index % 2 == 0));
        }

        while node_index >= 1 {
            let parent_index = ((node_index + 1) / 2) - 1;
            if parent_index < self.nodes.len() {
                let missing_hash = if parent_index % 2 == 0 {
                    parent_index - 1
                } else {
                    parent_index + 1
                };
                // println!(
                //     "missing hash {} hash: {}",
                //     missing_hash,
                //     hex::encode(&self.nodes[missing_hash])
                // );
                proof.push((self.nodes[missing_hash].clone(), parent_index % 2 == 0));
            }
            node_index = parent_index;
            if node_index == 2 || node_index == 1 {
                break;
            };
        }

        proof
    }

    pub fn verify_proof(
        &self,
        proof: Vec<(Vec<u8>, bool)>,
        data: u8,
        root_hash: Vec<u8>,
    ) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(&[data]);
        let mut current_hash = hasher.finalize().to_vec();

        for (hash, position) in proof {
            // println!("proff elemenet {}", hex::encode(&hash));
            let mut hasher = Sha256::new();
            if position {
                hasher.update(&hash);
                hasher.update(&current_hash);
            } else {
                hasher.update(&current_hash);
                hasher.update(&hash);
            }

            current_hash = hasher.finalize().to_vec();
        }

        // println!(
        //     "root hash: {} current hash {}",
        //     hex::encode(&root_hash),
        //     hex::encode(&current_hash)
        // );
        current_hash == root_hash
    }
}

pub fn run_demo() {
    let data = vec![1, 2, 3, 4, 5, 6];
    let merkle_tree = MerkleTree::new(data);

    let data_index: u8 = 1; // Index do dado para o qual queremos gerar a prova (ex: 23 no índice 1)
    let data_value = 2; // O valor do dado em si

    // // Gera a prova para o dado
    let proof: Vec<(Vec<u8>, bool)> = merkle_tree.generate_proof(data_index);

    // // Obtém o hash da raiz da árvore
    let root_hash = merkle_tree.nodes[0].clone();

    // // Verifica a prova
    let is_valid = merkle_tree.verify_proof(proof, data_value, root_hash);

    println!("A prova é válida? {}", is_valid);
}
