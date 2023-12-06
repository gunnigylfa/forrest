use std::collections::HashMap;

use crate::binary_tree::BinaryTreeBehavior;
use hex::FromHex;
use hex::{self, FromHexError};
use sha3::{Digest, Sha3_256};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    depth: u32,
    representation: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Handedness {
    Left,
    Right,
}

// A Merkle tree is a special case of a complete binary tree. Therefore, it shares the BinaryTreeBehavior trait
impl BinaryTreeBehavior for MerkleTree {}

impl MerkleTree {
    fn hex_to_bytes(s: String) -> Result<Vec<u8>, FromHexError> {
        let without_prefix = if s.starts_with("0x") {
            String::from(&s[2..s.len()])
        } else {
            s
        };
        Vec::<u8>::from_hex(without_prefix)
    }

    fn concatenate_hashes(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
        let mut concatenation = left;
        let mut right_vec = right;
        concatenation.append(&mut right_vec);
        return concatenation;
    }

    fn hash(v: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(v.clone());
        let hashed: Vec<u8> = hasher.finalize().to_vec();
        return hashed;
    }

    // Exercise 3:
    /// Creates a merkle tree of depth and initializez its leaves to the initial leaf value
    ///
    /// # Arguments
    ///
    /// * `depth` - An integer indicating the depth of the tree
    /// * `initial_leaf` - A string representation of a hexadecimal hash to be used as an initialization value for all of the tree's leaf nodes
    ///
    pub fn new(depth: u32, initial_leaf: String) -> Self {
        // Handle edge cases
        if depth == 0 {
            return MerkleTree {
                depth: depth,
                representation: vec![
                    Vec::new(),
                    Self::hex_to_bytes(initial_leaf)
                        .expect("Initial leaf should be a hexadecimal string"),
                ],
            };
        }
        if depth == 1 {
            let left = Self::hex_to_bytes(initial_leaf.clone())
                .expect("Initial leaf should be a hexadecimal string");
            let right = Self::hex_to_bytes(initial_leaf)
                .expect("Initial leaf should be a hexadecimal string");
            return MerkleTree {
                depth: depth,
                representation: vec![
                    Vec::new(),
                    Self::hash(Self::concatenate_hashes(left.clone(), right.clone())),
                    left,
                    right,
                ],
            };
        }

        let base: u32 = 2;
        let mut mt: MerkleTree = MerkleTree {
            depth,
            representation: vec![Vec::new(); base.pow(depth) as usize],
        };

        let as_bytes = Self::hex_to_bytes(initial_leaf.clone())
            .expect("Initial leaf should be a hexadecimal string");

        // Give all the leafs at the last depth the initial leaf value
        let start_of_nodes_at_depth = base.pow(depth - 1);
        for i in (start_of_nodes_at_depth as usize)..mt.representation.len() {
            mt.representation[i] = as_bytes.clone()
        }

        // Always go one less in depth and compute hashes for those nodes based on their respective children
        let mut current_depth = depth - 2;
        while current_depth > 0 {
            // Go from the start of this depth
            let start_of_nodes_at_depth = base.pow(current_depth) as usize;
            let end_of_nodes_at_depth = base.pow(current_depth + 1) as usize;

            let mut seen_hashes: HashMap<String, Vec<u8>> = HashMap::new();

            for i in start_of_nodes_at_depth..end_of_nodes_at_depth {
                // retrieve left and right child hash, concatenate together and hash
                let left_child_hash = mt.representation[MerkleTree::get_left_child(i)].clone();
                let right_child_hash = mt.representation[MerkleTree::get_right_child(i)].clone();

                let concatenation = Self::concatenate_hashes(left_child_hash, right_child_hash);
                let hex_concatenation = hex::encode(concatenation.clone());

                // Check if you have seen this concatenated has before? use cached hash if you have,
                // this is going to give us O(depth-1) runtime for creating our example tree of depth = 20
                let hashed: Vec<u8> =
                    if let Some(hash) = seen_hashes.get(hex_concatenation.as_str()) {
                        hash.to_vec()
                    } else {
                        let hashed: Vec<u8> = Self::hash(concatenation);
                        // Place this into the hashmap so we can reuse the sha3 computation later
                        seen_hashes.insert(hex_concatenation.clone(), hashed.clone());
                        hashed
                    };

                mt.representation[i] = hashed;
            }
            current_depth = current_depth - 1;
        }

        // Calculate the root hash by getting the left and right child of the root node and hashing their concatenated hashes
        let root_left_child_hash = mt.get(MerkleTree::get_left_child(1));
        let root_right_child_hash = mt.get(MerkleTree::get_right_child(1));

        let concatenation = Self::concatenate_hashes(root_left_child_hash, root_right_child_hash);

        let hashed: Vec<u8> = Self::hash(concatenation);

        if let Some(elem) = mt.representation.get_mut(1) {
            *elem = hashed;
        }

        return mt;
    }

