//! LRU cache for parsed locators and normalized results
//!
//! This module provides caching for locator parsing and normalization
//! to avoid redundant parsing of frequently used locators.
//!
//! Performance targets:
//! - Cache hit: <1us
//! - Cache miss with parsing: <100us

use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use super::unified::{UnifiedLocator, NormalizedLocator, LocatorParseError};
use crate::core::backend::ToolkitType;

/// Default cache capacity for locators
pub const DEFAULT_CACHE_CAPACITY: usize = 1000;

/// Default TTL for cached entries (5 minutes)
pub const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(300);

/// Cache statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of entries evicted
    pub evictions: u64,
    /// Current cache size
    pub size: usize,
}

impl CacheStats {
    /// Get the cache hit ratio
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.evictions = 0;
    }
}

/// Cached entry with timestamp for TTL
#[derive(Clone)]
struct CachedEntry<T> {
    value: T,
    created_at: Instant,
}

impl<T> CachedEntry<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            created_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }
}

/// Thread-safe LRU cache for parsed locators
pub struct LocatorCache {
    /// Cache for UnifiedLocator parsing results
    cache: RwLock<LruCache<String, CachedEntry<UnifiedLocator>>>,
    /// Time-to-live for cache entries
    ttl: Duration,
    /// Statistics
    stats: RwLock<CacheStats>,
}

impl LocatorCache {
    /// Create a new locator cache with default settings
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CACHE_CAPACITY)
    }

    /// Create a new locator cache with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1).unwrap());
        Self {
            cache: RwLock::new(LruCache::new(cap)),
            ttl: DEFAULT_CACHE_TTL,
            stats: RwLock::new(CacheStats::default()),
        }
    }

    /// Create a new locator cache with capacity and TTL
    pub fn with_capacity_and_ttl(capacity: usize, ttl: Duration) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1).unwrap());
        Self {
            cache: RwLock::new(LruCache::new(cap)),
            ttl,
            stats: RwLock::new(CacheStats::default()),
        }
    }

    /// Parse a locator, using cache if available
    pub fn parse(&self, locator: &str) -> Result<UnifiedLocator, LocatorParseError> {
        let key = locator.to_string();

        // Try read lock first for cache hit
        {
            let cache = self.cache.read().unwrap();
            if let Some(entry) = cache.peek(&key) {
                if !entry.is_expired(self.ttl) {
                    self.record_hit();
                    return Ok(entry.value.clone());
                }
            }
        }

        // Cache miss or expired - need to parse
        self.record_miss();
        let parsed = UnifiedLocator::parse(locator)?;

        // Store in cache
        {
            let mut cache = self.cache.write().unwrap();
            let was_full = cache.len() >= cache.cap().get();
            cache.put(key, CachedEntry::new(parsed.clone()));
            if was_full {
                self.record_eviction();
            }
            self.update_size(cache.len());
        }

        Ok(parsed)
    }

    /// Parse and normalize a locator for a specific toolkit
    pub fn parse_and_normalize(
        &self,
        locator: &str,
        toolkit: ToolkitType,
    ) -> Result<NormalizedLocator, LocatorParseError> {
        let parsed = self.parse(locator)?;
        Ok(parsed.normalize_for_toolkit(toolkit))
    }

    /// Get a cached locator without parsing if not found
    pub fn get(&self, locator: &str) -> Option<UnifiedLocator> {
        let cache = self.cache.read().unwrap();
        cache.peek(locator).and_then(|entry| {
            if entry.is_expired(self.ttl) {
                None
            } else {
                Some(entry.value.clone())
            }
        })
    }

    /// Check if a locator is cached
    pub fn contains(&self, locator: &str) -> bool {
        let cache = self.cache.read().unwrap();
        cache.peek(locator)
            .map(|entry| !entry.is_expired(self.ttl))
            .unwrap_or(false)
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
        self.update_size(0);
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.stats.read().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        self.stats.write().unwrap().reset();
    }

    /// Get current cache size
    pub fn len(&self) -> usize {
        self.cache.read().unwrap().len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.read().unwrap().is_empty()
    }

    /// Get cache capacity
    pub fn capacity(&self) -> usize {
        self.cache.read().unwrap().cap().get()
    }

    // Internal stat tracking methods
    fn record_hit(&self) {
        self.stats.write().unwrap().hits += 1;
    }

    fn record_miss(&self) {
        self.stats.write().unwrap().misses += 1;
    }

    fn record_eviction(&self) {
        self.stats.write().unwrap().evictions += 1;
    }

    fn update_size(&self, size: usize) {
        self.stats.write().unwrap().size = size;
    }
}

impl Default for LocatorCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Global locator cache singleton
static GLOBAL_CACHE: once_cell::sync::Lazy<Arc<LocatorCache>> =
    once_cell::sync::Lazy::new(|| Arc::new(LocatorCache::new()));

/// Get the global locator cache
pub fn global_cache() -> Arc<LocatorCache> {
    Arc::clone(&GLOBAL_CACHE)
}

/// Parse a locator using the global cache
pub fn cached_parse(locator: &str) -> Result<UnifiedLocator, LocatorParseError> {
    global_cache().parse(locator)
}

/// Parse and normalize a locator using the global cache
pub fn cached_parse_and_normalize(
    locator: &str,
    toolkit: ToolkitType,
) -> Result<NormalizedLocator, LocatorParseError> {
    global_cache().parse_and_normalize(locator, toolkit)
}

