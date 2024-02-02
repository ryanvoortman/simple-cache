use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

// Define the cache struct with a generic type that must implement the eq and hash traits.
pub struct Cache<K, V> where K: Eq + Hash {
    // the underlying storage for cache
    storage: HashMap<K,V>,
}

impl <K, V> Cache<K, V> where K: Eq + Hash + Clone {
    // create new empty cache
    pub fn new() -> Self {
        Cache {
            storage: HashMap::new(),
        }
    }

    // insert a key value pair into the cache
    pub fn set(&mut self, key: K, value: V) {
        self.storage.insert(key, value);
    }

    // retrieve a value from the cache by key, returning an option
    pub fn get(&self, key: &K) -> Option<&V> {
        self.storage.get(key)
    }
}

pub struct LRUCache<K, V> where K: Eq + Hash + Clone, {
    capacity: usize,
    storage: HashMap<K, V>,
    usage_order: VecDeque<K>,
}

impl<K, V> LRUCache<K, V> where K: Eq + Hash + Clone, {

    pub fn new(capacity: usize) -> Self<> {
        assert!(capacity > 0, "cache capacity must be greater than 0");
        LRUCache {
            capacity,
            storage: HashMap::new(),
            usage_order: VecDeque::new(),
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        // insert or update the value for the key
        self.storage.insert(key.clone(), value);
        // move this key to the front of the usage order to mark it as recently used
        self.update_usage(&key);
        // if cache exceeds its capacity, remove the least recently used item.
        if self.storage.len() > self.capacity {
            if let Some(least_recently_used) = self.usage_order.pop_back() {
                self.storage.remove(&least_recently_used);
            }
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.storage.contains_key(key) {
            // move this key to the front of the usage order
            self.update_usage(key);
            self.storage.get(key)
        } else {
            None
        }
    }

    pub fn update_usage(&mut self, key: &K) {
        // remove key if it already exists in usage order
        self.usage_order.retain(|existing_key| existing_key != key);
            // insert the key at the front to mark it as recently used
            self.usage_order.push_front(key.clone());
    }

}

fn main() {
    // Example usage of cache
    let mut cache = Cache::new();

    // Insert some key value pairs
    cache.set("key1", "value1");
    cache.set("key2", "value2");

    // Retrieve and print value
    if let Some(value) = cache.get(&"key1") {
        println!("Found value: {}", value);
    } else {
        println!("Value not found");
    }

    // try to retrieve a value that doesn't exist
    match cache.get(&"key3") {
        Some(value) => println!("Found value: {}", value),
        None => println!("Value not found"),
    }

    // example usage of LRUCache
    let mut lrucache = LRUCache::new(2);
    lrucache.set("key1", "value1");
    lrucache.set("key2", "value2");
    println!("Retrieved: {:?} ", lrucache.get(&"key1")); // should update key1s position
    lrucache.set("key3", "value3"); // this should evict key 2
    // trying to retrieve key2 should yield none since it was evicted
    match lrucache.get(&"key2") {
        Some(value) => println!("Retrieved: {:?}", value),
        None => println!("key2 was evicted"),

    }
}