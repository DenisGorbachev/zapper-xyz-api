use core::num::NonZeroU32;
use derive_more::{From, Into};
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use std::cell::LazyCell;
use std::time::Duration;

#[derive(From, Into, Debug)]
pub struct RateLimits {
    pub tokens: LazyCell<DefaultDirectRateLimiter>,
    pub nft_metadata: LazyCell<DefaultDirectRateLimiter>,
    pub search: LazyCell<DefaultDirectRateLimiter>,
    pub portfolio_balances: LazyCell<DefaultDirectRateLimiter>,
    pub transaction_history: LazyCell<DefaultDirectRateLimiter>,
    pub rankings: LazyCell<DefaultDirectRateLimiter>,
    pub other: LazyCell<DefaultDirectRateLimiter>,
}

impl Default for RateLimits {
    fn default() -> Self {
        Self {
            tokens: LazyCell::new(tokens_rate_limiter),
            nft_metadata: LazyCell::new(nft_metadata_rate_limiter),
            search: LazyCell::new(search_rate_limiter),
            portfolio_balances: LazyCell::new(portfolio_balances_rate_limiter),
            transaction_history: LazyCell::new(transaction_history_rate_limiter),
            rankings: LazyCell::new(rankings_rate_limiter),
            other: LazyCell::new(other_rate_limiter),
        }
    }
}

impl Clone for RateLimits {
    fn clone(&self) -> Self {
        Self::default()
    }
}

macro_rules! per_second_rate_limiter {
    ($function:ident, $requests_per_second:literal) => {
        fn $function() -> DefaultDirectRateLimiter {
            let requests_per_second = NonZeroU32::new($requests_per_second).expect("always succeeds because documented RPS values are non-zero");
            RateLimiter::direct(Quota::per_second(requests_per_second))
        }
    };
}

per_second_rate_limiter!(tokens_rate_limiter, 2);
per_second_rate_limiter!(nft_metadata_rate_limiter, 2);
per_second_rate_limiter!(search_rate_limiter, 2);
per_second_rate_limiter!(portfolio_balances_rate_limiter, 2);
per_second_rate_limiter!(transaction_history_rate_limiter, 2);
per_second_rate_limiter!(other_rate_limiter, 10);

fn rankings_rate_limiter() -> DefaultDirectRateLimiter {
    let period = Duration::from_secs(10);
    let quota = Quota::with_period(period).expect("always succeeds because the documented rankings period is non-zero");
    RateLimiter::direct(quota)
}
