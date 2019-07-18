use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

// Hash table trait, allows getting and setting.
// For now this is not a generic HashTable.
trait HashTable {
    fn get(&self, key: &String) -> Option<&String>;
    fn set(&mut self, key: String, value: String) -> Result<(), String>;
}

struct LinearProbingHashTable {
    capacity: usize,
    store: Vec<(String, String)>,
}

impl LinearProbingHashTable {
    fn new(capacity: usize) -> Self {
        Self {
            capacity, 
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

impl HashTable for LinearProbingHashTable {

    fn get(&self, key: &String) -> Option<&String> {
        None
    }

    fn set(&mut self, key: String, value: String) -> Result<(), String> {
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let table = LinearProbingHashTable::new(100);

    let keys = [
        "hello".to_string(),
        "bye".to_string()
    ];

    for key in keys.iter() {
        println!("{}", table.hash(&key));
    }
}
