# Trading bot — risk at each stage (prep for risk architecture)

Use this to prep for the **2nd part: risk architecture (F2F with Hao)**. Fill in your risks and checks at each stage. You can refer to your own bot or a hypothetical one.

---

## Diagram: stages of a trading bot

```bash
                    ┌─────────────────────────────────────────────────────────┐
                    │                    TRADING BOT FLOW                      │
                    └─────────────────────────────────────────────────────────┘

  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
  │   1. DATA    │     │  2. SIGNAL   │     │ 3. PRE-TRADE │     │  4. ORDER    │
  │   (ingest)   │────▶│  (decision)  │────▶│    RISK      │────▶│    SEND      │
  └──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘
        │                     │                     │                     │
        │                     │                     │                     │
        ▼                     ▼                     ▼                     ▼
  Market data,          Buy/sell, size,         Limits check:         To exchange /
  order book,           price, instrument      position, size,       venue (FIX,
  etc.                  from strategy         exposure, price       WebSocket, etc.)
                                                      │
                                                      │ PASS
                                                      ▼
  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
  │  6. ONGOING  │     │  5. FILL &   │     │  4. ORDER    │
  │  / MONITOR   │◀────│   POSITION   │◀────│    SEND      │
  │              │     │   UPDATE     │     │              │
  └──────────────┘     └──────────────┘     └──────────────┘
        │                     │
        │                     │
        ▼                     ▼
  P&L, drawdown,        Internal state:
  circuit breaker,      position, P&L,
  kill switch           reconcile with venue
```

---

## Your responses — fill in for each stage

### Stage 1: Data (ingest)

**What happens:**  
*(e.g. receive market data, order book, ticks)*

So lets enforce what can happen here, so we are looking in ingest data as correctly and as fast as possible. For a typical user this is the part of highest latency and it is the part of the signal that is most out of our control and we are relying on external sources to send us the data.

Our key desires here:

1. **Idempotency** — Observe events at most once (no double processing). Use an idempotency key so the same event is not applied twice.
2. **Ordering** — Process events in the right order (e.g. sequence numbers, timestamps) so we don’t act on stale or out-of-order data.
3. **Reliability** — High availability of the ingestion service from our side (reconnection, monitoring, optional secondary provider).
4. **Correctness** — Validate schema and sanity-check data (e.g. bid < ask, size > 0, valid symbol) so bad data never drives signals; reject or quarantine invalid messages.

- **Risks at this stage (what could go wrong?):**  
  *(your notes)*

So there are two key sources of failure here:

1. **Hardware / protocol failure** — Unreliable stream, spurts of data, connection drops, unreliable RPC or server connection.
2. **Data errors** — Old data, corrupt data, duplicate data (same event more than once).

- **Checks / what I'd do (or do):**  
  *(your notes)*

#### Hardware

1. **Unreliable stream / RPC or server** — Benchmark the stream; use a recognised/standard client; set up server colocation and benchmark the service to get reliable metrics for availability so you see when a failure or degradation occurs. If failures are common, use a secondary provider and enable switching.
2. **Traffic spikes** — Enable batch processing during high traffic with back pressure so the system doesn’t overload.
3. **Connection drops** — Test reconnection logic and alerting so you know when the connection drops and when it recovers.

#### Data errors

1. **Old data** — Define a threshold for “old” (e.g. max age in ms) and reject or flag data older than that; monitor when the service becomes degraded (e.g. latency, stale data rate).
2. **Corrupt data** — Validate schema (message format, required fields); use checksum/CRC if the feed provides it; reject or log-and-skip corrupt messages; monitor reject rate so you spot feed issues.
3. **Idempotency** — Create an idempotency key (e.g. unique value from two/three fields: symbol + sequence + timestamp) so the same event cannot be observed more than once and the signal is not doubled.

**Does anything need to survive a restart?**  
Yes. Persist last processed sequence number (or idempotency state) so after a restart you don’t reprocess duplicates or replay wrong data. Optionally checkpoint so you can resume from a known good point. If the protocol has sessions, you may need to re-establish connection/session state.

---

### Stage 2: Signal (decision)

**What happens:**  
*(e.g. strategy logic says “buy X at price Y, size Z”)*

The signal is an indication based on data and a strategy to perform some action (e.g. buy, short).

- **Risks at this stage:**
  - **Wrong strategy logic** — False positives (looks like a signal but isn’t); bug in maths or conditions.
  - **Wrong instrument / side / size** — Bug in strategy output (e.g. negative size, unknown symbol).
  - **Signal based on bad data** — Strategy runs on degraded, stale, or incorrect data from ingest (e.g. duplicate or out-of-order events).
  - **No kill switch** — Strategy keeps firing when it’s losing or misbehaving; no way to turn it off quickly.

