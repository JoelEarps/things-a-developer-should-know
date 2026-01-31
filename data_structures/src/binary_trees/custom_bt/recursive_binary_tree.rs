// This mod shows the design on my custom recursive binary tree
// This will show my work with lifetimes, ref and deref
// https://www.youtube.com/watch?v=yHi3q2Iiepc&t=549s

// The root of the tree
// We Box this function
struct Tree {
    root: Option<Box<Node>>,
}

struct Node {}
