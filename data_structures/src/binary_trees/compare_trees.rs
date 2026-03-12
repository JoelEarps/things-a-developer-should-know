// Question:
// Given the binary tree node structure below, design an algorithm to compare
// if two trees are exactly the same.
//
// For example:
// - Case 1 should return false
// - Case 2 should return true
// - Case 3 should return false
//
// (Java-style reference from the prompt)
// /**
//  * Definition for a binary tree node.
//  * public class TreeNode {
//  *     public int val;
//  *     public TreeNode left;
//  *     public TreeNode right;
//  *     public TreeNode(int x) { val = x; }
//  * }
//  */
//
/// Essentially building a binary tree structure here
/// TODO: Insert function, delete function, search function, print function
///
/// ## Where do values go? (This is NOT a binary search tree)
///
/// In this tree there is no rule like "smaller left, bigger right". **You** decide
/// where every value goes when you build the tree.
///
/// - **Left** and **right** are just two *slots* (positions) under a node. Each slot
///   can hold a whole subtree (or None). The value in the left slot has no special
///   relationship to the value in the right slot — they're independent.
/// - You could put 10 in the left slot and 10 in the right slot of the same node:
///   `TreeNode::node(3, TreeNode::leaf(10), TreeNode::leaf(10))` → node 3 with two
///   children both containing 10.
/// - "Where does 9 go?" — wherever you want. Examples:
///   - Left child of 2: replace `TreeNode::leaf(2)` with
///     `TreeNode::node(2, TreeNode::leaf(9), None)`.
///   - Right child of 4: replace `TreeNode::leaf(4)` with
///     `TreeNode::node(4, None, TreeNode::leaf(9))`.
///   There is no insert-by-value rule; you're just constructing a specific shape.
///
/// ## What this is and isn't
///
/// - **This is a binary tree.** A binary tree is defined by *shape*: each node has
///   at most two children (left and right). This struct has exactly that, so it's
///   a binary tree.
/// - **This is not a binary search tree (BST).** A BST is a binary tree *plus* an
///   ordering invariant (e.g. left subtree < node < right subtree). Here there is
///   no ordering rule — values can go in any position. So: general (unordered)
///   binary tree.

/// Simple binary tree node definition for this question.
///
/// Implementing PartialEq so that we can compare two trees using the == operator.
/// Eq is not implemented as we do not need to compare two trees for equality, we only need to compare the structure and values.
///
/// ## Why is `Box` used?
///
/// In Rust, every type must have a size known at compile time. A `TreeNode` that held
/// `Option<TreeNode>` for left/right would be infinitely sized: TreeNode contains
/// TreeNode contains TreeNode… `Box<T>` is a fixed-size pointer to heap-allocated `T`,
/// so `Option<Box<TreeNode>>` has a known size and allows the recursive structure.
/// See also: [compare_tree_equality_diagram.md](./compare_tree_equality_diagram.md).
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    /// Build an optional root for a node with the given value and optional left/right subtrees.
    /// Makes it easy to construct trees in tests: e.g. `TreeNode::node(1, TreeNode::leaf(2), None)`.
    pub fn node(
        val: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    ) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode { val, left, right }))
    }

    /// Build a leaf (no children) as an optional root.
    pub fn leaf(val: i32) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode::new(val)))
    }
}

/// Manual recursive equality: two optional subtrees are equal if both None,
/// or both Some and the two nodes are equal (which recurses).
fn opt_tree_eq(a: &Option<Box<TreeNode>>, b: &Option<Box<TreeNode>>) -> bool {
    match (a, b) {
        // End of the recursion, both three nodes are None, so they are equal and empty
        (None, None) => true,
        // Both nodes are Some, so we need to compare the values and the subtrees and recurse if necessary
        (Some(x), Some(y)) => x.eq(y),
        _ => false,
    }
}

/// PartialEq
impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        // Check values of the trees are the same
        self.val == other.val
            // Recurse on the left and right subtrees
            && opt_tree_eq(&self.left, &other.left)
            && opt_tree_eq(&self.right, &other.right)
    }
}


/// Why is this empty
impl Eq for TreeNode {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Case 1a from the problem: two identical trees → true.
    #[test]
    fn example_same_tree() {
        // Tree:    1
        //         / \
        //        2   3
        let tree_a = TreeNode::node(
            1,
            TreeNode::leaf(2),
            TreeNode::leaf(3),
        );
        let tree_b = TreeNode::node(
            1,
            TreeNode::leaf(2),
            TreeNode::leaf(3),
        );
        assert_eq!(tree_a, tree_b);
    }
    /// Case 1b: two identical trees with greater depth → true.
    #[test]
    fn example_bigger_depth() {
        // Tree:    1
        //         / \
        //        2   3
        //           / \
        //         10   4
        let tree_a = TreeNode::node(
            1,
            TreeNode::leaf(2),
            TreeNode::node(3, TreeNode::leaf(10), TreeNode::leaf(4)),
        );
        let tree_b = TreeNode::node(
            1,
            TreeNode::leaf(2),
            TreeNode::node(3, TreeNode::leaf(10), TreeNode::leaf(4)),
        );
        assert_eq!(tree_a, tree_b);
    }

    /// Case 2: same values, different structure → false.
    #[test]
    fn example_different_structure() {
        // Tree A:  1     Tree B:  1
        //         /                \
        //        2                  2
        let tree_a = TreeNode::node(1, TreeNode::leaf(2), None);
        let tree_b = TreeNode::node(1, None, TreeNode::leaf(2));
        assert_ne!(tree_a, tree_b);
    }

    /// Case 3: same structure, different value → false.
    #[test]
    fn example_different_value() {
        // Tree A:  1     Tree B:  1
        //         / \           / \
        //        2   3         9   3
        let tree_a = TreeNode::node(1, TreeNode::leaf(2), TreeNode::leaf(3));
        let tree_b = TreeNode::node(1, TreeNode::leaf(9), TreeNode::leaf(3));
        assert_ne!(tree_a, tree_b);
    }

    #[test]
    fn both_empty() {
        let a: Option<Box<TreeNode>> = None;
        let b: Option<Box<TreeNode>> = None;
        assert_eq!(a, b);
    }

    #[test]
    fn one_empty_one_not() {
        let empty: Option<Box<TreeNode>> = None;
        let one_node = TreeNode::leaf(1);
        assert_ne!(empty, one_node);
    }
}

// ---------------------------------------------------------------------------
// Time and space complexity (equality algorithm)
// ---------------------------------------------------------------------------
//
// Let n = number of nodes in the tree(s). We assume both trees have O(n) nodes.
//
// **Time complexity: O(n)**
// - We visit each node at most once in each tree. At every step we do O(1) work
//   (compare values, recurse). If we find a mismatch we stop; in the worst case
//   we traverse all nodes of the smaller tree or both trees. So worst case O(n).
//
// **Space complexity (auxiliary): O(h)** where h = height of the tree.
// - The only extra space is the *recursion call stack*. We recurse once per level
//   along a path from root to a leaf.
// - **Worst case:** O(n). A skewed tree (e.g. a linked list) has height h = n, so
//   we have n nested calls.
// - **Best case:** O(log n). A balanced tree has height h ≈ log₂(n), so the stack
//   depth is logarithmic.



