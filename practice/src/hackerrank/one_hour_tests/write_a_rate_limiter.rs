// Keywords: rate limiter, token bucket, sliding window, throttling
/*
Question:
Problem: Rate Limiter (Sliding Window)

Prompt:
Design and implement a rate limiter that allows at most N requests per user per T seconds.

Requirements:
Input: (userId, timestamp)
Output: allow / deny

Requests arrive in increasing timestamp order

Constraints:
Large number of users
Low latency required

What this tests:

Hash maps
Queues / deques
Sliding window logic
Clean code + edge cases
Talking through trade-offs

Follow-ups (very common):
How would you scale this?
What if timestamps are out of order?
What if this runs on multiple machines?

Clarify:

1. A rate limiter is a system that limits the number of requests that cna be made to a resource within a given time period.
2. We need to write a rate limiter that takes in a set of requests and then allows them or denies a section of them
e. returns either allow or deny based on the number of request made withing a time period.

Clarifying questions:
1. What is the format of the input?
2. What is the time period - can this be a const or should this be a customisable parameter?
3. What is N number of request - same as above?
4. is there a possibility where we allow some requests and deny others?

## Handling Ambiguity

Best way to handle ambiguity is to ask the interviewer for clarification. However lets say the interviewer is not clear and says something lieke
"Do what you want? Or make your own decisions". Then you need an approach:
1. Make reasonable assumptions an document them
2. Show reasoning
3. Be flexible - you can always change your approach later if needed as information and assumptions are clarified

The key thing here is use your own experience - if not be sensible!

Rate limiter ambiguity analysis:

Having designed rate limiters and similar services such as circuit breakers before, typically they are a struct that sit on top of
In alloy-rs for example the cirucit breaker sits on top of all instances of a rpc service and acts as a future based mechanism for whether to pass or not
Based on this, for now, I will design a rate limiter as its on struct that acts as a pass through mechanism for requests.

Clarifying questions:
1. What is the format of the input?

The input is a tuple of (userId, timestamp) with no other information.

2. What is the time period - can this be a const or should this be a customisable parameter?
3. What is N number of request - same as above?

I will make these configurable parameters passed in to the new instance of the rate limiter

4. is there a possibility where we allow some requests and deny others?

For now I will just do a simple rejection or acceptance - i.e. sent or failed, however in the future we could allow for back pressure over a time period
And therefore have a storage systems of messages to batch send for example.

Time period in elapsed to start with in milliseconds, as currently rpcs takes req/s - so lets assume we need ms clarity


## Designing the algorithms

### Example walk through

Example Input(s)

Let assume, our current rate limit period is 3 req/s

(0001, 1768147613) -> Start of 1 second - request 1
(0001, 1768147813) -> request 2 - 2 in 0.2s
(0001, 1768148013) -> request 3 - 3 in 0.4s
(0001, 1768148213) -> request 4 - 4 in 0.6s
(0002, 1768148413) -> request 5 - 5 in 0.8s
(0002, 1768148613) -> request 6 - 6 in 1s

Therefore at reqs 1 - 3 should pass and return true (or enum of allowed or not) and reqs 4 - 6 should be denied and return false

What should happen at reqs 4 - 6 is to be decided after the algorithm is designed.

### Algorithm

Struct that represents the rate limiter
Inputs are time period and number of requests, these are therefore customisable as per the assumptions and clarifications made.

Function to validate the input
Input is the example input

Once the first request is made within the rate limiting period, we need to track number of requests and a timestamp
If the time passed between the rate limit period is more than the configured limiting period then a request can be made
If we are within the rate limiting period and the number of requests is less than or equal to the configured number of requests then a request can be made
Else the request cannot be made

If the request is allowed we return Allowed
If the request cannot be made we return Denied


Potential problems:
1. Timestamp in the past
2. Lost data when returning false


*/

use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
pub enum RateLimiterResult {
    Allowed,
    Denied,
}

/// Struct that represents the rate limiter
pub struct RateLimiter {
    evaluation_period: u64,
    request_rate_limit: u64,
    user_id_to_number_of_requests: HashMap<u64, VecDeque<u64>>,
}

impl RateLimiter {
    /// Inputs are time period and number of requests, these are therefore customisable as per the assumptions and clarifications made.
    /// At start time - number of requests within the time period is equal to the request rate limit
    pub fn new(evaluation_period: u64, request_rate_limit: u64) -> Self {
        Self {
            evaluation_period,
            request_rate_limit,
            user_id_to_number_of_requests: HashMap::new(),
        }
    }

