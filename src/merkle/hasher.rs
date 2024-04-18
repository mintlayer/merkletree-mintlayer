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

/// A trait that represents a hasher for a merkle tree. A hasher deals only with nodes.
/// It may either hash a node, or hash a pair into a single node in a specific order.
/// It's important to note that all inputs and outputs in the hasher are of the same type,
/// the node type, specifically.
pub trait PairHasher: Sized + Clone {
    /// The node type in the merkle tree
    type NodeType: Clone;

    /// Hash a single node and return the hash value.
    fn hash_single(data: &Self::NodeType) -> Self::NodeType;

    /// Hash a pair of nodes and return the hash value.
    fn hash_pair(left: &Self::NodeType, right: &Self::NodeType) -> Self::NodeType;
}
