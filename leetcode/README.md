# leetcode

LeetCode solutions by **Pareto set** (pattern + difficulty).

## Structure

- `src/pareto_set/` — problems grouped by pattern (arrays & hashing, two pointers, sliding window, stack, binary search, linked list, etc.).
- `src/pareto_set/TODO.md` — checklist of what’s left to complete.

## Run

```bash
cargo test -p leetcode
cargo run -p leetcode
```

## Adding a problem

1. Add a new `.rs` file under the right pattern (e.g. `arrays_and_hashing/easy/two_sum.rs`).
2. Declare it in the pattern’s `mod.rs` (e.g. `pub mod two_sum;`).
3. Tick the item in `src/pareto_set/TODO.md`.

## Algorithms & data structures elsewhere in the repo

See **[docs/ALGORITHMS_AND_DS_INDEX.md](../docs/ALGORITHMS_AND_DS_INDEX.md)** for every `algorithms/` and `data_structures/` file and what’s still missing vs “full” DSA.
