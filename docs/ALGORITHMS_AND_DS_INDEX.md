# Algorithms & data structures — file index and coverage

Paths are from the repo root. Use this to see what’s implemented vs what’s still only planned.

---

## Algorithms crate (`algorithms/`)

| Area | File | In `main.rs`? | Notes |
|------|------|---------------|-------|
| **Search** | [search_algorithms/binary_search.rs](../algorithms/src/search_algorithms/binary_search.rs) | yes | Sorted search |
| **Search** | [search_algorithms/pairwise_algorithm.rs](../algorithms/src/search_algorithms/pairwise_algorithm.rs) | yes | Two pointers / pairwise |
| **Search** | [search_algorithms/ReadMe.md](../algorithms/src/search_algorithms/ReadMe.md) | — | Notes |
| **Sort** | [sort_algorithms/bubble_sort.rs](../algorithms/src/sort_algorithms/bubble_sort.rs) | yes | O(n²) |
| **Sort** | [sort_algorithms/ReadMe.md](../algorithms/src/sort_algorithms/ReadMe.md) | — | Mentions merge sort; **no `merge_sort.rs` in tree** |
| **Time complexity** | [time_complexity_explained/constant.rs](../algorithms/src/time_complexity_explained/constant.rs) | yes | Big-O examples |
| **Time complexity** | [time_complexity_explained/logarithmic.rs](../algorithms/src/time_complexity_explained/logarithmic.rs) | yes | |
| **Time complexity** | [time_complexity_explained/linear.rs](../algorithms/src/time_complexity_explained/linear.rs) | yes | |
| **Time complexity** | [time_complexity_explained/linearithmic.rs](../algorithms/src/time_complexity_explained/linearithmic.rs) | yes | |
| **Time complexity** | [time_complexity_explained/quadratic.rs](../algorithms/src/time_complexity_explained/quadratic.rs) | yes | |
| **Time complexity** | [time_complexity_explained/polynomial.rs](../algorithms/src/time_complexity_explained/polynomial.rs) | yes | |
| **Time complexity** | [time_complexity_explained/exponential.rs](../algorithms/src/time_complexity_explained/exponential.rs) | yes | |
| **Time complexity** | [time_complexity_explained/factorial.rs](../algorithms/src/time_complexity_explained/factorial.rs) | yes | |
| **Fixed point** | [fixed_point_and_floating_point/fixed_point_maths.rs](../algorithms/src/fixed_point_and_floating_point/fixed_point_maths.rs) | **no** | Not wired into `main.rs` |
| **Graph** | [graph_algorithms/ReadMe.md](../algorithms/src/graph_algorithms/ReadMe.md) | — | Lists DFS, BFS, Dijkstra, Bellman-Ford, A\* — **no `.rs` implementations in repo** |
| **Greedy** | [greedy_algorithms/ReadMe.md](../algorithms/src/greedy_algorithms/ReadMe.md) | — | **No greedy `.rs` files** |
| **Overview** | [ReadMe.md](../algorithms/src/ReadMe.md) | — | Long DSA reference (concepts only) |

**Wired from [main.rs](../algorithms/src/main.rs):** `search_algorithms`, `sort_algorithms`, `time_complexity_explained` only.

---

## Data structures crate (`data_structures/`)

| Area | File | In `lib.rs`? | Notes |
|------|------|----------------|-------|
| **Sliding window** | [src/sliding_window.rs](../data_structures/src/sliding_window.rs) | yes | Technique + stubs (`todo!()`) |
| **Binary trees** | [binary_trees/std_binary_trees.rs](../data_structures/src/binary_trees/std_binary_trees.rs) | yes | BTreeMap / ordered |
| **Binary trees** | [binary_trees/compare_trees.rs](../data_structures/src/binary_trees/compare_trees.rs) | yes | |
| **Binary trees** | [binary_trees/custom_bt/recursive_binary_tree.rs](../data_structures/src/binary_trees/custom_bt/recursive_binary_tree.rs) | yes | |
| **Stack** | [simple_structs/special_stack.rs](../data_structures/src/simple_structs/special_stack.rs) | yes | |
| **Docs** | [linked-list-vs-array-list.md](../data_structures/linked-list-vs-array-list.md) | — | |
| **Docs** | [ReadMe.md](../data_structures/ReadMe.md) | — | Lists stack, queue, heap, tree — many **not** as separate `.rs` modules |

**Wired from [lib.rs](../data_structures/src/lib.rs):** `binary_trees`, `simple_structs`, `sliding_window`.

**Not present as dedicated modules here:** generic queue, heap, singly linked list, graph adjacency list — some of that lives under [`rust/src/collections/`](../rust/src/collections/) (Vec, HashMap, custom vec) instead.

---

## LeetCode Pareto set (`leetcode/`)

- Problem solutions and checklist: [leetcode/src/pareto_set/TODO.md](../leetcode/src/pareto_set/TODO.md)
- Implements LeetCode-style problems; **orthogonal** to `algorithms/` (study algos) vs `leetcode/` (interview problems).

---

## Do you have “everything” for DSA + Pareto?

**No — gaps are normal.** Summary:

| Want | In `algorithms/` | In `data_structures/` |
|------|-------------------|------------------------|
| Binary search | yes | — |
| Two pointers (pairwise) | yes | — |
| Bubble sort | yes | — |
| Merge / quick / heap sort | **no `.rs`** (only docs) | — |
| BFS / DFS / Dijkstra | **no `.rs`** (ReadMe only) | — |
| Greedy examples | **no `.rs`** | — |
| Time complexity demos | yes (fill stubs in IMPLEMENTATION_TODO) | — |
| Stack pattern | — | yes (special_stack) |
| Sliding window notes | — | yes (stubs) |
| Trees | — | yes (binary trees) |
| HashMap / Vec deep dives | — | mostly in **rust** crate collections |

So: **algorithms** covers search + one sort + complexity; **data_structures** covers part of trees/stack/sliding window. The rest is either **docs only**, **leetcode** (per-problem), or **rust** collections.

---

## Suggested order to “complete” the set

1. **Algorithms:** add `merge_sort.rs`, graph `bfs.rs` / `dfs.rs` / `dijkstra.rs` under `graph_algorithms/`, wire `fixed_point` or drop from scope.
2. **Data structures:** finish `sliding_window.rs` + add queue/heap modules if you want them here vs leetcode-only.
3. **LeetCode:** work through [TODO.md](../leetcode/src/pareto_set/TODO.md) — that’s what “everything to be done” means for interview Pareto.
4. **Docs:** [IMPLEMENTATION_TODO.md](IMPLEMENTATION_TODO.md) still tracks repo-wide implementation tasks.
