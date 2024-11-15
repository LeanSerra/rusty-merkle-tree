use sha3::{Digest, Sha3_256};

pub type Hash = [u8; 32];

pub struct MerkleTree {
    layers: Vec<Vec<Hash>>,
}

impl MerkleTree {
    fn default() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn from_leaves<T: std::convert::AsRef<[u8]>>(leaves: &[T]) -> Self {
        let mut tree = Self::default();
        tree.build_first_layer(leaves);
        tree
    }

    fn build_first_layer<T: std::convert::AsRef<[u8]>>(&mut self, leaves: &[T]) {
        let mut hasher = Sha3_256::new();
        let mut first_layer: Vec<[u8; 32]> = Vec::new();
        for leave in leaves {
            hasher.update(leave);
            first_layer.push(hasher.finalize_reset().into());
        }
        self.layers.push(first_layer);
    }

    pub fn print_layers(self) {
        for (i, layer) in self.layers.iter().enumerate() {
            println!("Layer {i}: {{");
            for hash in layer {
                println!("{}", hex::encode(hash));
            }
            println!("}}");
        }
    }

    fn build_tree(&mut self) {
        todo!("Implement logic to build tree");
    }
}
