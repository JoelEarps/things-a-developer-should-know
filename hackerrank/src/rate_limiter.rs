//! Rate limiting algorithms behind a common [`RateLimiter`] wrapper.
//!
//! | Algorithm      | Limits by…                    | Burst behaviour                          |
//! |----------------|-------------------------------|------------------------------------------|
//! | Token bucket   | Tokens you collect            | Large upfront burst, then drip refill    |
//! | Leaky bucket   | Queue that drains             | Fills from empty; leaks at fixed rate    |
//! | Fixed window   | Count per calendar time slice | Double burst possible at slice edges     |
//! | Sliding window | Weighted count over ~2 slices | No full boundary burst; approximate      |
//!
//! All algorithms are **event-driven**: refill / window logic runs inside [`RateLimtingAlgorithm::is_allowed`]
//! on each request. No background task is required.

use std::{
    cmp::min,
    time::{Duration, Instant},
};

/// Generic wrapper — delegates to whichever [`RateLimtingAlgorithm`] it holds.
pub struct RateLimiter<R: RateLimtingAlgorithm> {
    algorithm: R,
}

impl<R: RateLimtingAlgorithm> RateLimiter<R> {
    pub(crate) fn attempt_request(&mut self) -> bool {
        self.algorithm.is_allowed()
    }
}

impl RateLimiter<TokenBucket> {
    /// Default: capacity 10, refill 5 tokens/sec.
    pub fn with_token_bucket() -> Self {
        Self {
            algorithm: TokenBucket::default(),
        }
    }

    pub fn with_token_bucket_config(capacity: usize, refill_rate: usize) -> Self {
        Self {
            algorithm: TokenBucket::new(capacity, refill_rate),
        }
    }
}

impl RateLimiter<FixedWindow> {
    pub fn with_fixed_window(requests_per_window: usize, window_duration: Duration) -> Self {
        Self {
            algorithm: FixedWindow::new(requests_per_window, window_duration),
        }
    }
}

impl RateLimiter<SlidingWindow> {
    pub fn with_sliding_window(requests_per_window: usize, window_duration: Duration) -> Self {
        Self {
            algorithm: SlidingWindow::new(requests_per_window, window_duration),
        }
    }
}

impl RateLimiter<LeakyBucket> {
    pub fn with_leaky_bucket(capacity: usize, leak_rate: usize) -> Self {
        Self {
            algorithm: LeakyBucket::new(capacity, leak_rate),
        }
    }
}

pub trait RateLimtingAlgorithm {
    /// Returns `true` if the request is within rate limits.
    fn is_allowed(&mut self) -> bool;
}

// -----------------------------------------------------------------------------
// Token bucket
// -----------------------------------------------------------------------------

/// Tokens drip in at `refill_rate` per second, capped at `capacity`.
///
/// Starts full so callers get an initial burst. Event-driven: [`TokenBucket::drip_refill`]
/// runs on every check — no background task.
pub struct TokenBucket {
    /// Maximum burst size.
    capacity: usize,
    /// Tokens available right now.
    tokens: usize,
    /// Tokens added per second.
    refill_rate: usize,
    /// When we last computed the drip (not the last request time).
    last_refill: Instant,
}

impl Default for TokenBucket {
    fn default() -> Self {
        TokenBucket::new(10, 5)
    }
}

impl TokenBucket {
    fn new(capacity: usize, refill_rate: usize) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    /// Add `elapsed × refill_rate` tokens, capped at [`Self::capacity`].
    fn drip_refill(&mut self) {
        let now = Instant::now();
        let elapsed_secs = now.duration_since(self.last_refill).as_secs() as usize;
        let added = elapsed_secs * self.refill_rate;
        self.tokens = min(self.tokens + added, self.capacity);
        self.last_refill = now;
    }
}

impl RateLimtingAlgorithm for TokenBucket {
    fn is_allowed(&mut self) -> bool {
        self.drip_refill();

        if self.tokens >= 1 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}

// -----------------------------------------------------------------------------
// Leaky bucket — TODO: implement yourself (comments only, see key points below)
// -----------------------------------------------------------------------------
//
// Analogy
//   Water pours in from the top; a hole drains at a fixed rate. Pour faster than
//   it leaks → overflow → reject the request.
//
// vs token bucket (read this before coding)
//   Token bucket: starts FULL — permission drips in, you spend tokens to allow.
//   Leaky bucket: starts EMPTY — requests pile up, bucket leaks out at a fixed rate.
//   Token bucket burst = upfront spike. Leaky bucket burst = how fast clients pour in.
//
// Event-driven (same as your other algorithms)
//   No background task. On every request: drain first, then decide.
//

/// Requests enter a queue and "leak" out at a fixed rate.
///
/// Starts empty ([`Self::level`] = 0). Event-driven: [`LeakyBucket::drip_drain`] runs on every check.
pub struct LeakyBucket {
    capacity: usize,
    level: usize,
    leak_rate: usize,
    last_leak: Instant,
}

impl LeakyBucket {
    fn new(max_capacity: usize, leak_rate: usize) -> Self {
        Self {
            capacity: max_capacity,
            level: 0,
            leak_rate,
            last_leak: Instant::now(),
        }
    }

