use std::time::Instant;
use std::{collections::HashMap, time::Duration};

fn main() {
    println!("Hello world");
}

struct CacheMeOutSide<T> {
    pub store: HashMap<String, (T, Option<Instant>)>,
}

impl<T> CacheMeOutSide<T> {
    pub fn new() -> Self {
        return CacheMeOutSide {
            store: HashMap::new(),
        };
    }

    /// Returns a value stored for a given key if found and not expired.
    pub fn get(&self, key: &String) -> Option<&T> {
        if let Some((value, exp)) = self.store.get(key) {
            return exp.map_or(Some(value), |exp| {
                if exp > Instant::now() {
                    Some(value)
                } else {
                    None
                }
            });
        }

        return None;
    }

    /// Stores a given key-value pair, which will be invalidated after the given duration
    pub fn insert(&mut self, key: String, value: T, duration: Option<Duration>) {
        let expiration = duration.map(|duration| Instant::now() + duration);
        self.store.insert(key, (value, expiration));
    }

    pub fn invalidate(&mut self, key: &String) {
        self.store.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn test_cache_insert_get_and_invalidate() {
        let mut cache = CacheMeOutSide::new();

        cache.insert("hello".to_string(), "world", None);
        assert_eq!(cache.get(&"hello".to_string()), Some(&"world"));

        cache.invalidate(&"hello".to_string());
        assert_eq!(cache.get(&"hello".to_string()), None);
    }

    #[test]
    fn test_expiration() {
        let mut cache = CacheMeOutSide::new();
        cache.insert("my-key".to_string(), 100, Some(Duration::from_millis(100)));

        // value can be retrieved before expiration
        assert_eq!(cache.get(&"my-key".to_string()), Some(&100));

        // wait for expiration
        sleep(Duration::from_millis(200));

        // should return None after expiration
        assert_eq!(cache.get(&"my-key".to_string()), None);
    }
}
