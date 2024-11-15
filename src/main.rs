mod merkle_tree;

use merkle_tree::MerkleTree;

fn main() {
    let tree = MerkleTree::from_leaves(&["1", "2", "3", "4"]);
    tree.print_layers();
}
