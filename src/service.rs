use crate::codec::{Codec, CodecOperations};
use crate::command::Commands;
use crate::database::{Database, Operations};

#[derive(Default)]
pub struct RedustService {
    db: Database,
    codec: Codec,
}

pub trait ServiceOperations {
    fn set(&mut self, key: String, value: String) -> String;

    fn get(&self, key: &str) -> String;

    fn delete(&mut self, key: &str) -> bool;

    fn execute(&mut self, cmd: String) -> String;

    fn find_operation(&mut self, partial_cmd: &str) -> Commands;

    fn parse_command(cmd: &str) -> Option<(&str, &str, Option<&str>)>;
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

    fn get(&self, key: &str) -> String {
        let value = self.db.get(key).map(|bytes| self.codec.decode(bytes));

        match value {
            Some(val) => val,
            None => String::from("None"),
        }
    }

    fn delete(&mut self, key: &str) -> bool {
        self.db.delete(key)
    }

    fn find_operation(&mut self, partial_cmd: &str) -> Commands {
        match partial_cmd.to_uppercase().as_str() {
            "SET" => Commands::SET,
            "GET" => Commands::GET,
            "DELETE" => Commands::DELETE,
            _ => Commands::None,
        }
    }

    fn parse_command(cmd: &str) -> Option<(&str, &str, Option<&str>)> {
        let mut parts = cmd.split_whitespace();

        let command = parts.next()?;
        let key = parts.next()?;
        let value = parts.next();

        Some((command, key, value))
    }

    fn execute(&mut self, cmd: String) -> String {
        let Some((partial_cmd, key, value)) = Self::parse_command(&cmd) else {
            return "Invalid command".to_string();
        };
        let command = self.find_operation(partial_cmd);
        match command {
            Commands::SET => match value {
                Some(value) => self.set(key.to_string(), value.to_string()),
                None => "SET requires a key and value".to_string(),
            },
            Commands::GET => self.get(key),
            Commands::DELETE => {
                if self.delete(key) {
                    return String::from("OK");
                }
                String::from("None")
            }
            Commands::None => "Invalid command".to_string(),
        }
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
        assert_eq!(svc.get("name"), "redust");
    }

    #[test]
    fn get_unknown_key_is_none() {
        let svc = RedustService::new();
        assert_eq!(svc.get("ghost"), "None");
    }

    #[test]
    fn set_overwrites_previous_value() {
        let mut svc = RedustService::new();
        svc.set("k".to_string(), "one".to_string());
        svc.set("k".to_string(), "two".to_string());
        assert_eq!(svc.get("k"), "two");
    }

    #[test]
    fn empty_value_is_stored_not_treated_as_missing() {
        let mut svc = RedustService::new();
        svc.set("k".to_string(), String::new());
        assert_eq!(svc.get("k"), "");
    }

    #[test]
    fn unicode_values_survive_roundtrip() {
        let mut svc = RedustService::new();
        svc.set("greeting".to_string(), "héllo 世界".to_string());
        assert_eq!(svc.get("greeting"), "héllo 世界");
    }

    #[test]
    fn delete_removes_key() {
        let mut svc = RedustService::new();
        svc.set("k".to_string(), "v".to_string());
        assert!(svc.delete("k"));
        assert_eq!(svc.get("k"), "None");
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
        assert_eq!(svc.get("k"), "new");
    }

    #[test]
    fn execute_set_get_delete() {
        let mut svc = RedustService::new();
        assert_eq!(svc.execute("SET k v".to_string()), "OK");
        assert_eq!(svc.execute("get k".to_string()), "v");
        assert_eq!(svc.execute("DELETE k".to_string()), "OK");
        assert_eq!(svc.execute("GET k".to_string()), "None");
        assert_eq!(svc.execute("DELETE k".to_string()), "None");
    }

    #[test]
    fn execute_set_without_value_is_error() {
        let mut svc = RedustService::new();
        assert_eq!(
            svc.execute("SET k".to_string()),
            "SET requires a key and value"
        );
    }

    #[test]
    fn execute_invalid_input() {
        let mut svc = RedustService::new();
        assert_eq!(svc.execute(String::new()), "Invalid command");
        assert_eq!(svc.execute("FOO k v".to_string()), "Invalid command");
    }

    #[test]
    fn find_operation_is_case_insensitive() {
        let mut svc = RedustService::new();
        assert!(matches!(svc.find_operation("set"), Commands::SET));
        assert!(matches!(svc.find_operation("Get"), Commands::GET));
        assert!(matches!(svc.find_operation("dElEtE"), Commands::DELETE));
    }

    #[test]
    fn find_operation_unknown_is_none() {
        let mut svc = RedustService::new();
        assert!(matches!(svc.find_operation("PING"), Commands::None));
        assert!(matches!(svc.find_operation(""), Commands::None));
    }

    #[test]
    fn parse_command_with_value() {
        assert_eq!(
            RedustService::parse_command("SET k v"),
            Some(("SET", "k", Some("v")))
        );
    }

    #[test]
    fn parse_command_without_value() {
        assert_eq!(
            RedustService::parse_command("GET k"),
            Some(("GET", "k", None))
        );
    }

    #[test]
    fn parse_command_missing_key_or_empty_is_none() {
        assert_eq!(RedustService::parse_command("GET"), None);
        assert_eq!(RedustService::parse_command(""), None);
        assert_eq!(RedustService::parse_command("   "), None);
    }

    #[test]
    fn parse_command_tolerates_extra_whitespace() {
        assert_eq!(
            RedustService::parse_command("  SET   k    v  "),
            Some(("SET", "k", Some("v")))
        );
    }

    #[test]
    fn execute_overwrites_via_command() {
        let mut svc = RedustService::new();
        svc.execute("SET k one".to_string());
        svc.execute("SET k two".to_string());
        assert_eq!(svc.execute("GET k".to_string()), "two");
    }

    #[test]
    fn execute_keys_are_case_sensitive_but_commands_are_not() {
        let mut svc = RedustService::new();
        svc.execute("set Key v".to_string());
        assert_eq!(svc.execute("get Key".to_string()), "v");
        assert_eq!(svc.execute("GET key".to_string()), "None");
    }

    #[test]
    fn execute_get_and_delete_without_key_is_invalid() {
        let mut svc = RedustService::new();
        assert_eq!(svc.execute("GET".to_string()), "Invalid command");
        assert_eq!(svc.execute("DELETE".to_string()), "Invalid command");
    }
}
