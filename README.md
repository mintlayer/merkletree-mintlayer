# Merkle tree - Mintlayer

An implementation of merkle tree, and related tooling, such as inclusion proofs, done for the Mintlayer blockchain.

## Introduction

This library was separated from the [mintlayer-core](https://github.com/mintlayer/merkletree-mintlayer) repository because it is stable and to benefit the community, by providing a simple, and robust implementation of a merkle tree.

### Advantages

- Heavily tested
- Simplistic
- Made to be extensible
- Minimal amount of dependencies

You can include scale-codec dependency for serialization, but it can be disabled too, in which case you choose your own serialization method, if needed.

### Special assumptions

This library doesn't hash the leaves.

## Examples

You can find examples for how to get started with this library in the [examples directory](./examples/README.md). However, this is a quick example:

```
use blake2::{digest::typenum, Digest};
use merkletree::{hasher::PairHasher, tree::MerkleTree};

// You can use any hashing function you like, we use blake2b here as an example
type Blake2bHasher = blake2::Blake2b<typenum::U32>;

// A helper function that hashes data, not necessary for your application
pub fn hash_data<T: AsRef<[u8]>>(data: T) -> TreeNode {
    let mut h = Blake2bHasher::new();
    Digest::update(&mut h, data);
    h.finalize_reset().into()
}

// You can use any node type you like, as long as you use it consistently in the tree
// See the PairHasher implementation
type TreeNode = [u8; 32];

// You have to define a type that implements `PairHasher` trait, which will tell the tree how to combine different nodes
#[derive(Clone)]
pub struct HashAlgo(Blake2bHasher);

impl HashAlgo {
    pub fn new() -> Self {
        Self(Blake2bHasher::new())
    }

    pub fn write<T: AsRef<[u8]>>(&mut self, in_bytes: T) {
        Digest::update(&mut self.0, in_bytes);
    }

    pub fn finalize(&mut self) -> TreeNode {
        self.0.finalize_reset().into()
    }
}

// This is the important part, your hasher has to implement PairHasher
impl PairHasher for HashAlgo {
    type Type = TreeNode;

    fn hash_pair(left: &Self::Type, right: &Self::Type) -> Self::Type {
        let mut h = Blake2bHasher::new();
        Digest::update(&mut h, left);
        Digest::update(&mut h, right);
        h.finalize_reset().into()
    }

    fn hash_single(data: &Self::Type) -> Self::Type {
        let mut h = Blake2bHasher::new();
        Digest::update(&mut h, data);
        h.finalize_reset().into()
    }
}

fn main() {
    // You have to hash the leaves or create them (any way you like)
    let leaf1 = hash_data("0");
    let leaf2 = hash_data("1");
    let leaf3 = hash_data("2");
    let leaf4 = hash_data("3");

    // The tree is defined as a vector, from left to right
    let tree =
        MerkleTree::<TreeNode, HashAlgo>::from_leaves(vec![leaf1, leaf2, leaf3, leaf4]).unwrap();

    println!("Merkle tree root: {}", hex::encode(tree.root()));

    // We attempt to recreate the expected root manually
    let mut node10 = HashAlgo::new();
    node10.write(leaf1);
    node10.write(leaf2);

    let mut node11 = HashAlgo::new();
    node11.write(leaf3);
    node11.write(leaf4);

    let mut node00 = HashAlgo::new();
    let n10 = node10.finalize();
    node00.write(n10);
    let n11 = node11.finalize();
    node00.write(n11);

    let root_that_we_created_manually = node00.finalize();

    // the root calculated matches the one calculated by the tree
    assert_eq!(tree.root(), root_that_we_created_manually);
}
```
