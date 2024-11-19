# rusty-merkle-tree
Simple implementation of a Merkle Tree built using Rust

# Running
### Clone repository
```
git clone https://github.com/LeanSerra/rusty-merkle-tree && cd rusty-merkle-tree
```
### Run
```
make
```
### Run release mode
```
make run_release
```
### Run tests
```
make test
```
# Examples
```Rust
// Create an empty tree make it mut to add more elements

let mut tree = MerkleTree::default();

// Add elements to the tree
tree.add_element(&"1");
tree.add_element(&"2");
tree.add_element(&"3");
tree.add_element(&"4");

// Generate a proof that an element exists and verify it

if let Some((proof, idx)) = tree.generate_proof(&"1") {
    assert!(tree.verify_proof(&"1", &proof, idx));
}

// Get the root of the tree

let root = tree.get_root();
```
