use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

// Hash table trait, allows getting and setting.
// For now this is not a generic HashTable.
trait HashTable {
    fn get(&self, key: &String) -> Option<&String>;
    fn set(&mut self, key: String, value: String) -> Result<(), String>;
    fn size(&self) -> usize;
}

struct NoCollisionsHashTable {
    capacity: usize,
    used_capacity: usize,
    store: Vec<(String, String)>,
}

impl NoCollisionsHashTable {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            used_capacity: 0,
            store: Vec::with_capacity(capacity),
        }
    }

    fn hash(&self, key: &String) -> usize {
        // Create the hasher
        let mut hasher = DefaultHasher::new();

        // Hash our key
        hasher.write(key.as_bytes());

        // Map to our capacity space and return
        let hash_index = hasher.finish();
        let index = (hash_index % (self.capacity as u64)) as usize;

        println!("hash_index {}, capacity {}, index {}", hash_index, self.capacity, index);

        index
    }
}

impl HashTable for NoCollisionsHashTable {

    fn get(&self, key: &String) -> Option<&String> {
        // Map the key to an index
        let index = self.hash(key);

        // Check if it's there
        // TODO this should do linear probing eventually
        match self.store.get(index) {
            Some(key_value) => Some(&key_value.1),
            None => None,
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), String> {

        // Check if we are at capacity and we cannot insert any more items.
        if self.used_capacity == self.capacity {
            return Err("at capacity".to_string());
        }

        // If we still have space, we hash and insert. For now we fail if
        // we have collisions.
        let index = self.hash(&key);

        match self.store.get(index) {
            Some(_) => Err("collision".to_string()),
            None => {
                self.store.insert(index, (key, value));
                self.used_capacity += 1;

                Ok(())
            }
        }
    }

    fn size(&self) -> usize {
        self.used_capacity
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_not_found() {
        let table = NoCollisionsHashTable::new(10);
        let item = table.get(&"hello".to_string());

        assert!(item.is_none());
    }

    #[test]
    fn test_get_found() {
        let mut table = NoCollisionsHashTable::new(10);
        let key = "hello".to_string();
        let value = "there".to_string();
        table.set(key.clone(), value.clone()).expect("set failed");

        let item = table.get(&key);

        assert!(item.is_some());
        assert_eq!(item.unwrap(), &value);
    }

    fn test_set_multiple() {
        let mut table = NoCollisionsHashTable::new(20);
        let values = [
            "hello", 
            "aaaaaaaaa",
            // "again"
        ];

        for value in values.iter() {
            let value = value.to_string();
            table.set(value.clone(), value.clone()).expect("set failed");
        }
    
        assert_eq!(table.size(), values.len());
    }
}