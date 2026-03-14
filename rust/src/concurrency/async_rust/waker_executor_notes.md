# Waker and executor notes

**Purpose**: Understand how async runtimes drive futures to completion.

## Concepts to cover

1. **Future** — a state machine that can return `Poll::Pending` or `Poll::Ready(T)`.
2. **Waker** — a handle that, when invoked (`wake()`), tells the executor “this task can make progress; poll it again”.
3. **Executor** — holds a set of tasks; when a task is woken, it gets polled again until it returns `Ready` or is dropped.

## Minimal executor (ideas)

- Store spawned futures in a queue (e.g. `VecDeque`).
- When a future returns `Pending`, it must store the `Waker` somewhere (e.g. in a shared state) so that when an I/O or timer completes, something can call `waker.wake()`.
- The executor loop: pop a task, poll it; if `Pending`, the future will have arranged to be woken later; if `Ready`, done. Repeat.

## Resources

- Rust async book: [The Waker](https://rust-lang.github.io/async-book/02_execution/03_wakeups.html)
- “Build your own async runtime” posts
- Tokio’s runtime and task queue (for real-world design)

## TODO

- [ ] Implement a minimal executor that can run a few futures (e.g. timers or mock I/O).
- [ ] Trace how `Waker::wake()` flows from “event ready” back into `poll()`.
