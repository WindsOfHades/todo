use std::fmt::Display;

use crate::priority::Priority;
use chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    id: String,
    title: String,
    priority: Priority,
    done: bool,
    created_at: String,
}

impl Task {
    pub fn new(id: &str, title: String, prio: Priority) -> Self {
        Self {
            id: id.to_string(),
            title,
            priority: prio,
            done: false,
            created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    pub fn is_done(&self) -> bool {
        self.done == true
    }

    pub fn set_done(&mut self, state: bool) {
        self.done = state;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.done { "✅" } else { "⬜" };
        let prio = match self.priority {
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🔴",
        };
        write!(
            f,
            "{} {} {} {} ({})",
            status, self.id, prio, self.title, self.created_at
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_task() {
        let t = Task::new("1", "hi".to_string(), Priority::High);
        assert_eq!(t.id, "1");
        assert_eq!(t.title, "hi");
        assert_eq!(t.priority, Priority::High);
        assert!(!t.done);
        let parsed = chrono::NaiveDateTime::parse_from_str(&t.created_at, "%Y-%m-%d %H:%M:%S");
        assert!(parsed.is_ok());
    }
    #[test]
    fn get_id() {
        let t = Task::new("1", "hi".to_string(), Priority::High);
        assert_eq!(t.id(), "1");
    }

    #[test]
    fn get_prio() {
        let t = Task::new("1", "hi".to_string(), Priority::High);
        assert_eq!(t.priority(), &Priority::High);
    }

    #[test]
    fn set_and_get_done() {
        let mut t = Task::new("1", "hi".to_string(), Priority::High);
        t.set_done(true);
        assert!(t.is_done());
    }
}
