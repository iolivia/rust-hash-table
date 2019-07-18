// Hash table trait, allows getting and setting.
// For now this is not a generic HashTable.
trait HashTable {
    fn get(&self, key: &String) -> Option<&String>;
    fn set(&mut self, key: String, value: String) -> Result<(), String>;
}

struct LinearProbingHashTable {
    store: Vec<(String, String)>,
}

impl LinearProbingHashTable {
    fn new() -> Self {
        Self{store: Vec::new()}
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
}
