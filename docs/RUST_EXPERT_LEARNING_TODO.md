# Rust Expert Learning Path — Your TODO

A structured checklist from “strong intermediate” to **Rust expert**. Tick items as you complete them. Each section maps to code in this repo where you can practice or add notes.

---

## How to use this doc

- **Phase 1–2**: You already have most of this; treat as revision and fill any gaps.
- **Phase 3–4**: This is where “expert” lives — deep async, unsafe, tooling, performance, and systems.
- File paths are relative to repo root (e.g. `rust/src/…`).

---

## Phase 1 — Core language (revise & fill gaps)

You already have modules for most of this. Use this as a revision checklist and complete any TODOs in the files.

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| Ownership, borrowing, references | ☐ | `rust/src/borrowing_and_references.rs` | Core of Rust; explain to someone else. |
| Mutability, `mut`, `const` | ☐ | `rust/src/variables_and_mutability.rs` | |
| Copy vs Clone | ☐ | `rust/src/copy_vs_clone.rs` | Uncomment TODOs and explain. |
| Structs, enums, `impl` | ☐ | `rust/src/custom_types.rs` | |
| Control flow: `if`, `match`, `loop`, `?` | ☐ | `rust/src/control_flow.rs` | |
| String vs `&str`, ownership of strings | ☐ | `rust/src/string_and_ampersand_str.rs` | |
| Option and Result, combinators | ☐ | `rust/src/combinators/`, `custom_combinator.rs` | |
| Iterators, closures, higher-order functions | ☐ | `rust/src/higher_order_funcs.rs` | |
| Lifetimes and elision | ☐ | `rust/src/lifetimes.rs` | Be able to explain why the compiler needs them. |
| Traits, `dyn Trait`, trait objects | ☐ | `rust/src/dyn_trait.rs` | Fat pointers, vtable. |
| Declarative macros `macro_rules!` | ☐ | `rust/src/macros.rs` | |
| Error handling: `Result`, `?`, custom types | ☐ | *(add notes)* See Phase 3 “Error handling” for expert depth. |

---

## Phase 2 — Memory, collections, concurrency basics

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| Stack vs heap, `Box` | ☐ | `rust/src/box.rs`, `rust/src/memory_management/` | |
| Raw pointers, `*const T`, `*mut T` | ☐ | `rust/src/pointers.rs` | When they’re needed; no automatic GC. |
| `Vec`, slices `[T]`, `&[T]` | ☐ | `rust/src/data_collections/arrays_and_hashing/` (vector, slices) | |
| Custom `Vec`-like (RawVec, growth) | ☐ | `rust/src/data_collections/.../vector/custom_vec.rs`, `raw_vec.rs` | |
| HashMap / HashSet, iteration (`iter` vs `into_iter`) | ☐ | `rust/.../hash_structures/hashmaps/`, `hashsets/` | Implement or document custom hashmap (see `IMPLEMENTATION_TODO.md`). |
| Atomics, `Ordering` (SeqCst, Acquire, Release) | ☐ | `rust/src/atomic.rs`, `rust/src/async_rust/atomic_variables.rs` | |
| Mutex, RwLock, interior mutability | ☐ | `rust/src/async_rust/mutex.rs` | |
| Channels and message passing | ☐ | `rust/src/tokio_specifics/` (and crossbeam in practice) | |
| Testing, mocking (manual + Mockall) | ☐ | `rust/src/testing/` | |
| Feature flags / conditional compilation | ☐ | `rust/src/feature_flags/` | |

---

## Phase 3 — Async, traits, and unsafe (expert foundations)

### Async Rust (deep)

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| Future, poll, Pending vs Ready | ☐ | `rust/src/async_rust/readme.md`, tokio docs | |
| **Pin and Unpin** — why async needs it | ☐ | `rust/src/async_rust/pin.rs`, `memory_management/box_and_pin.rs` | Complete “Show this here” in `pin.rs`. |
| Waker and executor wake-up | ☐ | *(add)* `rust/src/async_rust/waker_executor_notes.md` (stub below) | Implement a tiny executor or read Tokio’s. |
| Async runtimes: Tokio (multi-thread, current-thread) | ☐ | `rust/src/tokio_specifics/` | join_handles, tasks, timeouts, cancellation. |
| `Send` and `Sync` in async code | ☐ | `rust/src/async_rust/` | When a Future must be Send. |
| Streams (async iteration) | ☐ | `rust/src/streams.rs` | |

### Traits (advanced)

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| Supertraits, trait bounds, `where` clauses | ☐ | Use existing `dyn_trait` / custom_types; add notes if needed. | |
| Orphan rule, coherence | ☐ | *(concept)* You can’t impl external trait for external type. | |
| `impl Trait` vs `dyn Trait` (static vs dynamic dispatch) | ☐ | `rust/src/dyn_trait.rs` | When to use which. |
| Trait objects: object safety | ☐ | `rust/src/dyn_trait.rs` | Which methods are allowed. |
| Const generics | ☐ | *(add)* `rust/src/advanced_traits_and_generics.md` or small example. | `struct Buffer<const N: usize>`. |
| Specialization (nightly) / default impls | ☐ | *(optional)* Document when you need “overlapping” impls. | |

