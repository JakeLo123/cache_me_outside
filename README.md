# Prompt

Create an in-memory cache in Rust that stores key-value pairs and automatically evicts entries after a given expiration time. The cache should allow insertion, retrieval, and manual invalidation of entries.
Methods: insert, get, invalidate
get method should automatically invalidate expired entries before retrieval
example of use

```rust
let mut cache = Cache::new();
cache.insert("user_xyz", "session_value", Duration::new(30, 0)); // TTL 30 seconds

if let Some(token) = cache.get("user_xyz") {
    println!("Token: {}", token);
} else {
    println!("Token expired or not found");
}

cache.invalidate("user_xyz"); // manual invalidation
```