    /// Returns the root of the tree and converts it into a hexadecimal string representation
    pub fn root(&self) -> String {
        String::from("0x") + &hex::encode(&self.representation[1])
    }

    pub fn get(&self, index: usize) -> Vec<u8> {
        self.representation[index].clone()
    }
    pub fn leaf_range(&self) -> std::ops::Range<usize> {
        let base: u32 = 2;
        let start_of_nodes_at_depth = base.pow(self.depth - 1);
        (start_of_nodes_at_depth as usize)..self.representation.len()
    }

    pub fn pretty_print(&self) {
        // Print out the merkle tree with the hashes in hex
        let v: Vec<(usize, String)> = self
            .representation
            .iter()
            .map(|hash| String::from("0x") + &hex::encode(hash))
            .enumerate()
            .collect();
        for (i, v) in v {
            println!("Index {} and value {}", i, v)
        }
    }

    // Exercise 4:
    /// Sets a the hash value for a leaf node and rebalances affected nodes inthe merkle tree
    ///
    /// # Arguments
    ///
    /// * `index` - An integer indicating the the index of the leaf node to mutate
    /// * `value` - A hexadecimal string repesenting the hash to be set at this node
    ///
    pub fn set(&mut self, index: usize, value: String) {
        // Check if this is a leaf
        if !self.leaf_range().contains(&index) {
            panic!("Attempting to mutate non leaf value")
        }

        self.representation[index] =
            Self::hex_to_bytes(value).expect("Initial leaf should be a hexadecimal string");
        self.rebalance(index)
    }

    pub fn rebalance(&mut self, index: usize) {
        // go all the way to the root and recalculate hashes
        let mut current = index;
        while current > 0 {
            let parent = MerkleTree::get_parent(current);
            let left_child_hash = self.representation[MerkleTree::get_left_child(parent)].clone();
            let right_child_hash = self.representation[MerkleTree::get_right_child(parent)].clone();
            let concatenation = Self::concatenate_hashes(left_child_hash, right_child_hash);
            let hashed: Vec<u8> = Self::hash(concatenation);
            self.representation[parent] = hashed;
            current = parent;
        }
    }

    // Exercise 5:
    /// Generates the merkle proof path for a given leaf, the sibling_hash part of the path is returned as a hex string for better readability
    ///
    /// # Arguments
    ///
    /// * `leaf_index` - An integer indicating the the index of the leaf node among the group of leaves
    ///
    pub fn proof(&self, leaf_index: usize) -> Vec<(Handedness, String)> {
        let leaf_index_mapping: HashMap<usize, usize> =
            self.leaf_range().enumerate().into_iter().collect();
        let index = leaf_index_mapping
            .get(&leaf_index)
            .expect("Leaf index should correspond to an index in the leaf section")
            .clone();
        // Collect tuples of proof values where the first item of the tuple indicates if the current node is left or right handed
        // And the hash of the sibling
        let mut path: Vec<(Handedness, String)> = Vec::new();
        // If the index is even then it is a left child if the index is odd it is a right child
        let mut current = index;
        while current > 1 {
            let parent = Self::get_parent(current);
            let handedness = if current % 2 == 0 {
                Handedness::Left
            } else {
                Handedness::Right
            };
            let sibling_hash_vec = match handedness {
                Handedness::Left => {
                    self.representation[MerkleTree::get_right_child(parent)].clone()
                }
                Handedness::Right => {
                    self.representation[MerkleTree::get_left_child(parent)].clone()
                }
            };
            let sibling_hash_hex = String::from("0x") + &hex::encode(sibling_hash_vec);

            path.push((handedness, sibling_hash_hex));
            current = parent;
        }
        path
    }