    fn drip_drain(&mut self) {
        let now = Instant::now();
        let elapsed_secs = now.duration_since(self.last_leak).as_secs() as usize;
        let tokens_leaked = elapsed_secs * self.leak_rate;
        self.level = if let Some(tokens_left_after_leak) = self.level.checked_sub(tokens_leaked) {
            println!("Level doesn't go beyond bucket");
            tokens_left_after_leak
        } else {
            println!("Drained below 0");
            0
        };
        self.last_leak = now;
    }
}

impl RateLimtingAlgorithm for LeakyBucket {
    fn is_allowed(&mut self) -> bool {
        self.drip_drain();

        let new_level = self.level + 1;

        if new_level > self.capacity {
            println!("Increasing request will cause bucket to overflow, rejecting");
            false
        } else {
            println!(
                "Allowing request through, space left in bucket : {}",
                self.capacity - new_level
            );
            self.level = new_level;
            true
        }
    }
}

// -----------------------------------------------------------------------------
// Fixed window
// -----------------------------------------------------------------------------

/// A counter per fixed time slice (e.g. "10 requests per 2 seconds").
///
/// Simple, but allows a **boundary burst**: 10 requests at the end of one window and
/// 10 at the start of the next can pass 20 requests in ~2 seconds of wall-clock time.
pub struct FixedWindow {
    requests_per_window: usize,
    window_duration: Duration,
    window_start: Instant,
    /// Countdown of requests left in the current window.
    current_requests_remaining: usize,
}

impl FixedWindow {
    fn new(requests_per_window: usize, window_duration: Duration) -> Self {
        Self {
            requests_per_window,
            window_duration,
            window_start: Instant::now(),
            current_requests_remaining: requests_per_window,
        }
    }

    /// If the current slice has expired, start a new window and restore the full quota.
    fn maybe_reset_window(&mut self) {
        if self.window_start.elapsed() >= self.window_duration {
            self.window_start = Instant::now();
            self.current_requests_remaining = self.requests_per_window;
        }
    }
}

impl RateLimtingAlgorithm for FixedWindow {
    fn is_allowed(&mut self) -> bool {
        self.maybe_reset_window();

        if self.current_requests_remaining == 0 {
            false
        } else {
            self.current_requests_remaining -= 1;
            true
        }
    }
}

// -----------------------------------------------------------------------------
// Sliding window (counter-based)
// -----------------------------------------------------------------------------

/// Approximates a rolling window using two counters instead of storing every timestamp.
///
/// Used in production (e.g. Redis, Cloudflare) when a per-request log is too expensive.
/// Accurate enough to prevent the fixed-window boundary burst; see tests.
pub struct SlidingWindow {
    max_requests_per_window: usize,
    window_size: Duration,
    /// Start of the current fixed time slice.
    current_window_start: Instant,
    /// Requests counted in the current slice.
    current_window_count: usize,
    /// Requests counted in the slice before the current one.
    previous_window_count: usize,
}

impl SlidingWindow {
    pub fn new(requests_per_window: usize, window_duration: Duration) -> Self {
        Self {
            max_requests_per_window: requests_per_window,
            window_size: window_duration,
            current_window_start: Instant::now(),
            current_window_count: 0,
            previous_window_count: 0,
        }
    }
}

impl RateLimtingAlgorithm for SlidingWindow {
    fn is_allowed(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.current_window_start);

        if elapsed >= self.window_size {
            let windows_passed = elapsed.as_secs() / self.window_size.as_secs();

            if windows_passed == 1 {
                // One slice elapsed: shift current into previous, start a new slice.
                self.previous_window_count = self.current_window_count;
                self.current_window_count = 0;
                self.current_window_start += self.window_size;
            } else {
                // Idle too long: old counts no longer overlap the rolling window.
                self.previous_window_count = 0;
                self.current_window_count = 0;
                self.current_window_start = now;
            }
        }

        // How far we are through the current fixed slice (0.0 at start → 1.0 at end).
        let elapsed_in_window = now.duration_since(self.current_window_start).as_secs_f64();
        let window_length_secs = self.window_size.as_secs_f64();
        let weight = elapsed_in_window / window_length_secs;

