# Pareto set — what's left to complete

Tick as you implement. Order is by pattern, then difficulty.

**Related in this repo**

- **All algorithm source files + coverage gaps:** [docs/ALGORITHMS_AND_DS_INDEX.md](../../../docs/ALGORITHMS_AND_DS_INDEX.md)
- **Algorithms crate (search, sort, complexity):** [algorithms/src/main.rs](../../../algorithms/src/main.rs) → [search_algorithms/](../../../algorithms/src/search_algorithms/), [sort_algorithms/](../../../algorithms/src/sort_algorithms/), [time_complexity_explained/](../../../algorithms/src/time_complexity_explained/)
- **Data structures crate:** [data_structures/src/lib.rs](../../../data_structures/src/lib.rs) → [sliding_window.rs](../../../data_structures/src/sliding_window.rs), [binary_trees/](../../../data_structures/src/binary_trees/), [simple_structs/](../../../data_structures/src/simple_structs/)
- **Rust std / collections (Vec, HashMap, custom vec):** `rust/src/collections/`
- **Repo-wide implementation checklist:** [docs/IMPLEMENTATION_TODO.md](../../../docs/IMPLEMENTATION_TODO.md)

---

## Arrays & Hashing

- [x] **Easy** — Find duplicates (contains duplicate) — `arrays_and_hashing/easy/find_duplicates.rs`
- [ ] **Easy** — Valid anagram — add `valid_anagram.rs`
- [ ] **Easy** — Two sum — add `two_sum.rs`
- [ ] **Medium** — Group anagrams — add `arrays_and_hashing/medium/mod.rs` + `group_anagrams.rs`
- [ ] **Medium** — Top K frequent elements — add `top_k_frequent.rs`
- [ ] **Medium** — Product of array except self — add `product_of_array_except_self.rs`
- [ ] **Medium** — Encode/decode strings (or similar) — optional

---

## Two Pointers

- [ ] **Easy** — Valid palindrome — add `two_pointers/easy/valid_palindrome.rs`
- [ ] **Easy** — Two sum II (sorted input) — add `two_sum_ii.rs`
- [ ] **Medium** — 3Sum — add `three_sum.rs`
- [ ] **Medium** — Container with most water — add `container_with_most_water.rs`
- [ ] **Medium** — Trapping rain water — add `trapping_rain_water.rs`

---

## Sliding Window

- [ ] **Easy** — Best time to buy/sell stock — add `sliding_window/easy/best_time_buy_sell.rs`
- [ ] **Medium** — Longest substring without repeating — add `longest_substring_no_repeat.rs`
- [ ] **Medium** — Longest repeating character replacement — add `longest_repeating_char_replacement.rs`
- [ ] **Hard** — Minimum window substring — add `sliding_window/hard/min_window_substring.rs`

---

## Stack

- [ ] **Easy** — Valid parentheses — add `stack/easy/valid_parentheses.rs`
- [ ] **Medium** — Min stack — add `min_stack.rs`
- [ ] **Medium** — Evaluate reverse polish notation — add `eval_rpn.rs`
- [ ] **Medium** — Generate parentheses — add `generate_parentheses.rs`
- [ ] **Hard** — Largest rectangle in histogram — add `stack/hard/largest_rectangle_histogram.rs`

---

## Binary Search

- [ ] **Easy** — Binary search — add `binary_search/easy/binary_search.rs`
- [ ] **Medium** — Search a 2D matrix — add `search_2d_matrix.rs`
- [ ] **Medium** — Find min in rotated sorted array — add `find_min_rotated.rs`
- [ ] **Medium** — Search in rotated sorted array — add `search_rotated.rs`
- [ ] **Hard** — Median of two sorted arrays — add `binary_search/hard/median_two_sorted.rs`

---

## Linked List

- [ ] **Easy** — Reverse linked list — add `linked_list/easy/reverse_list.rs`
- [ ] **Easy** — Merge two sorted lists — add `merge_two_sorted_lists.rs`
- [ ] **Medium** — Reorder list — add `reorder_list.rs`
- [ ] **Medium** — Remove nth node from end — add `remove_nth_from_end.rs`
- [ ] **Hard** — Merge K sorted lists — add `linked_list/hard/merge_k_lists.rs`

---

## Not yet added (uncomment in `pareto_set/mod.rs` when ready)

- [ ] **Backtracking** — e.g. subsets, combination sum, word search
- [ ] **Graphs** — e.g. number of islands, clone graph, course schedule
- [ ] **Heap** — e.g. merge K sorted, find median from data stream
- [ ] **Intervals** — e.g. merge intervals, insert interval
- [ ] **Trees** — e.g. max depth, same tree, invert tree, level order
- [ ] **Tries** — e.g. implement trie, word search II
- [ ] **Dynamic programming** — e.g. climb stairs, house robber, coin change
- [ ] **Greedy** — e.g. jump game, max subarray, task scheduler
- [ ] **Math & geometry** — e.g. happy number, plus one
- [ ] **Bit manipulation** — e.g. single number, number of 1 bits

---

## Quick reference — file layout

```
pareto_set/
├── TODO.md                    ← this file
├── mod.rs
├── arrays_and_hashing/
│   ├── mod.rs
│   ├── easy/
│   │   ├── mod.rs
│   │   ├── find_duplicates.rs ✓
│   │   ├── valid_anagram.rs   (todo)
│   │   └── two_sum.rs        (todo)
│   └── medium/                (todo)
├── two_pointers/
├── sliding_window/
├── stack/
├── binary_search/
└── linked_list/
```

For each new problem: add a `.rs` file, implement solution + tests, then add `pub mod problem_name;` in the right `mod.rs`.
