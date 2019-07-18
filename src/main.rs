use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

// Hash table trait, allows getting and setting.
// For now this is not a generic HashTable.
trait HashTable {
    fn get(&self, key: &String) -> Option<&String>;
    fn set(&mut self, key: String, value: String) -> Result<(), String>;
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
            store: Vec::with_capacity(capacity)
        }
    }

    fn hash(&self, key: &String) -> usize {
        // Create the hasher
        let mut hasher = DefaultHasher::new();

        // Hash our key
        hasher.write(key.as_bytes());

        // Map to our capacity space and return
        (hasher.finish() % (self.capacity as u64)) as usize
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
            None => None
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
                self.used_capacity +=1;

                Ok(())
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut table = NoCollisionsHashTable::new(100);

    table.set("hello".to_string(), "there".to_string());
    table.set("bye".to_string(), "again".to_string());
}
