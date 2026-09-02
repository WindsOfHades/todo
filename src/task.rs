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
