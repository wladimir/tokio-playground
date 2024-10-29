use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RateLimitError {
    LimitExceeded,
}

impl Display for RateLimitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLimitError::LimitExceeded => write!(f, "rate limit exceeded"),
        }
    }
}

impl std::error::Error for RateLimitError {}
