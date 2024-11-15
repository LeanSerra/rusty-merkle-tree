use sha3::{Digest, Sha3_256};

fn main() {
    let mut hasher = Sha3_256::new();
    hasher.update(b"1");
    let hash = hasher.finalize();
    assert_eq!(
        "67b176705b46206614219f47a05aee7ae6a3edbe850bbbe214c536b989aea4d2",
        hex::encode(hash)
    );
}
