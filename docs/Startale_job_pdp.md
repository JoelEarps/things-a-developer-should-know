## Trading Engine

| Complete | Topic                 | Theory To Study                                                | Mini Project / Task                                               | Repo Location |
| -------- | --------------------- | -------------------------------------------------------------- | ----------------------------------------------------------------- | ------------- |
|          | Limit Order Book      | Price-time priority, order matching algorithms, memory layouts | Implement a **limit order book supporting market + limit orders** | `trading-bot` |
|          | Order Matching Engine | Matching algorithms, partial fills, trade generation           | Build **matching engine capable of processing 50k orders/sec**    | `trading-bot` |
|          | Snapshot + Diff State | State reconstruction models used by exchanges                  | Implement **snapshot + incremental orderbook diff recovery**      | `trading-bot` |
|          | Event Sourcing        | Append-only logs, replayable systems                           | Create **event log replay system for orderbook recovery**         | `trading-bot` |
|          | Market Data Pipeline  | Real-time data ingestion architecture                          | Build **trade + orderbook ingestion pipeline**                    | `trading-bot` |
|          | Strategy Engine       | Strategy evaluation loops, signal generation                   | Implement **simple strategies (market making / arbitrage)**       | `trading-bot` |
|          | Latency Optimisation  | Cache locality, zero-copy parsing, batching                    | Benchmark different **orderbook data structures**                 | `trading-bot` |
|          | WebSocket Feed        | Market data streaming protocols                                | Create **WebSocket server streaming orderbook updates**           | `trading-bot` |
|          | Risk Engine           | Margin checks, liquidation triggers                            | Implement **position + margin tracking engine**                   | `trading-bot` |
|          | Backtesting Framework | Historical simulation models                                   | Build **historical backtesting runner for strategies**            | `trading-bot` |
|          | Exchange Adapter      | API abstraction for exchanges                                  | Implement **adapter interface for Binance-like APIs**             | `trading-bot` |
|          | Load Testing          | Throughput testing and benchmarking                            | Create **order flood benchmark tool**                             | `trading-bot` |

## Observability Library

| Complete | Topic                 | Theory To Study                         | Mini Project / Task                                | Repo Location |
| -------- | --------------------- | --------------------------------------- | -------------------------------------------------- | ------------- |
|          | Metrics Fundamentals  | Counters, gauges, histograms, sampling  | Implement **config-defined metric types**          | `observe-rs`  |
|          | Config Driven Metrics | Dynamic instrumentation                 | Build **YAML-driven metrics configuration system** | `observe-rs`  |
|          | Tracing               | Structured logging, distributed tracing | Integrate **tracing spans + context propagation**  | `observe-rs`  |
|          | Prometheus Export     | Metrics scraping standards              | Implement **Prometheus exporter endpoint**         | `observe-rs`  |
|          | Latency Histograms    | Percentile metrics, SLAs                | Implement **p50/p95/p99 latency histograms**       | `observe-rs`  |
|          | Async Instrumentation | Observability for async workloads       | Build **Tokio task monitoring instrumentation**    | `observe-rs`  |
|          | Resource Monitoring   | CPU/memory/thread metrics               | Implement **runtime resource metrics collector**   | `observe-rs`  |
|          | Alert Rules           | Observability-driven alerting           | Add **configurable alert triggers**                | `observe-rs`  |
|          | Logging Pipeline      | Structured logging architecture         | Implement **JSON structured logging output**       | `observe-rs`  |
|          | Plugin System         | Modular instrumentation                 | Build **plugin system for custom metrics sources** | `observe-rs`  |

## Rust Dev Repo

| Complete | Topic                 | Theory To Study                     | Mini Project / Task                                      | Repo Location |
| -------- | --------------------- | ----------------------------------- | -------------------------------------------------------- | ------------- |
|          | Ownership & Lifetimes | Borrow checker internals, lifetimes | Implement **examples showing complex lifetime patterns** | `rust-dev`    |
|          | Async Internals       | Futures, Poll, pinning              | Implement **minimal async executor**                     | `rust-dev`    |
|          | Concurrency           | Mutex vs RwLock vs Atomics          | Implement **lock-free queue**                            | `rust-dev`    |
|          | Memory Layout         | Cache behaviour, struct packing     | Benchmark **struct layout performance**                  | `rust-dev`    |
|          | Unsafe Rust           | When and how to use unsafe          | Implement **lock-free ring buffer using unsafe**         | `rust-dev`    |
|          | Data Structures       | Algorithmic efficiency              | Implement **custom hash map**                            | `rust-dev`    |
|          | Networking            | TCP/UDP/WebSocket protocols         | Implement **high-performance async TCP server**          | `rust-dev`    |
|          | System Profiling      | CPU profiling tools                 | Benchmark programs using **flamegraphs**                 | `rust-dev`    |
|          | Algorithms            | Complexity analysis                 | Solve **Leetcode problems in Rust with optimisations**   | `rust-dev`    |
|          | Serialization         | Binary encoding formats             | Implement **custom binary protocol parser**              | `rust-dev`    |
