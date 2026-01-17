//! Element property caching for frequently accessed elements
//!
//! This module provides caching for element properties to reduce
//! redundant RPC calls when accessing the same element multiple times.
//!
//! Performance targets:
//! - Cache hit for properties: <1us
//! - TTL-based invalidation for stale data

use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Default cache capacity for elements
pub const DEFAULT_ELEMENT_CACHE_CAPACITY: usize = 500;

/// Default TTL for cached element properties (10 seconds)
/// UI elements can change state, so we use a shorter TTL
pub const DEFAULT_ELEMENT_TTL: Duration = Duration::from_secs(10);

/// Cached element properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementProperties {
    /// Element hash code (ID)
    pub hash_code: i64,
    /// Text content
    pub text: Option<String>,
    /// Name
    pub name: Option<String>,
    /// Visibility state
    pub visible: bool,
    /// Enabled state
    pub enabled: bool,
    /// Focused state
    pub focused: bool,
    /// Bounds (x, y, width, height)
    pub bounds: (i32, i32, i32, i32),
    /// Additional properties
    pub extra: HashMap<String, serde_json::Value>,
}

impl ElementProperties {
    /// Create new element properties
    pub fn new(hash_code: i64) -> Self {
        Self {
            hash_code,
            text: None,
            name: None,
            visible: true,
            enabled: true,
            focused: false,
            bounds: (0, 0, 0, 0),
            extra: HashMap::new(),
        }
    }

    /// Create from JSON value
    pub fn from_json(json: &serde_json::Value) -> Option<Self> {
        let hash_code = json.get("hashCode")
            .or_else(|| json.get("id"))
            .and_then(|v| v.as_i64())?;

        Some(Self {
            hash_code,
            text: json.get("text").and_then(|v| v.as_str()).map(String::from),
            name: json.get("name").and_then(|v| v.as_str()).map(String::from),
            visible: json.get("visible").and_then(|v| v.as_bool()).unwrap_or(true),
            enabled: json.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
            focused: json.get("focused").and_then(|v| v.as_bool()).unwrap_or(false),
            bounds: (
                json.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                json.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                json.get("width").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                json.get("height").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            ),
            extra: json.get("properties")
                .and_then(|v| v.as_object())
                .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
        })
    }

    /// Get a specific property
    pub fn get_property(&self, name: &str) -> Option<&serde_json::Value> {
        self.extra.get(name)
    }

    /// Set a property
    pub fn set_property(&mut self, name: &str, value: serde_json::Value) {
        self.extra.insert(name.to_string(), value);
    }
}

/// Cached entry with timestamp
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

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct ElementCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub invalidations: u64,
    pub size: usize,
}

impl ElementCacheStats {
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { 0.0 } else { self.hits as f64 / total as f64 }
    }
}

/// Thread-safe element property cache
pub struct ElementCache {
    /// Cache mapping element hash codes to properties
    cache: RwLock<LruCache<i64, CachedEntry<ElementProperties>>>,
    /// Time-to-live for entries
    ttl: Duration,
    /// Statistics
    stats: RwLock<ElementCacheStats>,
}

impl ElementCache {
    /// Create a new element cache with default settings
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_ELEMENT_CACHE_CAPACITY)
    }

    /// Create with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1).unwrap());
        Self {
            cache: RwLock::new(LruCache::new(cap)),
            ttl: DEFAULT_ELEMENT_TTL,
            stats: RwLock::new(ElementCacheStats::default()),
        }
    }

    /// Create with capacity and TTL
    pub fn with_capacity_and_ttl(capacity: usize, ttl: Duration) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1).unwrap());
        Self {
            cache: RwLock::new(LruCache::new(cap)),
            ttl,
            stats: RwLock::new(ElementCacheStats::default()),
        }
    }

    /// Get element properties from cache
    pub fn get(&self, hash_code: i64) -> Option<ElementProperties> {
        let cache = self.cache.read().unwrap();
        if let Some(entry) = cache.peek(&hash_code) {
            if !entry.is_expired(self.ttl) {
                self.stats.write().unwrap().hits += 1;
                return Some(entry.value.clone());
            }
        }
        self.stats.write().unwrap().misses += 1;
        None
    }

    /// Store element properties in cache
    pub fn put(&self, properties: ElementProperties) {
        let hash_code = properties.hash_code;
        let mut cache = self.cache.write().unwrap();
        cache.put(hash_code, CachedEntry::new(properties));
        self.stats.write().unwrap().size = cache.len();
    }

    /// Update specific properties without full replacement
    pub fn update<F>(&self, hash_code: i64, update_fn: F) -> bool
    where
        F: FnOnce(&mut ElementProperties),
    {
        let mut cache = self.cache.write().unwrap();
        if let Some(entry) = cache.get_mut(&hash_code) {
            if !entry.is_expired(self.ttl) {
                update_fn(&mut entry.value);
                entry.created_at = Instant::now(); // Refresh timestamp
                return true;
            }
        }
        false
    }

    /// Invalidate a specific element
    pub fn invalidate(&self, hash_code: i64) {
        let mut cache = self.cache.write().unwrap();
        if cache.pop(&hash_code).is_some() {
            let mut stats = self.stats.write().unwrap();
            stats.invalidations += 1;
            stats.size = cache.len();
        }
    }

    /// Invalidate multiple elements
    pub fn invalidate_many(&self, hash_codes: &[i64]) {
        let mut cache = self.cache.write().unwrap();
        let mut invalidated = 0;
        for &hash_code in hash_codes {
            if cache.pop(&hash_code).is_some() {
                invalidated += 1;
            }
        }
        let mut stats = self.stats.write().unwrap();
        stats.invalidations += invalidated;
        stats.size = cache.len();
    }

    /// Clear the entire cache
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
        self.stats.write().unwrap().size = 0;
    }

    /// Get cache statistics
    pub fn stats(&self) -> ElementCacheStats {
        self.stats.read().unwrap().clone()
    }

    /// Get current cache size
    pub fn len(&self) -> usize {
        self.cache.read().unwrap().len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.read().unwrap().is_empty()
    }
}

