use std::fmt::Display;

use sha3::{Digest, Sha3_256};

pub type Hash = [u8; 32];

#[derive(Default)]
pub struct MerkleTree {
    layers: Vec<Vec<Hash>>,
}

impl Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, layer) in self.layers.iter().enumerate() {
            writeln!(f, "Layer: {idx}: {{")?;
            for hash in layer {
                writeln!(f, "\t{}", hex::encode(hash))?;
            }
            writeln!(f, "}}")?;
        }
        Ok(())
    }
}

impl MerkleTree {
    pub fn from_leaves<T: std::convert::AsRef<[u8]>>(leaves: &[T]) -> Self {
        let mut tree = Self::default();
        tree.layers.push(Self::build_first_layer(leaves));
        while let Some(previous_layer) = tree.layers.first() {
            if previous_layer.len() <= 1 {
                break;
            }
            let next_layer = Self::build_next_layer(previous_layer);
            tree.layers.insert(0, next_layer);
        }
        tree
    }

    fn build_first_layer<T: std::convert::AsRef<[u8]>>(leaves: &[T]) -> Vec<Hash> {
        let mut first_layer: Vec<[u8; 32]> = Vec::new();
        for leave in leaves {
            first_layer.push(Sha3_256::digest(leave).into());
        }
        first_layer
    }

    fn build_next_layer(previous_layer: &[Hash]) -> Vec<Hash> {
        let mut next_layer = Vec::new();
        let next_layer_size = previous_layer.len().div_ceil(2);
        let mut hasher = Sha3_256::new();
        for i in 0..next_layer_size {
            let Some(left_child) = previous_layer.get(i * 2) else {
                todo!("Handle this error");
            };
            let right_child = match previous_layer.get(i * 2 + 1) {
                Some(elem) => elem,
                None => left_child,
            };
            hasher.update(left_child);
            hasher.update(right_child);
            next_layer.push(hasher.finalize_reset().into());
        }
        next_layer
    }

    pub fn add_element<T: std::convert::AsRef<[u8]>>(&mut self, elem: &T) {
        let mut hasher = Sha3_256::new();
        let Some(last_layer) = self.layers.last_mut() else {
            return;
        };
        hasher.update(elem);
        last_layer.push(hasher.finalize_reset().into());

        // The element is the first on the tree
        if last_layer.len() == 1 {
            return;
        }
        // We need to create the root layer
        if last_layer.len() == 2 {
            hasher.update(last_layer[0]);
            hasher.update(last_layer[1]);
            self.layers.insert(0, vec![hasher.finalize().into()]);
            return;
        }
        // We have 2 or more layers
        let mut prev_idx = self.layers.len() - 1;
        let mut curr_idx = self.layers.len() - 2;

        loop {
            let previous_layer = self.layers[prev_idx].clone();
            let current_layer = &mut self.layers[curr_idx];
            let previous_layer_elem_count = previous_layer.len();

            if (previous_layer.len() - 1) % 2 == 0 {
                // Duplicate left node
                hasher.update(previous_layer[previous_layer_elem_count - 1]);
                hasher.update(previous_layer[previous_layer_elem_count - 1]);
                current_layer.push(hasher.finalize_reset().into());
            } else {
                hasher.update(previous_layer[previous_layer_elem_count - 2]);
                hasher.update(previous_layer[previous_layer_elem_count - 1]);
                let Some(last_element) = current_layer.last_mut() else {
                    panic!();
                };
                *last_element = hasher.finalize_reset().into();
            }

            if curr_idx > 0 {
                curr_idx -= 1;
            } else {
                break;
            }
            prev_idx -= 1;
        }

        let Some(first_layer) = self.layers.first() else {
            panic!()
        };

        // We need to create the last layer
        if first_layer.len() != 1 {
            hasher.update(first_layer[0]);
            hasher.update(first_layer[1]);
            self.layers.insert(0, vec![hasher.finalize().into()]);
        }
    }

    pub fn get_root(&self) -> Option<Hash> {
        match self.layers.first() {
            Some(root_layer) => root_layer.first().copied(),
            None => None,
        }
    }

    fn get_leaves(&self) -> Option<&Vec<Hash>> {
        self.layers.last()
    }

    pub fn generate_proof<T: std::convert::AsRef<[u8]>>(
        &self,
        elem: &T,
    ) -> Option<(Vec<Hash>, usize)> {
        let leaf_idx = self.get_leaf_idx(Sha3_256::digest(elem).into())?;
        let mut proof = Vec::new();
        let mut idx = leaf_idx;
        let height = self.layers.len();
        for layer in self.layers[1..height].iter().rev() {
            if idx % 2 == 0 {
                // If the right element does not exist we duplicate the left one and add it to the proof
                let right_child = match layer.get(idx + 1) {
                    Some(right) => right,
                    None => &layer[idx],
                };
                proof.push(*right_child);
            } else {
                proof.push(layer[idx - 1]);
            }
            idx /= 2;
        }

        Some((proof, leaf_idx))
    }

