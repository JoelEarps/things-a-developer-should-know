# Advanced traits and generics

**Purpose**: Const generics, coherence, and when to reach for them.

## Const generics

- **`const N: usize`** in type parameters: e.g. `struct Buffer<const N: usize> { data: [u8; N] }`.
- Lets you keep array size in the type without macros; useful for fixed-size buffers, matrices, etc.
- Stabilized in recent Rust; no nightly required for basic use.

## Coherence and orphan rule

- **Orphan rule**: you can implement a trait for a type only if either the trait or the type is defined in the current crate. Prevents two crates from impl’ing the same trait for the same type and causing conflicts.
- **Coherence**: the set of trait impls must be consistent; the compiler checks that no two impls overlap in a way that would make dispatch ambiguous.

## Optional: specialization (nightly)

- **`default` in impls**: allows overlapping impls where one is “more specific” and one is default. Unstable; use only if you need to specialize and understand the rules.

## TODO

- [ ] Add a tiny example: `struct FixedBuffer<const N: usize>` and a method that uses `N`.
- [ ] Document one case where the orphan rule forces a newtype or a different design.
