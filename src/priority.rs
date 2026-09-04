use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Priority {
    High,
    Low,
    Medium,
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" | "h" => Ok(Priority::High),
            "medium" | "m" => Ok(Priority::Medium),
            "low" | "l" => Ok(Priority::Low),
            _ => Err(format!("{s} is not a valid priority")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_case_string_to_high_prio() {
        let result = Priority::from_str("high");
        assert_eq!(result, Ok(Priority::High));
        let result = Priority::from_str("h");
        assert_eq!(result, Ok(Priority::High));
    }

    #[test]
    fn upper_case_string_to_high_prio() {
        let result = Priority::from_str("HIGH");
        assert_eq!(result, Ok(Priority::High));
        let result = Priority::from_str("H");
        assert_eq!(result, Ok(Priority::High));
    }

    #[test]
    fn string_to_medium_prio() {
        let result = Priority::from_str("medium");
        assert_eq!(result, Ok(Priority::Medium));
        let result = Priority::from_str("m");
        assert_eq!(result, Ok(Priority::Medium));
    }

    #[test]
    fn string_to_low_prio() {
        let result = Priority::from_str("low");
        assert_eq!(result, Ok(Priority::Low));
        let result = Priority::from_str("l");
        assert_eq!(result, Ok(Priority::Low));
    }

    #[test]
    fn empty_string_to_prio() {
        let result = Priority::from_str("");
        assert!(result.is_err());
    }

    #[test]
    fn unknown_string_to_prio() {
        let result = Priority::from_str("bad");
        assert!(result.is_err());
    }
}
