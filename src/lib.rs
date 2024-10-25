mod merkle;
mod tree;

#[cfg(test)]
mod tests {
    use merkle::MerkleTree;
    use tree::{hash_concat, hash_data, Data, HashDirection, Proof, Tree};

    use super::*;

    // root hash for 4 leaves
    const ROOT_HASH_4: &str = "9675e04b4ba9dc81b06e81731e2d21caa2c95557a85dcfa3fff70c9ff0f30b2e";
    // root hash for 8 leaves
    const ROOT_HASH_8: &str = "0727b310f87099c1ba2ec0ba408def82c308237c8577f0bdfd2643e9cc6b7578";

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
        let expected_root = ROOT_HASH_4;
        assert_eq!(hex::encode(tree.root()), expected_root);

        // Uncomment if your implementation allows for unbalanced trees
        // let data = example_data(3);
        // let tree = MerkleTree::construct(&data);
        // let expected_root = "773a93ac37ea78b3f14ac31872c83886b0a0f1fec562c4e848e023c889c2ce9f";
        // assert_eq!(hex::encode(tree.root()), expected_root);

        let data = example_data(8);
        let tree = MerkleTree::construct(&data);
        let expected_root = ROOT_HASH_8;
        assert_eq!(hex::encode(tree.root()), expected_root);
    }

    #[test]
    fn test_verify() {
        let data = example_data(4);
        let expected_root = ROOT_HASH_4;
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
        let expected_root = ROOT_HASH_8;
        assert!(MerkleTree::verify(
            &data,
            &hex::decode(expected_root).unwrap()
        ));
    }

    #[test]
    // tree with 4 leaf nodes
    fn test_exercise_2_fourleaf() {
        let a1 = vec![0u8];
        let a2 = vec![1u8];
        let a3 = vec![2u8];
        let a4 = vec![3u8];
        let h1 = hash_data(&a1);
        let h2 = hash_data(&a2);
        let h3 = hash_data(&a3);
        let h4 = hash_data(&a4);
        let h5 = hash_concat(&h1, &h2);
        let h6 = hash_concat(&h3, &h4);
        let h7 = hash_concat(&h5, &h6);

        let root_hash = {
            // sanity check
            let hash_str = ROOT_HASH_4;
            assert_eq!(h7, hex::decode(hash_str).unwrap());
            &h7
        };

        // H1 ; [(1, H2), (1, H6)]
        let data = &a1;
        let proof = vec![(HashDirection::Right, &h2), (HashDirection::Right, &h6)];
        assert!(MerkleTree::verify_proof(
            &data,
            &Proof { hashes: proof },
            &root_hash
        ));

        // H2 ; [(0, H1), (1, H6)]
        let data = &a2;
        let proof = vec![(HashDirection::Left, &h1), (HashDirection::Right, &h6)];
        assert!(MerkleTree::verify_proof(
            &data,
            &Proof { hashes: proof },
            &root_hash
        ));

        // H3 ; [(1, H4), (0, H5)]
        let data = &a3;
        let proof = vec![(HashDirection::Right, &h4), (HashDirection::Left, &h5)];
        assert!(MerkleTree::verify_proof(
            &data,
            &Proof { hashes: proof },
            &root_hash
        ));

        // H4 ; [(0, H3), (0, H5)]
        let data = &a4;
        let proof = vec![(HashDirection::Left, &h3), (HashDirection::Left, &h5)];
        assert!(MerkleTree::verify_proof(
            &data,
            &Proof { hashes: proof },
            &root_hash
        ));
    }

    #[test]
    // tree with 8 leaf nodes
    fn test_exercise_2_eightleaf() {
        let a1 = vec![0u8];
        let a2 = vec![1u8];
        let a3 = vec![2u8];
        let a4 = vec![3u8];
        let a5 = vec![4u8];
        let a6 = vec![5u8];
        let a7 = vec![6u8];
        let a8 = vec![7u8];
        let h1 = hash_data(&a1);
        let h2 = hash_data(&a2);
        let h3 = hash_data(&a3);
        let h4 = hash_data(&a4);
        let h5 = hash_data(&a5);
        let h6 = hash_data(&a6);
        let h7 = hash_data(&a7);
        let h8 = hash_data(&a8);
        let h9 = hash_concat(&h1, &h2);
        let h10 = hash_concat(&h3, &h4);
        let h11 = hash_concat(&h5, &h6);
        let h12 = hash_concat(&h7, &h8);
        let h13 = hash_concat(&h9, &h10);
        let h14 = hash_concat(&h11, &h12);
        let h15 = hash_concat(&h13, &h14);

        let root_hash = {
            // sanity check
            let hash_str = ROOT_HASH_8;
            assert_eq!(h15, hex::decode(hash_str).unwrap());
            &h15
        };

        // H1 ; [(1, H2), (1, H10), (1, H14)]
        let data = &a1;
        let proof = vec![
            (HashDirection::Right, &h2),
            (HashDirection::Right, &h10),
            (HashDirection::Right, &h14),
        ];
        assert!(MerkleTree::verify_proof(
            &data,
            &Proof { hashes: proof },
            &root_hash
        ));

        // H4 ; [(0, H3), (0, H9), (1, H14)]
        let data = &a4;
        let proof = vec![
            (HashDirection::Left, &h3),
            (HashDirection::Left, &h9),
            (HashDirection::Right, &h14),
        ];
        assert!(MerkleTree::verify_proof(
            &data,
            &Proof { hashes: proof },
            &root_hash
        ));

        // H7 ; [(1, H8), (0, H11), (0, H13)]
        let data = &a7;
        let proof = vec![
            (HashDirection::Right, &h8),
            (HashDirection::Left, &h11),
            (HashDirection::Left, &h13),
        ];
        assert!(MerkleTree::verify_proof(
            &data,
            &Proof { hashes: proof },
            &root_hash
        ));
    }

    #[test]
    fn test_exercise_3_fourleaf() {
        let a1 = vec![0u8];
        let a2 = vec![1u8];
        let a3 = vec![2u8];
        let a4 = vec![3u8];
        let h1 = hash_data(&a1);
        let h2 = hash_data(&a2);
        let h3 = hash_data(&a3);
        let h4 = hash_data(&a4);
        let h5 = hash_concat(&h1, &h2);
        let h6 = hash_concat(&h3, &h4);
        let h7 = hash_concat(&h5, &h6);
        let tree = MerkleTree::construct(&example_data(4));

        // H1 ; [(1, H2), (1, H6)]
        let rv = tree.prove(&a1);
        let expected = vec![(HashDirection::Right, &h2), (HashDirection::Right, &h6)];
        assert_eq!(rv, Some(Proof { hashes: expected }));

        // H2 ; [(0, H1), (1, H6)]
        let rv = tree.prove(&a2);
        let expected = vec![(HashDirection::Left, &h1), (HashDirection::Right, &h6)];
        assert_eq!(rv, Some(Proof { hashes: expected }));

        // H3 ; [(1, H4), (0, H5)]
        let rv = tree.prove(&a3);
        let expected = vec![(HashDirection::Right, &h4), (HashDirection::Left, &h5)];
        assert_eq!(rv, Some(Proof { hashes: expected }));

        // H4 ; [(0, H3), (0, H5)]
        let rv = tree.prove(&a4);
        let expected = vec![(HashDirection::Left, &h3), (HashDirection::Left, &h5)];
        assert_eq!(rv, Some(Proof { hashes: expected }));
    }

    #[test]
    fn test_exercise_3_eightleaf() {
        let a1 = vec![0u8];
        let a2 = vec![1u8];
        let a3 = vec![2u8];
        let a4 = vec![3u8];
        let a5 = vec![4u8];
        let a6 = vec![5u8];
        let a7 = vec![6u8];
        let a8 = vec![7u8];
        let h1 = hash_data(&a1);
        let h2 = hash_data(&a2);
        let h3 = hash_data(&a3);
        let h4 = hash_data(&a4);
        let h5 = hash_data(&a5);
        let h6 = hash_data(&a6);
        let h7 = hash_data(&a7);
        let h8 = hash_data(&a8);
        let h9 = hash_concat(&h1, &h2);
        let h10 = hash_concat(&h3, &h4);
        let h11 = hash_concat(&h5, &h6);
        let h12 = hash_concat(&h7, &h8);
        let h13 = hash_concat(&h9, &h10);
        let h14 = hash_concat(&h11, &h12);
        let h15 = hash_concat(&h13, &h14);
        let tree = MerkleTree::construct(&example_data(8));

        // H1 ; [(1, H2), (1, H10), (1, H14)]
        let rv = tree.prove(&a1);
        let expected = vec![
            (HashDirection::Right, &h2),
            (HashDirection::Right, &h10),
            (HashDirection::Right, &h14),
        ];
        assert_eq!(rv, Some(Proof { hashes: expected }));

        // H4 ; [(0, H3), (0, H9), (1, H14)]
        let rv = tree.prove(&a4);
        let expected = vec![
            (HashDirection::Left, &h3),
            (HashDirection::Left, &h9),
            (HashDirection::Right, &h14),
        ];
        assert_eq!(rv, Some(Proof { hashes: expected }));

        // H7 ; [(1, H8), (0, H11), (0, H13)]
        let rv = tree.prove(&a7);
        let expected = vec![
            (HashDirection::Right, &h8),
            (HashDirection::Left, &h11),
            (HashDirection::Left, &h13),
        ];
        assert_eq!(rv, Some(Proof { hashes: expected }));
    }
}
