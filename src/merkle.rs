use crate::tree::{Data, Hash, Proof, Tree};

pub struct MerkleTree {
    // ???
}

impl Tree for MerkleTree {
    fn root(&self) -> Hash {
        todo!("For tests to work")
    }

    fn construct(input: &[Data]) -> Self {
        todo!("Exercise 1")
    }

    fn verify(input: &[Data], root_hash: &Hash) -> bool {
        todo!("Exercise 1b")
    }

    fn verify_proof(data: &Data, proof: &Proof, root_hash: &Hash) -> bool {
        todo!("Exercise 2")
    }

    fn prove(&self, data: &Data) -> Option<Proof> {
        todo!("Exercise 3")
    }
}
