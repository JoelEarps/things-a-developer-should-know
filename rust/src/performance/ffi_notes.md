# FFI — calling C from Rust (and vice versa)

**Purpose**: Safe wrappers around C ABI; bindgen; invariants.

## Concepts

- **`extern "C"`**: C calling convention; use for functions you call from C or that call into C.
- **bindgen**: generate Rust bindings from C headers.
- **cbindgen**: generate C headers from Rust (for exposing Rust libs to C).
- **Safe wrappers**: keep `unsafe` in a thin layer; expose a Rust API that upholds Rust’s guarantees.

## Safety

- C can do anything; Rust cannot verify it. Document invariants and what the C code must uphold.
- Panics across FFI boundary are undefined behaviour unless you catch and suppress (or the ABI supports it). Prefer returning error codes from C and mapping to `Result` in Rust.

## TODO

- [ ] Add a minimal example: a tiny C function (e.g. `int add(int a, int b)`) and call it from Rust via `extern "C"`.
- [ ] Use bindgen once on a small header; wrap the result in a safe Rust API.
- [ ] Document: when to use `#[no_mangle]`, `extern "C"`, and how to pass strings/slices carefully.