    // Exercise 6:
    /// Returns the root hash calculated from a leaf node and its merkle proof path
    ///
    /// # Arguments
    ///
    /// * `path` - The merkle proof paths to use for testing
    /// * `leaf_hash` - A hexadecimal string repesenting the hash at a leaf node
    ///
    pub fn verify(path: Vec<(Handedness, String)>, leaf_hash: String) -> String {
        // Start with the leaf node hash and then fold over the path in the correct direction
        path.iter()
            .fold(leaf_hash.clone(), |acc, (handedness, sibling_hash)| {
                let hash_bytes_of_current = MerkleTree::hex_to_bytes(acc.clone()).unwrap();
                let hash_bytes_of_sibling = MerkleTree::hex_to_bytes(sibling_hash.clone()).unwrap();
                let concatenated: Vec<u8> = match handedness {
                    Handedness::Left => {
                        // The current hash should be on the left side of the concatenation
                        Self::concatenate_hashes(hash_bytes_of_current, hash_bytes_of_sibling)
                    }
                    Handedness::Right => {
                        // The current hash should be on the right side of the concatenation
                        Self::concatenate_hashes(hash_bytes_of_sibling, hash_bytes_of_current)
                    }
                };
                String::from("0x") + &hex::encode(MerkleTree::hash(concatenated))
            })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use num_bigint::BigUint;
    use num_traits::Num;

    #[test]
    fn should_create_a_merkle_tree_of_fixed_depth() {
        let initial_leaf =
            String::from("0xabababababababababababababababababababababababababababababababab");
        let mt: MerkleTree = MerkleTree::new(20, initial_leaf);
        assert_eq!(
            mt.root(),
            String::from("0xd4490f4d374ca8a44685fe9471c5b8dbe58cdffd13d30d9aba15dd29efb92930")
        );
    }

    #[test]
    #[should_panic(expected = "Initial leaf should be a hexadecimal string")]
    fn should_panic_if_initial_leaf_is_not_hex_format() {
        let initial_leaf = String::from("Unexpected");
        let _: MerkleTree = MerkleTree::new(20, initial_leaf);
    }

    #[test]
    fn should_create_a_merkle_tree_of_zero_depth_returning_a_root_only_tree() {
        let initial_leaf =
            String::from("0xabababababababababababababababababababababababababababababababab");
        let mt: MerkleTree = MerkleTree::new(0, initial_leaf);
        assert_eq!(
            mt.root(),
            String::from("0xabababababababababababababababababababababababababababababababab")
        );
    }

    #[test]
    fn should_create_a_merkle_tree_of_depth_depth_one_returning_a_root_and_leaves() {
        let initial_leaf =
            String::from("0xabababababababababababababababababababababababababababababababab");
        let mt: MerkleTree = MerkleTree::new(1, initial_leaf);
        assert_eq!(
            mt.root(),
            String::from("0x699fc94ff1ec83f1abf531030e324003e7758298281645245f7c698425a5e0e7")
        );
    }

    #[test]
    fn should_create_a_merkle_tree_and_do_an_ad_hoc_mutation() {
        let initial_leaf =
            String::from("0x0000000000000000000000000000000000000000000000000000000000000000");
        let mut mt: MerkleTree = MerkleTree::new(5, initial_leaf);

        for (i, index) in mt.leaf_range().enumerate() {
            let huge_hex_int = BigUint::from_str_radix(
                "1111111111111111111111111111111111111111111111111111111111111111",
                16,
            )
            .unwrap();
            //  I don't want to deal with padding because big int arithmetic gives me 0x0
            if i == 0 {
                mt.set(
                    index,
                    "0x0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
                )
            } else {
                mt.set(index, format!("{:#X}", (i * huge_hex_int)))
            }
        }

        assert_eq!(
            mt.root(),
            String::from("0x57054e43fa56333fd51343b09460d48b9204999c376624f52480c5593b91eff4")
        );
    }

    #[test]
    fn should_come_up_with_a_merkle_proof_path() {
        let initial_leaf =
            String::from("0x0000000000000000000000000000000000000000000000000000000000000000");
        let mut mt: MerkleTree = MerkleTree::new(5, initial_leaf);

        for (i, index) in mt.leaf_range().enumerate() {
            let huge_hex_int = BigUint::from_str_radix(
                "1111111111111111111111111111111111111111111111111111111111111111",
                16,
            )
            .unwrap();
            //  I don't want to deal with padding because big int arithmetic gives me 0x0
            if i == 0 {
                mt.set(
                    index,
                    "0x0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
                )
            } else {
                mt.set(index, format!("{:#X}", (i * huge_hex_int)))
            }
        }

        assert_eq!(
            mt.proof(3),
            vec![
                (
                    Handedness::Right,
                    "0x2222222222222222222222222222222222222222222222222222222222222222".to_owned()
                ),
                (
                    Handedness::Right,
                    "0x35e794f1b42c224a8e390ce37e141a8d74aa53e151c1d1b9a03f88c65adb9e10".to_owned()
                ),
                (
                    Handedness::Left,
                    "0x26fca7737f48fa702664c8b468e34c858e62f51762386bd0bddaa7050e0dd7c0".to_owned()
                ),
                (
                    Handedness::Left,
                    "0xe7e11a86a0c1d8d8624b1629cb58e39bb4d0364cb8cb33c4029662ab30336858".to_owned()
                )
            ]
        )
    }

    #[test]
    fn should_verify_a_merkle_proof_given_a_path_and_leaf() {
        let initial_leaf =
            String::from("0x0000000000000000000000000000000000000000000000000000000000000000");
        let mut mt: MerkleTree = MerkleTree::new(5, initial_leaf);

        for (i, index) in mt.leaf_range().enumerate() {
            let huge_hex_int = BigUint::from_str_radix(
                "1111111111111111111111111111111111111111111111111111111111111111",
                16,
            )
            .unwrap();
            //  I don't want to deal with padding because big int arithmetic gives me 0x0
            if i == 0 {
                mt.set(
                    index,
                    "0x0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
                )
            } else {
                mt.set(index, format!("{:#X}", (i * huge_hex_int)))
            }
        }

        let root = mt.root();
        let proof_path = mt.proof(3);

        assert_ne!(
            MerkleTree::verify(
                proof_path,
                "0x5555555555555555555555555555555555555555555555555555555555555555".to_owned()
            ),
            root,
            "Retrieved root should not be equal to the calculated root since the leaf does not belong to the path"
        )
    }

    #[test]
    fn should_verify_a_merkle_proof_given_a_path_and_leaf_that_belongs_to_the_path() {
        let initial_leaf =
            String::from("0x0000000000000000000000000000000000000000000000000000000000000000");
        let mut mt: MerkleTree = MerkleTree::new(5, initial_leaf);

        for (i, index) in mt.leaf_range().enumerate() {
            let huge_hex_int = BigUint::from_str_radix(
                "1111111111111111111111111111111111111111111111111111111111111111",
                16,
            )
            .unwrap();
            //  I don't want to deal with padding because big int arithmetic gives me 0x0
            if i == 0 {
                mt.set(
                    index,
                    "0x0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
                )
            } else {
                mt.set(index, format!("{:#X}", (i * huge_hex_int)))
            }
        }

        let root = mt.root();
        let proof_path = mt.proof(3);

        assert_eq!(
            MerkleTree::verify(
                proof_path,
                "0x3333333333333333333333333333333333333333333333333333333333333333".to_owned()
            ),
            root,
            "Retrieved root should be equal to the calculated root since the leaf is a part of the path"
        )
    }
}
