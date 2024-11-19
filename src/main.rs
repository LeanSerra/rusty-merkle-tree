mod merkle_tree;

use merkle_tree::MerkleTree;

fn main() {
    let tree = MerkleTree::from_leaves(&["1", "2", "3", "4", "5"]);
    println!("tree with 4 and 5");
    println!("{tree}");
    let mut tree = MerkleTree::from_leaves(&["1", "2", "3"]);
    println!("Tree without 4 and 5");
    println!("Before adding 4");
    println!("{tree}");
    tree.add_element("4");
    println!("After adding 4 and before 5");
    println!("{tree}");
    tree.add_element("5");
    println!("After adding 5");
    println!("{tree}");
}
