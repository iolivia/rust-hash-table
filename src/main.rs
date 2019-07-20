use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[derive(Debug, Clone)]
struct HashItem {
    key: String, 
    value: String
}

// Hash table trait, allows getting and setting.
// For now this is not a generic HashTable.
trait HashTable {
    fn get(&self, key: &String) -> Option<&HashItem>;
    fn set(&mut self, key: String, value: String) -> Result<(), String>;
    fn remove(&mut self, key: &String);
    fn size(&self) -> usize;
}

struct NoCollisionsHashTable {
    capacity: usize,
    used_capacity: usize,
    store: Vec<Option<HashItem>>,
}

impl NoCollisionsHashTable {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            used_capacity: 0,
            store: vec![None; capacity],
        }
    }

    fn hash(&self, key: &String) -> usize {
        // Create the hasher
        let mut hasher = DefaultHasher::new();

        // Hash our key
        println!("{} as bytes -> {:?}", key, key.as_bytes());

        hasher.write(key.as_bytes());

        // Map to our capacity space and return
        let hash_index = hasher.finish();
        let index = (hash_index % (self.capacity as u64 - 1)) as usize;

        println!("hash_index {}, capacity {}, index {}", hash_index, self.capacity, index);

        index
    }
}

impl HashTable for NoCollisionsHashTable {

    fn get(&self, key: &String) -> Option<&HashItem> {
        // Map the key to an index
        let index = self.hash(key);

        // Check if it's there
        // TODO this should do linear probing eventually
        match self.store.get(index) {
            Some(hash_item) => hash_item.as_ref(),
            None => None,
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), String> {

        // Check if we are at capacity and we cannot insert any more items.
        if self.used_capacity == self.capacity {
            return Err("at capacity".to_string());
        }

        println!("{:?}", self.store);

        // If we still have space, we hash and insert. For now we fail if
        // we have collisions.
        let index = self.hash(&key);

        match self.store.get(index) {
            Some(Some(_)) => Err("collision".to_string()),
            Some(None) => {
              self.store.insert(index, Some(HashItem{key, value,}));
                self.used_capacity += 1;

                Ok(())  
            }
            None => panic!("invalid range")
        }
    }

    fn remove(&mut self, key: &String) {
        let index = self.hash(&key);
        self.store[index] = None;
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
    fn get_not_found() {
        let table = NoCollisionsHashTable::new(10);
        let item = table.get(&"hello".to_string());

        assert!(item.is_none());
    }

    #[test]
    fn set_get() {
        let mut table = NoCollisionsHashTable::new(10);
        let key = "hello".to_string();
        let value = "there".to_string();
        table.set(key.clone(), value.clone()).expect("set failed");

        let item = table.get(&key);

        assert!(item.is_some());
        assert_eq!(item.unwrap().key, key);
        assert_eq!(item.unwrap().value, value);
    }

    #[test]
    fn set_multiple() {
        let mut table = NoCollisionsHashTable::new(20);
        let values = [
            "hello", 
            "aaaaaaaaa",
            "again"
        ];

        for value in values.iter() {
            let value = value.to_string();
            table.set(value.clone(), value.clone()).expect("set failed");
        }
    
        assert_eq!(table.size(), values.len());
    }

    #[test]
    fn set_remove_get() {
        let mut table = NoCollisionsHashTable::new(10);
        let key = "hello".to_string();
        let value = "there".to_string();

        // Set
        table.set(key.clone(), value.clone()).expect("set failed");

        // Remove
        table.remove(&key);

        let item = table.get(&key);

        assert!(item.is_none());
    }
}