        // `(1 - weight)` is the fraction of the *previous* slice still inside the
        // rolling window. Example: limit 10 / 2s, 10 requests at t=0, check at t=1s
        // → weight 0.5 → estimate 10 × 0.5 + 0 = 5 → ~5 more allowed, not 10.
        let estimated_count =
            self.previous_window_count as f64 * (1.0 - weight) + self.current_window_count as f64;

        if estimated_count >= self.max_requests_per_window as f64 {
            false
        } else {
            self.current_window_count += 1;
            true
        }
    }
}

#[cfg(test)]
mod rate_limiter_algo_tests {
    use std::time::Duration;

    use crate::rate_limiter::{RateLimiter, TokenBucket};

    #[test]
    fn token_bucket() {
        let mut limiter = RateLimiter::<TokenBucket>::with_token_bucket();

        for i in 1..=10 {
            assert!(
                limiter.attempt_request(),
                "request {i} should be allowed (capacity 10)"
            );
        }

        assert!(
            !limiter.attempt_request(),
            "11th request should be denied when bucket is empty"
        );

        std::thread::sleep(Duration::from_secs(3));

        for i in 1..=5 {
            assert!(
                limiter.attempt_request(),
                "request {i} after refill should be allowed"
            );
        }
    }

    #[test]
    fn partial_refill_during_traffic() {
        let mut limiter = RateLimiter::<TokenBucket>::with_token_bucket();

        for i in 1..=10 {
            assert!(
                limiter.attempt_request(),
                "request {i} should be allowed (capacity 10)"
            );
        }

        assert!(
            !limiter.attempt_request(),
            "11th request should be denied when bucket is empty"
        );

        std::thread::sleep(Duration::from_secs(1));

        for i in 1..=5 {
            assert!(
                limiter.attempt_request(),
                "request {i} after partial drip should be allowed"
            );
        }

        assert!(
            !limiter.attempt_request(),
            "6th request after partial refill should be denied"
        );
    }

    /// Token bucket burst is intentional: 10 immediately, drip recovery, not a calendar reset.
    #[test]
    fn token_bucket_burst_behavior() {
        let capacity = 10;
        let refill_rate = 5;
        let mut limiter = RateLimiter::with_token_bucket_config(capacity, refill_rate);

        let initial_burst: usize = (0..capacity)
            .map(|_| limiter.attempt_request() as usize)
            .sum();
        assert_eq!(initial_burst, capacity);
        assert!(!limiter.attempt_request());

        std::thread::sleep(Duration::from_secs(1));

        let partial_burst: usize = (0..refill_rate)
            .map(|_| limiter.attempt_request() as usize)
            .sum();
        assert_eq!(partial_burst, refill_rate);
        assert!(!limiter.attempt_request());

        std::thread::sleep(Duration::from_secs(2));

        let second_burst: usize = (0..capacity)
            .map(|_| limiter.attempt_request() as usize)
            .sum();
        assert_eq!(second_burst, capacity);
        assert!(!limiter.attempt_request());

        assert_eq!(
            initial_burst + partial_burst + second_burst,
            capacity + refill_rate + capacity
        );
    }

    #[test]
    fn fixed_window_exhausts_within_window() {
        let mut limiter = RateLimiter::with_fixed_window(10, Duration::from_secs(2));

        for i in 1..=10 {
            assert!(
                limiter.attempt_request(),
                "request {i} should be allowed (10 per 2s window)"
            );
        }

        assert!(
            !limiter.attempt_request(),
            "11th request in the same window should be denied"
        );
    }

    #[test]
    fn fixed_window_resets_after_window_expires() {
        let mut limiter = RateLimiter::with_fixed_window(10, Duration::from_secs(2));

        for _ in 0..10 {
            assert!(limiter.attempt_request());
        }
        assert!(!limiter.attempt_request());

        std::thread::sleep(Duration::from_secs(2));

        assert!(
            limiter.attempt_request(),
            "first request in a new window should be allowed"
        );
    }

    /// Fixed window allows 10 + 10 at slice boundaries (~2× the intended rate).
    #[test]
    fn fixed_window_boundary_burst() {
        let window = Duration::from_secs(2);
        let limit = 10;
        let mut limiter = RateLimiter::with_fixed_window(limit, window);

        let allowed_window_1: usize = (0..limit).map(|_| limiter.attempt_request() as usize).sum();
        assert_eq!(allowed_window_1, limit);
        assert!(!limiter.attempt_request());

        std::thread::sleep(window);

        let allowed_window_2: usize = (0..limit).map(|_| limiter.attempt_request() as usize).sum();
        assert_eq!(allowed_window_2, limit);
        assert!(!limiter.attempt_request());

        assert_eq!(allowed_window_1 + allowed_window_2, limit * 2);
    }

