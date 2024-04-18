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

// This is an example where we calculate the inclusion proof of multiple-leaves in the tree and test it

use blake2::{digest::typenum, Digest};
use merkletree::{
    hasher::PairHasher,
    proof::{
        multi::{MultiProofHashes, MultiProofNodes},
        verify_result::ProofVerifyResult,
    },
    tree::MerkleTree,
};

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

    // Proof that leaves with numbers 1 and 3, this is an abstract form of the proof that depends on the tree
    let inclusion_proof = MultiProofNodes::from_tree_leaves(&tree, &[1, 3]).unwrap();

    // Now this object is self-contained, and can be used to prove that a leaf exists, so it can be serialized and transferred over wire,
    // (feel free to use the Encode/Decode using scale-codec, but you have to enable the feature)
    let proof_hashes = inclusion_proof.into_values();

    // Now we pretend we serialized the data, and restore it from serialization, and attempted to prove that the leaf is included
    let restored_leaf_index = proof_hashes.tree_leaf_count();
    let restored_nodes = proof_hashes.nodes().clone();

    let inclusion_proof_reconstructed = MultiProofHashes::<_, HashAlgo>::from_leaf_count_and_nodes(
        restored_leaf_index,
        restored_nodes,
    );

    // Now we prove that leaf1 (of index 1) and leaf3 (of index3) exist in the tree
    // We do that by creating a map of the leaves, whose presence in the tree we want to prove, and the tree root
    assert_eq!(
        inclusion_proof_reconstructed
            .verify([(1, leaf1), (3, leaf3)].into(), tree.root())
            .unwrap(),
        ProofVerifyResult::PassedDecisively
    );
}