- **Checks / what I'd do:**
  - **Validate strategy with testing:**
    1. **Unit tests** — Validate strategy maths and logic; cover edge cases and error scenarios.
    2. **Integration tests:**
       - **Backtesting** — Run on historical data to check it would have behaved as expected.
       - **Paper trading** — Trigger the strategy with real market data but no real orders; monitor performance.
    3. **Mirrored / shadow pipeline** — Run the strategy on live data with no output (or mocked orders) and monitor performance before enabling real trading.
    4. **Adequate rollback** — Once the strategy is released, if it misbehaves (e.g. wrong P&amp;L, bad fills), have a rollback or disable path (e.g. feature flag, kill switch per strategy).
  - **Per-strategy metrics and success criteria** — If a strategy is losing a lot or breaching criteria, turn it off automatically or alert; track P&amp;L, fill rate, slippage per strategy.
  - **Run only on validated data** — Use idempotency and ordering from Stage 1 so you don’t trigger incorrect signals from duplicate or out-of-order data; optionally disable or throttle signals when ingest shows degradation (e.g. stale data rate above threshold).

---

### Stage 3: Pre-trade risk

**What happens:**  
Before sending, we check **pre-defined boundaries** and **sanity**: max exposure, max trade size, slippage tolerance, position limits, price reasonableness, and (optionally) trading window. If any check fails, reject the order; don’t send.

- **Risks if we skip or get this wrong:**
  - **Exceed position limit** — Order would take position over max allowed (e.g. max long 1000, we send buy 500 when already long 800).
  - **Exceed size limit** — Single order larger than max (e.g. max 100, we send 500).
  - **Exceed exposure / notional limit** — Total exposure (e.g. notional) would go over cap.
  - **Wrong side or instrument** — Bug sends buy instead of sell, or wrong symbol.
  - **Trading when not allowed** — No check that we’re in a trading window or that trading isn’t halted (e.g. kill switch).
  - **Market state changing too quickly** — By the time we send, price or liquidity may have moved; pre-trade used stale state.

- **Checks / what I'd do:**
  - **Pre-defined boundaries** — Max exposure, max trade size, position limit: (current position + order size) ≤ position limit; order size ≤ max order size; exposure / notional ≤ limit. Reject if any breach.
  - **Slippage tolerance** — e.g. max acceptable % worse than target price (or min amount out); reject or don’t send if market would fill outside that band.
  - **Sanity checks** — Price reasonableness (don’t send if price moved beyond X% from signal); valid instrument, side, size &gt; 0; trading window / kill switch not active.
  - **Single source of truth for position** — Pre-trade must use the same position/state that we’ll update on fill; otherwise limits are wrong.
  - **Transaction simulation (e.g. EVM)** — Before sending, simulate the tx to see how it would affect state (e.g. will it revert? what’s the outcome?). Caveat: simulation can fail or be stale under high traffic or fast state change; use as an extra check, not a replacement for hard limits.

---

### Stage 4: Order send

**What happens:**  
Order is sent to exchange / venue (e.g. FIX, WebSocket, RPC). We wait for accept/reject or (e.g. on-chain) for tx to be confirmed.

- **Risks at this stage:**
  - **Order fails with no response / stuck pending** — Venue or RPC doesn’t respond; order is in a non-final state (e.g. pending) for too long.
  - **Market moved by execution time** — By the time the order executes, the market could have moved, especially during high volatility.
  - **RPC slow / tx status not confirmed** — e.g. issues getting tx status as confirmed (e.g. we faced this with QuickNode).
  - **Duplicate send** — Same order sent twice (e.g. retry or bug); can cause double fill or confusion.
  - **Sync/async boundary** — Sending and receiving (acks, fills) on the same path can block; batching orders and processing responses elsewhere separates send from confirm and can improve latency and reliability.

- **Checks / what I'd do:**
  - **Timeout if not confirmed** — If order has no response or is stuck in a non-finalised state, cancel or abandon after a timeout so we don’t leave it hanging.
  - **Idempotency key** — Client order ID (or tx hash once sent) to match order to expected confirmed transaction; venue or our matching can dedupe so we don’t double-count or double-send.
  - **Continuous polling / backup for confirmation** — Poll latest state (e.g. block) to see if order has been processed or was missed; use primary RPC and a backup (e.g. backup broadcast) to get confirmed tx state and watch pending → confirmed.
  - **Slippage and simulation** — e.g. minimum amount out vs actual amount out; run tx simulation before performing the trade where applicable.
  - **Retry with threshold** — Retry a failed order up to a failure threshold; if orders fail consistently, consider removing venue from the trade route (may be unstable).
  - **Venue failover** — If primary RPC or venue is slow/failing, switch to backup; monitor venue health.
  - **Don’t block send on confirm** — Where possible, send orders and handle acks/fills asynchronously (e.g. batch send, process responses elsewhere) so the send path isn’t blocked waiting for confirmation.

