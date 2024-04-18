// Copyright (c) 2021-2024 RBB S.r.l
// opensource@mintlayer.org
// SPDX-License-Identifier: MIT
// Licensed under the MIT License;
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://github.com/mintlayer/merkletree-mintlayer/blob/master/LICENSE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This is a simple example where we calculate the root of a merkle tree

use blake2::{digest::typenum, Digest};
use merkletree_mintlayer::{hasher::PairHasher, tree::MerkleTree};

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
    type NodeType = TreeNode;

    fn hash_pair(left: &Self::NodeType, right: &Self::NodeType) -> Self::NodeType {
        let mut h = Blake2bHasher::new();
        Digest::update(&mut h, left);
        Digest::update(&mut h, right);
        h.finalize_reset().into()
    }

    fn hash_single(data: &Self::NodeType) -> Self::NodeType {
        let mut h = Blake2bHasher::new();
        Digest::update(&mut h, data);
        h.finalize_reset().into()
    }
}

fn main() {
    // You have to hash the leaves or create them (any way you like)
    let leaf0 = hash_data("0");
    let leaf1 = hash_data("1");
    let leaf2 = hash_data("2");
    let leaf3 = hash_data("3");

    // The tree is defined from a vector of leaves, from left to right
    let tree =
        MerkleTree::<TreeNode, HashAlgo>::from_leaves(vec![leaf0, leaf1, leaf2, leaf3]).unwrap();

    // Let's get the root
    let tree_root = tree.root();
    println!("Merkle tree root: {}", hex::encode(tree_root));

    // Let's verify some properties about this tree
    // The number of leaves is 4
    assert_eq!(tree.leaf_count().get(), 4);
    // The number of levels is 3 (4 leaves -> 2 nodes -> 1 root)
    assert_eq!(tree.level_count().get(), 3);
    // Total number of nodes in the tree (4 + 2 + 1)
    assert_eq!(tree.total_node_count().get(), 7);

    // We attempt to recreate the expected root manually
    let mut node10 = HashAlgo::new();
    node10.write(leaf0);
    node10.write(leaf1);

    let mut node11 = HashAlgo::new();
    node11.write(leaf2);
    node11.write(leaf3);

    let mut node00 = HashAlgo::new();
    let n10 = node10.finalize();
    node00.write(n10);
    let n11 = node11.finalize();
    node00.write(n11);

    let root_that_we_created_manually = node00.finalize();

    // the root calculated matches the one calculated by the tree
    assert_eq!(tree.root(), root_that_we_created_manually);
}
