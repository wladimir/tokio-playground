use anyhow::{Error, Result};

pub type CacheResult<T> = Result<T>;

pub trait CacheErrorExt<T> {
    fn cache_err(self, msg: impl Into<String>) -> CacheResult<T>;
}

impl<T> CacheErrorExt<T> for Option<T> {
    fn cache_err(self, msg: impl Into<String>) -> CacheResult<T> {
        self.ok_or_else(|| Error::msg(msg.into()))
    }
}
