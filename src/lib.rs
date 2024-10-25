mod merkle;
mod tree;

#[cfg(test)]
mod tests {
    use merkle::MerkleTree;
    use tree::{Data, Tree as _};

    use super::*;

    fn example_data(n: usize) -> Vec<Data> {
        let mut data = vec![];
        for i in 0..n {
            data.push(vec![i as u8]);
        }
        data
    }

    #[test]
    fn test_constructions() {
        let data = example_data(4);
        let tree = MerkleTree::construct(&data);
        let expected_root = "9675e04b4ba9dc81b06e81731e2d21caa2c95557a85dcfa3fff70c9ff0f30b2e";
        assert_eq!(hex::encode(tree.root()), expected_root);

        // Uncomment if your implementation allows for unbalanced trees
        // let data = example_data(3);
        // let tree = MerkleTree::construct(&data);
        // let expected_root = "773a93ac37ea78b3f14ac31872c83886b0a0f1fec562c4e848e023c889c2ce9f";
        // assert_eq!(hex::encode(tree.root()), expected_root);

        let data = example_data(8);
        let tree = MerkleTree::construct(&data);
        let expected_root = "0727b310f87099c1ba2ec0ba408def82c308237c8577f0bdfd2643e9cc6b7578";
        assert_eq!(hex::encode(tree.root()), expected_root);
    }

    #[test]
    fn test_verify() {
        let data = example_data(4);
        let expected_root = "9675e04b4ba9dc81b06e81731e2d21caa2c95557a85dcfa3fff70c9ff0f30b2e";
        assert!(MerkleTree::verify(
            &data,
            &hex::decode(expected_root).unwrap()
        ));

        // Uncomment if your implementation allows for unbalanced trees
        // let data = example_data(3);
        // let expected_root = "773a93ac37ea78b3f14ac31872c83886b0a0f1fec562c4e848e023c889c2ce9f";
        // assert!(MerkleTree::verify(
        //     &data,
        //     &hex::decode(expected_root).unwrap()
        // ));

        let data = example_data(8);
        let expected_root = "0727b310f87099c1ba2ec0ba408def82c308237c8577f0bdfd2643e9cc6b7578";
        assert!(MerkleTree::verify(
            &data,
            &hex::decode(expected_root).unwrap()
        ));
    }
}
