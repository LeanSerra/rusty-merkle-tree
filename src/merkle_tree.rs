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
            if previous_layer.len() == 1 {
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
}