impl Default for ElementCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Global element cache
static GLOBAL_ELEMENT_CACHE: once_cell::sync::Lazy<ElementCache> =
    once_cell::sync::Lazy::new(ElementCache::new);

/// Get the global element cache
pub fn global_element_cache() -> &'static ElementCache {
    &GLOBAL_ELEMENT_CACHE
}

/// Element finder cache for locator-to-element-id mappings
/// This caches the result of find operations
pub struct FinderCache {
    /// Maps locator strings to element hash codes
    cache: RwLock<LruCache<String, CachedEntry<Vec<i64>>>>,
    ttl: Duration,
    stats: RwLock<ElementCacheStats>,
}

impl FinderCache {
    /// Create a new finder cache
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::new(1).unwrap());
        Self {
            cache: RwLock::new(LruCache::new(cap)),
            // Shorter TTL for find results since UI structure can change
            ttl: Duration::from_secs(5),
            stats: RwLock::new(ElementCacheStats::default()),
        }
    }

    /// Get cached find results for a locator
    pub fn get(&self, locator: &str) -> Option<Vec<i64>> {
        let cache = self.cache.read().unwrap();
        if let Some(entry) = cache.peek(locator) {
            if !entry.is_expired(self.ttl) {
                self.stats.write().unwrap().hits += 1;
                return Some(entry.value.clone());
            }
        }
        self.stats.write().unwrap().misses += 1;
        None
    }

    /// Cache find results for a locator
    pub fn put(&self, locator: &str, element_ids: Vec<i64>) {
        let mut cache = self.cache.write().unwrap();
        cache.put(locator.to_string(), CachedEntry::new(element_ids));
        self.stats.write().unwrap().size = cache.len();
    }

    /// Invalidate all find cache entries
    /// Called when UI structure might have changed (after click, type, etc.)
    pub fn invalidate_all(&self) {
        let mut cache = self.cache.write().unwrap();
        let prev_size = cache.len();
        cache.clear();
        let mut stats = self.stats.write().unwrap();
        stats.invalidations += prev_size as u64;
        stats.size = 0;
    }

    /// Get statistics
    pub fn stats(&self) -> ElementCacheStats {
        self.stats.read().unwrap().clone()
    }
}

impl Default for FinderCache {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_properties_from_json() {
        let json = serde_json::json!({
            "hashCode": 12345,
            "text": "Hello",
            "name": "testElement",
            "visible": true,
            "enabled": true,
            "x": 100,
            "y": 200,
            "width": 80,
            "height": 25,
        });

        let props = ElementProperties::from_json(&json).unwrap();
        assert_eq!(props.hash_code, 12345);
        assert_eq!(props.text, Some("Hello".to_string()));
        assert_eq!(props.name, Some("testElement".to_string()));
        assert_eq!(props.bounds, (100, 200, 80, 25));
    }

    #[test]
    fn test_element_cache_basic() {
        let cache = ElementCache::with_capacity(10);

        let mut props = ElementProperties::new(12345);
        props.text = Some("Hello".to_string());
        cache.put(props);

        let cached = cache.get(12345).unwrap();
        assert_eq!(cached.text, Some("Hello".to_string()));

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
    }

    #[test]
    fn test_element_cache_update() {
        let cache = ElementCache::with_capacity(10);

        let mut props = ElementProperties::new(12345);
        props.text = Some("Hello".to_string());
        cache.put(props);

        let updated = cache.update(12345, |p| {
            p.text = Some("World".to_string());
        });
        assert!(updated);

        let cached = cache.get(12345).unwrap();
        assert_eq!(cached.text, Some("World".to_string()));
    }

    #[test]
    fn test_element_cache_invalidation() {
        let cache = ElementCache::with_capacity(10);

        cache.put(ElementProperties::new(12345));
        assert_eq!(cache.len(), 1);

        cache.invalidate(12345);
        assert_eq!(cache.len(), 0);
        assert!(cache.get(12345).is_none());
    }

    #[test]
    fn test_finder_cache() {
        let cache = FinderCache::new(10);

        cache.put("name:button", vec![123, 456]);

        let result = cache.get("name:button").unwrap();
        assert_eq!(result, vec![123, 456]);

        cache.invalidate_all();
        assert!(cache.get("name:button").is_none());
    }
}
