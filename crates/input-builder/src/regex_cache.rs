use std::num::NonZeroUsize;

use anyhow::Result;
use lru::LruCache;
use regex::Regex;

use crate::FilteredResponse;

pub struct RegexCache {
    cache: LruCache<String, Regex>,
}

impl RegexCache {
    pub fn new(size: NonZeroUsize) -> Self {
        Self {
            cache: LruCache::new(size),
        }
    }

    pub fn find(&mut self, pattern: &str, text: &str) -> Result<Vec<FilteredResponse>> {
        let regex = self
            .cache
            .try_get_or_insert(pattern.to_string(), || Regex::new(pattern))?;

        let matches = regex.find_iter(text);

        let mut filtered_responses = Vec::new();
        for m in matches {
            let begin = m.start();
            let length = m.len();

            let s = m.as_str();

            let filtered_response = FilteredResponse {
                begin: begin as u64,
                length: length as u64,
                bytes: s.as_bytes().to_vec(),
            };

            filtered_responses.push(filtered_response);
        }

        Ok(filtered_responses)
    }
}
