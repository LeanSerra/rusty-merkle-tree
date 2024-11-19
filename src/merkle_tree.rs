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

    pub fn add_element<T: std::convert::AsRef<[u8]>>(&mut self, elem: T) {
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
}
