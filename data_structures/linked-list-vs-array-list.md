# Linked List vs Array List — Operation Complexity

| Operation | Linked List | Array List |
|-----------|-------------|------------|
| **Get** (by index) | O(n) — must traverse from head (or from tail if doubly linked and index is in second half) | O(1) — direct indexing by offset |
| **Insert** | O(1) at head (or at a known node); O(n) if by index (traverse to position first). At end: O(1) with tail pointer, O(n) if singly linked and no tail | O(1) amortized at end (append); O(n) at beginning or middle (elements must shift) |
| **Delete** | O(1) at head or at a known node (doubly linked); O(n) if by index or if singly linked and you need to find the previous node. At end: O(1) with tail (doubly), O(n) if singly | O(1) at end (pop); O(n) at beginning or middle (elements must shift) |

**Summary:** Array list wins on random access (get by index); linked list wins on insert/delete at a *known position* (e.g. at head or at a node you already have a pointer to), because no shifting is needed. When you only have an *index*, both insert and delete are O(n) for array list (shift) and O(n) for linked list (traverse to find the node).

---

## When you only need first and last element (no index access)

If you frequently need to **get (or insert/remove) only the first and last element** and never by index, use one of:

| Structure | Get first / last | Notes |
|-----------|------------------|--------|
| **Deque (double-ended queue)** | O(1) amortized at both ends | **Preferred in practice.** Contiguous or block-based (e.g. Rust `VecDeque`, C++ `std::deque`, Java `ArrayDeque`, Python `collections.deque`). Cache-friendly, good real-world performance. |
| **Doubly linked list (with head + tail pointers)** | O(1) at both ends | No amortization; true O(1) at both ends. Worse cache locality and more memory per element (two pointers). Use when you need guaranteed O(1) or when you also insert/remove in the middle given a node. |

**Recommendation:** Start with a **deque**; switch to a doubly linked list only if you need strict O(1) without amortization or middle-of-list operations at a known node.