    pub fn validate_rate_limit(&mut self, (user_id, timestamp): (u64, u64)) -> RateLimiterResult {
        let current_req_queue = self
            .user_id_to_number_of_requests
            .entry(user_id)
            .or_insert_with(VecDeque::new);

        // Get the current window start by going back in time by the evalation period, therefore window is [timestamp - evaluation_period, timestamp]
        let window_start = timestamp.saturating_sub(self.evaluation_period);

        // Remove all requests from the queue that are older than the window start
        while let Some(&oldest_request) = current_req_queue.front() {
            if oldest_request < window_start {
                current_req_queue.pop_front();
            } else {
                break;
            }
        }

        // If the queue is less than the request rate limit, allow and add the new request
        if current_req_queue.len() < self.request_rate_limit as usize {
            current_req_queue.push_back(timestamp);
            RateLimiterResult::Allowed
        } else {
            RateLimiterResult::Denied
        }
    }
}

#[cfg(test)]
mod write_a_rate_limiter_tests {
    use super::*;

    #[test]
    fn test_write_a_rate_limiter() {
        // Rate limiter: 3 requests per 1000ms (1 second) window
        let mut rate_limiter = RateLimiter::new(1000, 3);

        // User 1: First 3 requests should be allowed (within limit)
        // Timestamps are 200ms apart
        assert_eq!(
            rate_limiter.validate_rate_limit((1, 1768147613)),
            RateLimiterResult::Allowed,
            "First request should be allowed"
        );
        assert_eq!(
            rate_limiter.validate_rate_limit((1, 1768147813)),
            RateLimiterResult::Allowed,
            "Second request (200ms later) should be allowed"
        );
        assert_eq!(
            rate_limiter.validate_rate_limit((1, 1768148013)),
            RateLimiterResult::Allowed,
            "Third request (400ms later) should be allowed - at limit"
        );

        // User 1: 4th request should be denied (exceeds limit of 3)
        // All 3 previous requests are still within the 1000ms window
        assert_eq!(
            rate_limiter.validate_rate_limit((1, 1768148213)),
            RateLimiterResult::Denied,
            "Fourth request (600ms later) should be denied - exceeds limit"
        );

        // User 1: 5th request should be denied (still 3 requests in window)
        assert_eq!(
            rate_limiter.validate_rate_limit((1, 1768148413)),
            RateLimiterResult::Denied,
            "Fifth request (800ms later) should be denied"
        );

        // User 1: At timestamp 1768148613 (exactly 1000ms after first request)
        // Window: [1768147613, 1768148613]
        // Queue: [1768147613, 1768147813, 1768148013]
        // First request (1768147613) is at window_start boundary, so it's kept (not < window_start)
        // Count = 3, so request is denied
        assert_eq!(
            rate_limiter.validate_rate_limit((1, 1768148613)),
            RateLimiterResult::Denied,
            "Sixth request (exactly 1000ms after first) should be denied - first request is still at boundary"
        );

        // User 1: At timestamp 1768148713 (1100ms after first, so 100ms after window boundary)
        // Window: [1768147713, 1768148713]
        // First request (1768147613) < 1768147713, so it's removed
        // Queue should now have: [1768147813, 1768148013] (2 requests)
        // Count = 2 < 3, so request is allowed
        assert_eq!(
            rate_limiter.validate_rate_limit((1, 1768148713)),
            RateLimiterResult::Allowed,
            "Seventh request (1100ms after first) should be allowed - first request fell out of window"
        );

        // User 2: Different user should be tracked separately
        // User 2 should be able to make 3 requests independently
        assert_eq!(
            rate_limiter.validate_rate_limit((2, 1768148613)),
            RateLimiterResult::Allowed,
            "Different user should be allowed independently"
        );
        assert_eq!(
            rate_limiter.validate_rate_limit((2, 1768148713)),
            RateLimiterResult::Allowed,
            "Different user's second request should be allowed"
        );
        assert_eq!(
            rate_limiter.validate_rate_limit((2, 1768148813)),
            RateLimiterResult::Allowed,
            "Different user's third request should be allowed"
        );
        assert_eq!(
            rate_limiter.validate_rate_limit((2, 1768148913)),
            RateLimiterResult::Denied,
            "Different user's fourth request should be denied"
        );
    }
}