    #[test]
    fn sliding_window_exhausts_within_period() {
        let mut limiter = RateLimiter::with_sliding_window(10, Duration::from_secs(2));

        for i in 1..=10 {
            assert!(
                limiter.attempt_request(),
                "request {i} should be allowed (10 per 2s sliding window)"
            );
        }

        assert!(
            !limiter.attempt_request(),
            "11th request should be denied while prior requests still count"
        );
    }

    #[test]
    fn sliding_window_still_limited_after_short_wait() {
        let mut limiter = RateLimiter::with_sliding_window(10, Duration::from_secs(2));

        for _ in 0..10 {
            assert!(limiter.attempt_request());
        }
        assert!(!limiter.attempt_request());

        std::thread::sleep(Duration::from_secs(1));

        assert!(
            !limiter.attempt_request(),
            "after 1s, sliding window should still deny (unlike token bucket partial refill)"
        );
    }

    /// Sliding window blocks a full second burst at the slice boundary (fixed window allows 10).
    #[test]
    fn sliding_window_no_boundary_double_burst() {
        let limit = 10;
        let window = Duration::from_secs(2);
        let mut limiter = RateLimiter::with_sliding_window(limit, window);

        let first_burst: usize = (0..limit).map(|_| limiter.attempt_request() as usize).sum();
        assert_eq!(first_burst, limit);
        assert!(!limiter.attempt_request());

        std::thread::sleep(window);

        let second_burst: usize = (0..limit).map(|_| limiter.attempt_request() as usize).sum();
        assert!(
            second_burst < limit,
            "sliding window should not allow a full second burst of {limit} (fixed window would); got {second_burst}"
        );

        std::thread::sleep(Duration::from_secs(1));

        let partial: usize = (0..limit).map(|_| limiter.attempt_request() as usize).sum();
        assert!(
            (4..=6).contains(&partial),
            "roughly half the limit should open as the previous window fades out; got {partial}"
        );
    }

    #[test]
    fn sliding_window_allows_after_full_window_elapses() {
        let mut limiter = RateLimiter::with_sliding_window(10, Duration::from_secs(2));

        for _ in 0..10 {
            assert!(limiter.attempt_request());
        }
        assert!(!limiter.attempt_request());

        std::thread::sleep(Duration::from_secs(3));

        assert!(
            limiter.attempt_request(),
            "after the full sliding period, requests should be allowed again"
        );
    }

    #[test]
    fn leaky_bucket_fills_to_capacity() {
        let mut limiter = RateLimiter::with_leaky_bucket(5, 2);

        for i in 1..=5 {
            assert!(
                limiter.attempt_request(),
                "request {i} should fit while bucket fills from empty"
            );
        }

        assert!(
            !limiter.attempt_request(),
            "6th request should be denied when bucket is full"
        );
    }

    #[test]
    fn leaky_bucket_drains_over_time() {
        let mut limiter = RateLimiter::with_leaky_bucket(5, 2);

        for _ in 0..5 {
            assert!(limiter.attempt_request());
        }
        assert!(!limiter.attempt_request());

        std::thread::sleep(Duration::from_secs(1));

        for i in 1..=2 {
            assert!(
                limiter.attempt_request(),
                "request {i} should fit after ~2 requests leaked out in 1 second"
            );
        }

        assert!(
            !limiter.attempt_request(),
            "bucket should be full again after accepting 2 more"
        );
    }

    #[test]
    fn leaky_bucket_starts_empty() {
        let mut limiter = RateLimiter::with_leaky_bucket(10, 5);

        assert!(
            limiter.attempt_request(),
            "first request should be allowed from an empty bucket"
        );

        // Token bucket with default config would still allow 9 more immediately
        // because it starts at capacity 10 — leaky bucket must climb from 0.
        let mut token_limiter = RateLimiter::<TokenBucket>::with_token_bucket();
        let token_burst: usize = (0..10)
            .map(|_| token_limiter.attempt_request() as usize)
            .sum();
        assert_eq!(
            token_burst, 10,
            "token bucket starts full and bursts immediately"
        );

        let mut leaky_limiter = RateLimiter::with_leaky_bucket(10, 5);
        for _ in 0..10 {
            assert!(leaky_limiter.attempt_request());
        }
        assert!(
            !leaky_limiter.attempt_request(),
            "leaky bucket fills gradually to capacity then denies"
        );
    }
}
