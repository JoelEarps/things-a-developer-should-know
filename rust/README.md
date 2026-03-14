# Rust crate — language & std patterns

This crate is **organised by domain** so you can follow a clear learning path. The folder structure matches the suggested order in **`docs/RUST_EXPERT_LEARNING_TODO.md`**.

## Folder structure (learning order)

| # | Domain | Path | Contents |
|---|--------|------|----------|
| 1 | **Fundamentals** | `src/fundamentals/` | Variables, mutability, control flow, custom types, closures |
| 2 | **Ownership** | `src/ownership/` | Borrowing, references, Copy vs Clone |
| 3 | **Types and traits** | `src/types_and_traits/` | String/str, lifetimes, dyn Trait, Option/Result combinators |
| 4 | **Memory** | `src/memory/` | Box, pointers, Unique, Pin (memory_management) |
| 5 | **Collections** | `src/collections/` | data_collections: Vec, slices, HashMap, HashSet, custom vec |
| 6 | **Iterators** | `src/iterators/` | Higher-order functions, iterator patterns |
| 7 | **Macros** | `src/macros/` | macro_rules!, procedural macros |
| 8 | **Concurrency** | `src/concurrency/` | atomic, async_rust (pin, mutex, atomics), tokio_specifics, streams |
| 9 | **Unsafe and FFI** | `src/unsafe_and_ffi/` | unsafe, safe wrappers, FFI notes |
| 10 | **Testing** | `src/testing/` | Manual mocking, Mockall |
| 11 | **Patterns** | `src/patterns/` | Feature flags, generating_timestamps |
| — | **Performance** | `src/performance/` | Docs only: performance roadmap, rayon, error handling, FFI, advanced traits |

## Running

```bash
cargo run -p rust
cargo run -p rust --bin feature_flags_test
```

## Re-exports

- `rust::AppComponentManager` — from `patterns::feature_flags` (for the feature-flags bin).
- `rust::update_cursor_fields!` — macro from `macros`.

## See also

- **Learning path**: `docs/RUST_EXPERT_LEARNING_TODO.md`
- **Implementation TODOs**: `docs/IMPLEMENTATION_TODO.md`
- **Performance roadmap**: `src/performance/performance_optimisation_in_rust/ReadMe.md`
