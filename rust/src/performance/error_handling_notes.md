# Error handling — expert notes

**Purpose**: Library vs application errors; context; custom types.

## Library errors (downstream can inspect)

- **thiserror**: derive `Error` and `Display`, implement `From` for your error variants.
- Use **enums** for known failure modes so callers can match.
- Implement `source()` and optional `backtrace` for chaining and debugging.

## Application / binary errors (user-facing)

- **anyhow**: `Result<T, anyhow::Error>`; use `?` and `.context("step X")` to add context.
- Convert library errors with `.into()` or `?`; wrap with `.context("what you were doing")`.
- Log or display the chain; avoid losing context when bubbling up.

## Custom Error and From

- Implement `std::error::Error` (and `Display`, `Debug`).
- Implement `From<OtherError>` for your type so `?` works.
- Use `#[from]` with thiserror to generate `From` impls.

## TODO

- [ ] Add a small example in this crate: a library with `thiserror` enum and a binary that uses `anyhow` and `.context()`.
- [ ] Document when to use `Box<dyn Error>` vs a concrete enum.