---

### Stage 5: Fill & position update

**What happens:**  
Exchange confirms fill; we update our position and P&amp;L. Stage 5 is about **books correct** — our position and P&amp;L must match the venue (and stay correct).

- **Risks at this stage:**
  - **Wrong fill** — Fill report has wrong size, price, or side; we update position incorrectly.
  - **Missing fill** — Order filled on venue but we never got the fill (drop, bug); we think we’re flat but we’re long → position wrong.
  - **Duplicate fill** — Same fill applied twice (no idempotency); we double-count → position wrong.
  - **Stale position** — We use an outdated position for the next pre-trade or signal → next order can breach limits.

  In Hyperliquid we subscribed to user fills data this way we had an inflight order tracker vs filled orders allowing reconciliation, we then streamed these to Kafka to allow update of the master book of all trades.

  - **No single source of truth** — Position updated from multiple places or not reconciled with venue → we don’t know if we’re in sync.doc
  - **Position moves away from strategy** — Strategy drift; we’re not where the strategy expects.
  - **Stop loss / take profit hit** — Triggers exit or strategy change; we need to act (exit, cancel strategy, or lock in profit).
  - **Fill less than expected** — Partial fill or slippage; we hit a slippage limit or market was too volatile.
  - **Does position reflect venue state?** — Our off-chain state may not match the market; we may need to re-sync to make better predictions.

- **Checks / what I'd do:**
  - **Reconcile with venue** — Periodically (or on each fill) match our position and P&amp;L to the venue (or on-chain). If mismatch, alert and fix (e.g. re-sync from venue as source of truth).
  - **Single source of truth** — All position updates from one feed (e.g. fill feed); one place that “owns” position and P&amp;L.
  - **Match fill to order** — Every fill must match an order we sent; reject or quarantine fills we don’t recognise.
  - **Idempotency on fills** — Don’t apply the same fill twice (e.g. by fill ID or order ID + fill sequence).
  - **Stop loss hit** — Exit the trade or cancel the strategy.
  - **Take profit hit** — Lock in profit (exit and realise); optionally cancel or pause strategy.
  - **Market too volatile** — Disable the market (or strategy) until stability is regained; re-enable when conditions are acceptable.

---

### Stage 6: Ongoing / monitor

**What happens:**  
We monitor live P&amp;L, drawdown, exposure, and system health. If we breach a threshold (e.g. max drawdown, max loss, runaway strategy), we trigger a circuit breaker or kill switch: stop new orders, optionally flatten positions, and alert.

- **Risks at this stage:**
  - **Runaway strategy** — Strategy keeps firing and losing; no automatic stop.
  - **Max drawdown / max loss breached** — We lose more than allowed; no circuit breaker to stop trading.
  - **Stuck or hung process** — Bot or service stops responding; orders or fills are missed; no health check or alert.
  - **Position or P&amp;L unknown** — We don't know current exposure or P&amp;L; can't make good decisions or enforce limits.
  - **No kill switch** — No way to stop all new orders (or flatten) quickly when something goes wrong.
  - **Single point of failure** — Monitoring or kill switch depends on one component; if it fails, we can't react.

- **Checks / what I'd do:**
  - **Live P&amp;L and exposure** — Real-time view of position and P&amp;L so we know where we stand; feed into limits and alerts.
  - **Max drawdown / max loss** — Define thresholds; if breached, trigger circuit breaker (e.g. stop new orders, alert, optionally flatten).
  - **Per-strategy success criteria** — If a strategy is losing a lot or breaching criteria, turn it off automatically or alert (see Stage 2).
  - **Circuit breaker / kill switch** — One action (e.g. button or API) to stop all new orders (and optionally flatten); must be fast, reliable, and tested.
  - **Health checks and alerting** — Monitor process health (e.g. heartbeat, latency); alert on failure, stale data, or missed heartbeats so we know when something is wrong.
  - **Avoid single point of failure** — Where possible, redundant monitoring or kill path (e.g. secondary process, manual override) so we can still react if one component fails.

---

## If something goes wrong end-to-end

**What would you want in place? (e.g. kill switch, max loss, stop all new orders)**

- **Kill switch** — One reliable way to stop all new orders (and optionally flatten positions) immediately; tested regularly.
- **Max loss / max drawdown** — Hard limits; if breached, circuit breaker stops new orders (and optionally flattens) and alerts.
- **Clear escalation** — Who is notified when something goes wrong; runbooks for "strategy misbehaving," "venue down," "position mismatch," etc.
- **Post-incident** — Reconcile position and P&amp;L with venue; root-cause and fix; review and tighten limits or controls if needed.

---

*When you’ve filled this in, you can share it and we can review your responses.*
