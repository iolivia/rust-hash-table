// Hash table trait, allows getting and setting.
// For now this is not a generic HashTable.
trait HashTable {
    fn get(&self, key: &String) -> &String;
    fn set(&mut self, key: String, value: String) -> Result<(), String>;
}

fn main() {
    println!("Hello, world!");
}
