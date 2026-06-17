use core::num::NonZeroU32;
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use std::sync::LazyLock;
use std::time::Duration;

#[derive(Debug)]
pub struct RateLimits {
    pub tokens: LazyLock<DefaultDirectRateLimiter>,
    pub nft_metadata: LazyLock<DefaultDirectRateLimiter>,
    pub search: LazyLock<DefaultDirectRateLimiter>,
    pub portfolio_balances: LazyLock<DefaultDirectRateLimiter>,
    pub transaction_history: LazyLock<DefaultDirectRateLimiter>,
    pub rankings: LazyLock<DefaultDirectRateLimiter>,
    pub other: LazyLock<DefaultDirectRateLimiter>,
}

impl Default for RateLimits {
    fn default() -> Self {
        Self {
            tokens: LazyLock::new(tokens_rate_limiter),
            nft_metadata: LazyLock::new(nft_metadata_rate_limiter),
            search: LazyLock::new(search_rate_limiter),
            portfolio_balances: LazyLock::new(portfolio_balances_rate_limiter),
            transaction_history: LazyLock::new(transaction_history_rate_limiter),
            rankings: LazyLock::new(rankings_rate_limiter),
            other: LazyLock::new(other_rate_limiter),
        }
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
