/// Stub mínimo — implementação real com BTreeMap/RBTree na issue #6.
pub struct Database;

impl Database {
    pub fn new() -> Self {
        Database
    }

    pub fn set(&mut self, _key: String, _value: String) -> String {
        "OK".to_string()
    }

    pub fn get(&self, _key: &str) -> String {
        "(nil)".to_string()
    }

    pub fn delete(&mut self, _key: &str) -> String {
        "(nil)".to_string()
    }

    pub fn range(&self, _low: &str, _high: &str) -> String {
        "(empty)".to_string()
    }

    pub fn keys(&self) -> String {
        "(empty)".to_string()
    }

    pub fn min(&self) -> String {
        "(nil)".to_string()
    }

    pub fn max(&self) -> String {
        "(nil)".to_string()
    }

    pub fn save(&self) -> String {
        "(not implemented)".to_string()
    }

    pub fn load(&mut self) -> String {
        "(not implemented)".to_string()
    }
}