### Unsafe and FFI

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| `unsafe` — when and what it allows | ☐ | `rust/src/unsafe_vs_safe.rs` | |
| Safe wrappers around unsafe code | ☐ | Same; also custom_vec / raw_vec. | |
| **FFI**: C ABI, `extern "C"`, calling C from Rust | ☐ | *(add)* `rust/src/ffi_notes.md` (stub below) | bindgen, cbindgen. |
| Invariants and “contract” of unsafe blocks | ☐ | Nomicon, Rustonomicon. | |

### Error handling (expert)

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| `thiserror` / `anyhow` patterns | ☐ | *(add)* `rust/src/error_handling_notes.md` (stub below) | Library vs application errors. |
| Context and `.context()` for debugging | ☐ | Same. | |
| Custom `Error` and `From` impls | ☐ | Same. | |

---

## Phase 4 — Performance, tooling, and systems (expert level)

### Performance (see also `rust/src/performance_optimisation_in_rust/ReadMe.md`)

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| Zero-cost abstractions, inlining, monomorphization | ☐ | Performance ReadMe, compiler docs. | |
| `#[repr(...)]`, layout, alignment | ☐ | Performance ReadMe, Nomicon. | |
| **Benchmarking**: Criterion, avoiding pitfalls | ☐ | `low-level/`, `IMPLEMENTATION_TODO.md` (object pool, arena). | |
| **Profiling**: perf, flamegraph, Instruments (macOS) | ☐ | Doc only or add `docs/profiling_rust.md`. | |
| LTO, PGO, `RUSTFLAGS`, `target-cpu=native` | ☐ | Performance ReadMe §3. | |
| **Platform-specific**: `#[cfg]`, `target_feature`, multi-versioning | ☐ | Performance ReadMe §5; implement one `cfg` example. | |
| **SIMD**: `portable-simd`, `core::arch` | ☐ | Performance ReadMe; optional small example in `rust/` or `low-level/`. | |
| Cache-friendly layout, false sharing, padding | ☐ | `IMPLEMENTATION_TODO.md` (false sharing), low-level. | |

### Allocators and low-level

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| **Arena (bump) allocator** | ☐ | `low-level/src/arena_allocator.rs` | Implement from comments; measure vs global allocator. |
| **Object pool** | ☐ | `low-level/src/object_pool.rs` | Implement; measure. |
| **LRU cache** | ☐ | `low-level/readme.md` (links), `caching/ReadMe.md` | Build your own. |
| Circular buffer | ☐ | `low-level/readme.md` | |
| `std::alloc::GlobalAlloc` (custom allocator) | ☐ | *(concept)* Optional; know it exists. | |

### Concurrency (advanced)

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| **Lock-free** structures (crossbeam, etc.) | ☐ | Performance ReadMe; optional `rust/src/async_rust/lock_free_notes.md`. | When lock-free beats mutex. |
| Rayon: parallel iterators, work-stealing | ☐ | *(add)* `rust/src/rayon_notes.md` (stub below) | When it helps / doesn’t. |
| Scoped threads vs `std::thread` | ☐ | `rust/src/tokio_specifics/scoped_vs_non_scoped.rs` | |

### Procedural macros (expert)

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| Derive macros (e.g. derive(MyTrait)) | ☐ | `projects/custom-macros/` | Implement at least one. |
| Attribute and function-like macros | ☐ | Same. | |
| `syn`, `quote`, `proc_macro` | ☐ | Same. | |

### Domain / interview

| Topic | Status | Where in repo | Notes |
|-------|--------|----------------|-------|
| HFT / trading bot risk at each stage | ☐ | `docs/trading-bot-risk-prep.md` | Fill risks and checks. |
| Pen-and-paper: linked list, binary search, reverse list | ☐ | `docs/IMPLEMENTATION_TODO.md` (interview prep). | |
| One-hour test: Top K frequent elements | ☐ | `practice/.../top_k_frequent_elements.rs` | |

---

## Suggested order (if starting from Phase 3)

1. **Pin & Unpin** → then waker/executor notes.  
2. **Error handling** (thiserror/anyhow) → use in a small CLI or service.  
3. **Unsafe & FFI** → one small C interop example.  
4. **Performance**: Criterion benchmarks for one of arena/object pool; one `#[cfg]` or SIMD example.  
5. **Procedural macros** in `projects/custom-macros`.  
6. **Low-level**: implement arena and object pool, then LRU or circular buffer.

---

## Quick reference — new stubs added in repo

| Stub file | Purpose |
|-----------|--------|
| `rust/src/async_rust/waker_executor_notes.md` | Future, Waker, minimal executor. |
| `rust/src/error_handling_notes.md` | thiserror, anyhow, context. |
| `rust/src/ffi_notes.md` | C ABI, bindgen, safe wrappers. |
| `rust/src/rayon_notes.md` | Rayon, when to use, work-stealing. |
| `rust/src/advanced_traits_and_generics.md` | Const generics, coherence (optional). |

---

## Links

- **Implementation TODOs** (algorithms, data structures, low-level): `docs/IMPLEMENTATION_TODO.md`
- **Keywords / search index**: `docs/KEYWORDS_INDEX.md`
- **Rust performance roadmap**: `rust/src/performance_optimisation_in_rust/ReadMe.md`
- **Low-level tasks**: `low-level/readme.md`
