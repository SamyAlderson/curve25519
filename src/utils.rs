//! This is a comment explaining the purpose of the module

use std::collections::HashMap;

pub fn get_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map
}