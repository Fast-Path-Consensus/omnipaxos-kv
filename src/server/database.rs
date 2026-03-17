use omnipaxos_kv::common::kv::KVCommand;
use std::collections::HashMap;

pub struct Database {
    db: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }

    /// Read-only lookup (for fast-path re-reads without re-executing writes)
    pub fn get(&self, key: &str) -> Option<Option<String>> {
        Some(self.db.get(key).cloned())
    }

    pub fn handle_command(&mut self, command: KVCommand) -> Option<Option<String>> {
        match command {
            KVCommand::Put(key, value) => {
                self.db.insert(key, value);
                None
            }
            KVCommand::Delete(key) => {
                self.db.remove(&key);
                None
            }
            KVCommand::Get(key) => Some(self.db.get(&key).map(|v| v.clone())),
        }
    }
}
