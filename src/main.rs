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
    fn insert(&mut self, key: String, value: String) -> Result<(), String>;
    fn remove(&mut self, key: &String);
    fn size(&self) -> usize;
}

struct NoCollisionsHashTable {
    capacity: usize,
    size: usize,
    store: Vec<Option<Vec<HashItem>>>,
}

impl NoCollisionsHashTable {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            size: 0,
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
        match self.store.get(index) {
            Some(Some(hash_vec)) => {
                // find the key in the vec
                hash_vec.iter().find(|hash_item| hash_item.key == *key)
            }
            _ => None,
        }
    }

    fn insert(&mut self, key: String, value: String) -> Result<(), String> {

        // Check if we are at capacity and we cannot insert any more items.
        if self.size == self.capacity {
            return Err("at capacity".to_string());
        }

        println!("{:?}", self.store);

        // If we still have space, we hash and insert. For now we fail if
        // we have collisions.
        let index = self.hash(&key);

        match self.store.get_mut(index) {
            Some(Some(hash_vec)) => {

                let duplicate_key = hash_vec.iter().find(|hash_item| hash_item.key == *key);

                if duplicate_key.is_some() {
                    Err("duplicate key".to_string())
                } else {
                    // Insert it
                    hash_vec.push(HashItem{key, value});
                    self.size += 1;

                    Ok(())  
                }
            }
            Some(None) => {
                let mut hash_vec = Vec::new();
                hash_vec.push(HashItem{key, value});

                self.store.insert(index, Some(hash_vec));
                self.size += 1;

                Ok(())  
            }
            None => panic!("invalid range")
        }
    }

    fn remove(&mut self, key: &String) {
        let index = self.hash(&key);
        self.store[index] = None;
        self.size -= 1;
    }

    fn size(&self) -> usize {
        self.size
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
    fn insert_get() {
        let mut table = NoCollisionsHashTable::new(10);
        let key = "hello".to_string();
        let value = "there".to_string();
        table.insert(key.clone(), value.clone()).expect("insert failed");

        let item = table.get(&key);

        assert!(item.is_some());
        assert_eq!(item.unwrap().key, key);
        assert_eq!(item.unwrap().value, value);
    }

    #[test]
    fn insert_multiple() {
        let mut table = NoCollisionsHashTable::new(20);
        let values = [
            "hello", 
            "aaaaaaaaa",
            "again"
        ];

        for value in values.iter() {
            let value = value.to_string();
            table.insert(value.clone(), value.clone()).expect("insert failed");
        }
    
        assert_eq!(table.size(), values.len());
    }

    #[test]
    fn insert_remove_get() {
        let mut table = NoCollisionsHashTable::new(10);
        let key = "hello".to_string();
        let value = "there".to_string();

        // insert
        table.insert(key.clone(), value.clone()).expect("insert failed");

        // Remove
        table.remove(&key);

        let item = table.get(&key);

        assert!(item.is_none());
    }

    #[test]
    fn insert_decrements_size() {
        let mut table = NoCollisionsHashTable::new(10);
        let key = "hello".to_string();
        let value = "there".to_string();

        // insert
        table.insert(key.clone(), value.clone()).expect("insert failed");
        assert_eq!(table.size(), 1);

        // Remove
        table.remove(&key);
        assert_eq!(table.size(), 0);
    }

    #[test]
    fn insert_same_key_fails() {
        let mut table = NoCollisionsHashTable::new(10);
        let key = "hello".to_string();
        let value = "there".to_string();

        // first insert should succeed
        table.insert(key.clone(), value.clone()).expect("fail");

        // second insert should fail
        table.insert(key.clone(), value.clone()).expect_err("duplicate key");
    }
}