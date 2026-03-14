# Rayon — parallel iterators and work-stealing

**Purpose**: When and how to use Rayon for CPU-bound parallelism.

## Concepts

- **Parallel iterators**: `par_iter()`, `into_par_iter()` — drop-in style similar to `iter()`, but work is distributed across a thread pool.
- **Work-stealing**: idle threads take work from busy threads; good for uneven workloads.
- **When it helps**: embarrassingly parallel, CPU-bound tasks (e.g. map over a large slice, parallel sort).
- **When it doesn’t**: fine-grained tasks (overhead dominates); I/O-bound (use async instead); when you need strict ordering or shared mutable state without care (use proper sync).

## Basics

- `rayon::iter::IntoParallelIterator`, `ParallelIterator`.
- `par_sort`, `par_sort_unstable` for parallel sort.
- Scope with `rayon::scope` if you need to spawn tasks that borrow from the stack.

## TODO

- [ ] Add a small benchmark: same computation with `iter()` vs `par_iter()` and compare (e.g. sum of square roots over a large slice).
- [ ] Document: thread pool size, when to avoid Rayon (e.g. inside async runtimes without care).