/// Normalized locator cache with toolkit-specific caching
pub struct NormalizationCache {
    /// Cache per toolkit type
    swing_cache: RwLock<LruCache<String, CachedEntry<NormalizedLocator>>>,
    swt_cache: RwLock<LruCache<String, CachedEntry<NormalizedLocator>>>,
    rcp_cache: RwLock<LruCache<String, CachedEntry<NormalizedLocator>>>,
    ttl: Duration,
    stats: RwLock<CacheStats>,
}

impl NormalizationCache {
    /// Create a new normalization cache
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1).unwrap());
        Self {
            swing_cache: RwLock::new(LruCache::new(cap)),
            swt_cache: RwLock::new(LruCache::new(cap)),
            rcp_cache: RwLock::new(LruCache::new(cap)),
            ttl: DEFAULT_CACHE_TTL,
            stats: RwLock::new(CacheStats::default()),
        }
    }

    /// Get the cache for a specific toolkit
    fn get_cache(&self, toolkit: ToolkitType) -> &RwLock<LruCache<String, CachedEntry<NormalizedLocator>>> {
        match toolkit {
            ToolkitType::Swing => &self.swing_cache,
            ToolkitType::Swt => &self.swt_cache,
            ToolkitType::Rcp => &self.rcp_cache,
        }
    }

    /// Normalize a locator, using cache if available
    pub fn normalize(
        &self,
        locator: &UnifiedLocator,
        toolkit: ToolkitType,
    ) -> NormalizedLocator {
        let key = locator.original.clone();
        let cache = self.get_cache(toolkit);

        // Try cache hit
        {
            let cache_read = cache.read().unwrap();
            if let Some(entry) = cache_read.peek(&key) {
                if !entry.is_expired(self.ttl) {
                    self.stats.write().unwrap().hits += 1;
                    return entry.value.clone();
                }
            }
        }

        // Cache miss
        self.stats.write().unwrap().misses += 1;
        let normalized = locator.normalize_for_toolkit(toolkit);

        // Store in cache
        {
            let mut cache_write = cache.write().unwrap();
            cache_write.put(key, CachedEntry::new(normalized.clone()));
        }

        normalized
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.stats.read().unwrap().clone()
    }

    /// Clear all caches
    pub fn clear(&self) {
        self.swing_cache.write().unwrap().clear();
        self.swt_cache.write().unwrap().clear();
        self.rcp_cache.write().unwrap().clear();
    }
}

impl Default for NormalizationCache {
    fn default() -> Self {
        Self::new(DEFAULT_CACHE_CAPACITY)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locator_cache_basic() {
        let cache = LocatorCache::with_capacity(10);

        // First parse should miss
        let result = cache.parse("name:testButton").unwrap();
        assert_eq!(result.value, "testButton");

        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 0);

        // Second parse should hit
        let result2 = cache.parse("name:testButton").unwrap();
        assert_eq!(result2.value, "testButton");

        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 1);
    }

    #[test]
    fn test_locator_cache_eviction() {
        let cache = LocatorCache::with_capacity(2);

        cache.parse("name:button1").unwrap();
        cache.parse("name:button2").unwrap();
        cache.parse("name:button3").unwrap(); // Should evict button1

        let stats = cache.stats();
        assert_eq!(stats.evictions, 1);
        assert_eq!(cache.len(), 2);

        // button1 should not be in cache
        assert!(!cache.contains("name:button1"));
        assert!(cache.contains("name:button2"));
        assert!(cache.contains("name:button3"));
    }

    #[test]
    fn test_locator_cache_hit_ratio() {
        let cache = LocatorCache::with_capacity(10);

        // 1 miss
        cache.parse("name:button").unwrap();
        // 3 hits
        cache.parse("name:button").unwrap();
        cache.parse("name:button").unwrap();
        cache.parse("name:button").unwrap();

        let stats = cache.stats();
        assert_eq!(stats.hit_ratio(), 0.75); // 3/4
    }

    #[test]
    fn test_locator_cache_clear() {
        let cache = LocatorCache::with_capacity(10);

        cache.parse("name:button1").unwrap();
        cache.parse("name:button2").unwrap();
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert!(cache.is_empty());
        assert!(!cache.contains("name:button1"));
    }

    #[test]
    fn test_cached_parse() {
        let result = cached_parse("text:Click Me").unwrap();
        assert_eq!(result.value, "Click Me");

        // Second call should use cache
        let result2 = cached_parse("text:Click Me").unwrap();
        assert_eq!(result2.value, result.value);
    }

    #[test]
    fn test_normalization_cache() {
        let cache = NormalizationCache::new(10);
        let locator = UnifiedLocator::class("Button");

        // Normalize for Swing
        let swing_norm = cache.normalize(&locator, ToolkitType::Swing);
        assert_eq!(swing_norm.value, "JButton");

        // Normalize for SWT
        let swt_norm = cache.normalize(&locator, ToolkitType::Swt);
        assert_eq!(swt_norm.value, "Button");

        // Check stats
        let stats = cache.stats();
        assert_eq!(stats.misses, 2);

        // Cache hits
        cache.normalize(&locator, ToolkitType::Swing);
        cache.normalize(&locator, ToolkitType::Swt);

        let stats = cache.stats();
        assert_eq!(stats.hits, 2);
    }
}