    pub fn verify_proof<T: std::convert::AsRef<[u8]>>(
        &self,
        elem: &T,
        proof: &Vec<Hash>,
        mut idx: usize,
    ) -> bool {
        let mut hasher = Sha3_256::new();
        hasher.update(elem);
        let mut hash: Hash = hasher.finalize_reset().into();
        for part in proof {
            if idx % 2 == 0 {
                hasher.update(hash);
                hasher.update(part);
                hash = hasher.finalize_reset().into();
            } else {
                hasher.update(part);
                hasher.update(hash);
                hash = hasher.finalize_reset().into();
            }
            idx /= 2;
        }

        let Some(root) = self.get_root() else {
            return false;
        };
        root == hash
    }

    fn get_leaf_idx(&self, target: Hash) -> Option<usize> {
        let leaves = self.get_leaves()?;
        leaves.iter().position(|e| *e == target)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree_creation_power_of_two() {
        let tree = MerkleTree::from_leaves(&["1", "2", "3", "4"]);
        assert_eq!(
            Some([
                137, 153, 44, 123, 164, 130, 79, 195, 21, 135, 186, 74, 94, 220, 125, 98, 73, 20,
                100, 119, 87, 220, 77, 185, 218, 60, 243, 252, 72, 120, 28, 89
            ]),
            tree.get_root()
        );
    }

    #[test]
    fn tree_creation_not_power_of_two() {
        let tree = MerkleTree::from_leaves(&["1", "2", "3"]);
        assert_eq!(
            Some([
                98, 188, 83, 184, 206, 109, 0, 66, 150, 238, 25, 116, 106, 0, 246, 132, 255, 226,
                108, 94, 28, 117, 105, 41, 66, 101, 52, 101, 72, 97, 26, 94
            ]),
            tree.get_root()
        );
    }

    #[test]
    fn empty_tree() {
        let leaves: Vec<String> = Vec::new();
        let tree = MerkleTree::from_leaves(&leaves);
        assert_eq!(None, tree.get_root())
    }

    #[test]
    fn tree_add_element() {
        let leaves: Vec<String> = Vec::new();
        let mut tree = MerkleTree::from_leaves(&leaves);
        assert_eq!(None, tree.get_root());
        tree.add_element(&"1");
        assert_eq!(
            Some([
                103, 177, 118, 112, 91, 70, 32, 102, 20, 33, 159, 71, 160, 90, 238, 122, 230, 163,
                237, 190, 133, 11, 187, 226, 20, 197, 54, 185, 137, 174, 164, 210
            ]),
            tree.get_root()
        );
        tree.add_element(&"2");
        assert_eq!(
            Some([
                129, 126, 89, 113, 153, 50, 84, 184, 160, 87, 205, 216, 126, 177, 231, 150, 152,
                165, 130, 249, 154, 118, 205, 125, 38, 65, 70, 141, 241, 48, 219, 0
            ]),
            tree.get_root()
        );
        tree.add_element(&"3");
        assert_eq!(
            Some([
                98, 188, 83, 184, 206, 109, 0, 66, 150, 238, 25, 116, 106, 0, 246, 132, 255, 226,
                108, 94, 28, 117, 105, 41, 66, 101, 52, 101, 72, 97, 26, 94
            ]),
            tree.get_root()
        );
        tree.add_element(&"4");
        assert_eq!(
            Some([
                137, 153, 44, 123, 164, 130, 79, 195, 21, 135, 186, 74, 94, 220, 125, 98, 73, 20,
                100, 119, 87, 220, 77, 185, 218, 60, 243, 252, 72, 120, 28, 89
            ]),
            tree.get_root()
        );
    }

    #[test]
    fn verify_generated_proof() {
        let tree = MerkleTree::from_leaves(&["1", "2", "3"]);
        let Some((proof, idx)) = tree.generate_proof(&"1") else {
            panic!()
        };
        assert!(tree.verify_proof(&"1", &proof, idx))
    }

    #[test]
    fn verify_proof_after_tree_modification() {
        let mut tree = MerkleTree::from_leaves(&["1", "2", "3"]);
        let Some((proof, idx)) = tree.generate_proof(&"1") else {
            panic!()
        };
        assert!(tree.verify_proof(&"1", &proof, idx));
        tree.add_element(&"4");
        assert!(!tree.verify_proof(&"1", &proof, idx));
    }
}
