use crate::codec::{Codec, CodecOperations};
use crate::database::{Database, Operations};

#[derive(Default)]
pub struct RedustService {
    db: Database,
    codec: Codec,
}

pub trait ServiceOperations {
    fn set(&mut self, key: String, value: String) -> String;

    fn get(&self, key: &str) -> Option<String>;

    fn delete(&mut self, key: &str) -> bool;
}

impl RedustService {
    pub fn new() -> Self {
        Self {
            codec: Codec::new(),
            db: Database::new(),
        }
    }
}

impl ServiceOperations for RedustService {
    fn set(&mut self, key: String, value: String) -> String {
        let encoded = self.codec.encode(value);
        self.db.insert(key, encoded);
        String::from("OK")
    }

    fn get(&self, key: &str) -> Option<String> {
        self.db.get(key).map(|bytes| self.codec.decode(bytes))
    }

    fn delete(&mut self, key: &str) -> bool {
        self.db.delete(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_returns_ok() {
        let mut svc = RedustService::new();
        assert_eq!(svc.set("k".to_string(), "v".to_string()), "OK");
    }

    #[test]
    fn get_returns_what_was_set() {
        let mut svc = RedustService::new();
        svc.set("name".to_string(), "redust".to_string());
        assert_eq!(svc.get("name"), Some("redust".to_string()));
    }

    #[test]
    fn get_unknown_key_is_none() {
        let svc = RedustService::new();
        assert_eq!(svc.get("ghost"), None);
    }

    #[test]
    fn set_overwrites_previous_value() {
        let mut svc = RedustService::new();
        svc.set("k".to_string(), "one".to_string());
        svc.set("k".to_string(), "two".to_string());
        assert_eq!(svc.get("k"), Some("two".to_string()));
    }

    #[test]
    fn empty_value_is_stored_not_treated_as_missing() {
        let mut svc = RedustService::new();
        svc.set("k".to_string(), String::new());
        assert_eq!(svc.get("k"), Some(String::new()));
    }

    #[test]
    fn unicode_values_survive_roundtrip() {
        let mut svc = RedustService::new();
        svc.set("greeting".to_string(), "héllo 世界".to_string());
        assert_eq!(svc.get("greeting"), Some("héllo 世界".to_string()));
    }

    #[test]
    fn delete_removes_key() {
        let mut svc = RedustService::new();
        svc.set("k".to_string(), "v".to_string());
        assert!(svc.delete("k"));
        assert_eq!(svc.get("k"), None);
    }

    #[test]
    fn delete_missing_key_is_false() {
        let mut svc = RedustService::new();
        assert!(!svc.delete("k"));
    }

    #[test]
    fn delete_twice_only_succeeds_once() {
        let mut svc = RedustService::new();
        svc.set("k".to_string(), "v".to_string());
        assert!(svc.delete("k"));
        assert!(!svc.delete("k"));
    }

    #[test]
    fn can_set_again_after_delete() {
        let mut svc = RedustService::new();
        svc.set("k".to_string(), "old".to_string());
        svc.delete("k");
        svc.set("k".to_string(), "new".to_string());
        assert_eq!(svc.get("k"), Some("new".to_string()));
    }
}
