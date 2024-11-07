use std::num::NonZeroUsize;

use alloy::primitives::B256;
use anyhow::Result;
use lru::LruCache;
use regex::Regex;
use t3zktls_core::FilteredResponse;

pub struct RegexCache {
    cache: LruCache<B256, Regex>,
}

impl RegexCache {
    pub fn new(size: NonZeroUsize) -> Self {
        Self {
            cache: LruCache::new(size),
        }
    }

    pub fn find(&mut self, key: B256, pattern: &str, text: &str) -> Result<Vec<FilteredResponse>> {
        let regex = self.cache.try_get_or_insert(key, || Regex::new(pattern))?;

        let matches = regex.find_iter(text);

        let mut filtered_responses = Vec::new();
        for m in matches {
            let begin = m.start();
            let length = m.len();

            let s = m.as_str();

            let filtered_response = FilteredResponse {
                begin,
                length,
                content: s.as_bytes().to_vec(),
            };

            filtered_responses.push(filtered_response);
        }

        Ok(filtered_responses)
    }
}
