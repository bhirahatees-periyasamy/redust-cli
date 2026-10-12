use std::collections::HashMap;

pub trait Operations {
    fn insert(&mut self, key: String, value: Vec<u8>) -> Option<Vec<u8>>;
    fn get(&self, key: &str) -> Option<&Vec<u8>>;
    fn delete(&mut self, key: &str) -> bool;
}

#[derive(Default)]
pub struct Database {
    records: HashMap<String, Vec<u8>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }
}

impl Operations for Database {
    fn insert(&mut self, key: String, value: Vec<u8>) -> Option<Vec<u8>> {
        self.records.insert(key, value)
    }

    fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.records.get(key)
    }

    fn delete(&mut self, key: &str) -> bool {
        self.records.remove(key).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_on_empty_db_is_none() {
        let db = Database::new();
        assert!(db.get("nope").is_none());
    }

    #[test]
    fn insert_then_get() {
        let mut db = Database::new();
        db.insert("k".to_string(), b"v".to_vec());
        assert_eq!(db.get("k"), Some(&b"v".to_vec()));
    }

    #[test]
    fn first_insert_returns_none() {
        let mut db = Database::new();
        assert_eq!(db.insert("k".to_string(), vec![1]), None);
    }

    #[test]
    fn insert_existing_key_returns_old_value_and_replaces() {
        let mut db = Database::new();
        db.insert("k".to_string(), vec![1]);
        let old = db.insert("k".to_string(), vec![2]);
        assert_eq!(old, Some(vec![1]));
        assert_eq!(db.get("k"), Some(&vec![2]));
    }

    #[test]
    fn keys_are_independent() {
        let mut db = Database::new();
        db.insert("a".to_string(), vec![1]);
        db.insert("b".to_string(), vec![2]);
        assert_eq!(db.get("a"), Some(&vec![1]));
        assert_eq!(db.get("b"), Some(&vec![2]));
    }

    #[test]
    fn keys_are_case_sensitive() {
        let mut db = Database::new();
        db.insert("Key".to_string(), vec![1]);
        assert!(db.get("key").is_none());
    }

    #[test]
    fn delete_existing_key() {
        let mut db = Database::new();
        db.insert("k".to_string(), vec![1]);
        assert!(db.delete("k"));
        assert!(db.get("k").is_none());
    }

    #[test]
    fn delete_missing_key_returns_false() {
        let mut db = Database::new();
        assert!(!db.delete("k"));
    }

    #[test]
    fn delete_only_removes_target() {
        let mut db = Database::new();
        db.insert("a".to_string(), vec![1]);
        db.insert("b".to_string(), vec![2]);
        db.delete("a");
        assert_eq!(db.get("b"), Some(&vec![2]));
    }

    #[test]
    fn empty_key_and_empty_value_are_allowed() {
        let mut db = Database::new();
        db.insert(String::new(), Vec::new());
        assert_eq!(db.get(""), Some(&Vec::new()));
    }
}
