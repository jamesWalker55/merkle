use std::collections::VecDeque;

use crate::tree::{hash_concat, hash_data, Data, Hash, HashDirection, Proof, Tree};

#[derive(Debug)]
enum MerkleNext {
    Data(Data),
    Nodes(Box<MerkleTree>, Box<MerkleTree>),
}

#[derive(Debug)]
pub struct MerkleTree {
    hash: Hash,
    next: MerkleNext,
}

impl MerkleTree {
    fn new_from_data(data: Data) -> Self {
        Self {
            hash: hash_data(&data),
            next: MerkleNext::Data(data),
        }
    }

    fn new_from_nodes(a: Box<MerkleTree>, b: Box<MerkleTree>) -> Self {
        Self {
            hash: hash_concat(&a.hash, &b.hash),
            next: MerkleNext::Nodes(a, b),
        }
    }

    fn prove(&self, data: &Data) -> Option<Proof> {
        let mut buf = Vec::new();
        if !self.prove_sub(&mut buf, data) {
            return None;
        }
        Some(Proof { hashes: buf })
    }

    /// Given a vector and a data, check if the children contain this data.
    ///
    /// If the data is found, append the other child's hash (e.g. if found in left node, append
    /// right node's hash), and return true.
    ///
    /// Otherwise, return false.
    fn prove_sub<'a>(&'a self, buf: &mut Vec<(HashDirection, &'a Hash)>, data: &Data) -> bool {
        match &self.next {
            MerkleNext::Data(vec) => vec == data,
            MerkleNext::Nodes(left, right) => {
                if left.prove_sub(buf, data) {
                    buf.push((HashDirection::Right, &right.hash));
                    true
                } else if right.prove_sub(buf, data) {
                    buf.push((HashDirection::Left, &left.hash));
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Tree for MerkleTree {
    fn root(&self) -> Hash {
        self.hash.clone()
    }

    fn construct(input: &[Data]) -> Self {
        // ensure input data count is a power of 2
        {
            let log2 = (input.len() as f32).log2();
            assert!(
                log2.fract() == 0.0,
                "number of input data must be a power of 2, got {}",
                input.len(),
            );
        }

        // create leaf nodes
        let mut nodes: VecDeque<_> = input
            .iter()
            .map(|data| MerkleTree::new_from_data(data.clone()))
            .collect();

        // pair every 2 nodes until 1 node remaining
        while nodes.len() != 1 {
            for _ in (0..nodes.len()).step_by(2) {
                let left = nodes.pop_front().unwrap();
                let right = nodes.pop_front().unwrap();
                let new_node = MerkleTree::new_from_nodes(Box::new(left), Box::new(right));
                nodes.push_back(new_node);
            }
        }

        nodes.pop_back().unwrap()
    }

    fn verify(input: &[Data], root_hash: &Hash) -> bool {
        let tree = Self::construct(input);
        &tree.root() == root_hash
    }

    fn verify_proof(data: &Data, proof: &Proof, root_hash: &Hash) -> bool {
        let mut hash = hash_data(data);

        for (hash_dir, other_hash) in proof.hashes.iter() {
            match hash_dir {
                HashDirection::Left => {
                    hash = hash_concat(other_hash, &hash);
                }
                HashDirection::Right => {
                    hash = hash_concat(&hash, other_hash);
                }
            }
        }

        &hash == root_hash
    }

    fn prove(&self, data: &Data) -> Option<Proof> {
        self.prove(data)
    }
}
