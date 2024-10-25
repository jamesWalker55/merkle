use sha2::Digest;

pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

/// Which side to put Hash on when concatinating proof hashes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HashDirection {
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct Proof<'a> {
    /// The hashes to use when verifying the proof
    /// The first element of the tuple is which side the hash should be on when concatinating
    pub hashes: Vec<(HashDirection, &'a Hash)>,
}

pub trait Tree {
    /// Gets root hash for this tree
    fn root(&self) -> Hash;

    /// Constructs a Merkle tree from given input data
    fn construct(input: &[Data]) -> Self;

    /// Verifies that the given input data produces the given root hash
    fn verify(input: &[Data], root_hash: &Hash) -> bool;

    /// Verifies that the given data and proof_path correctly produce the given root_hash
    fn verify_proof(data: &Data, proof: &Proof, root_hash: &Hash) -> bool;

    /// Returns a list of hashes that can be used to prove that the given data is in this tree
    fn prove(&self, data: &Data) -> Option<Proof>;
}

pub(crate) fn hash_data(data: &Data) -> Hash {
    sha2::Sha256::digest(data).to_vec()
}

pub(crate) fn hash_concat(h1: &Hash, h2: &Hash) -> Hash {
    let h3 = h1.iter().chain(h2).copied().collect();
    hash_data(&h3)
}
