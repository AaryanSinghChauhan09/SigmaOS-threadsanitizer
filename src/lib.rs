// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-threadsanitizer: threadsanitizer component model.
//!
//! Real implementation: threadsanitizer_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadsanitizerItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadsanitizerError {
    EmptyThreadsanitizerItemName,
    DuplicateThreadsanitizerItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ThreadsanitizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyThreadsanitizerItemName => write!(f, "threadsanitizer_item name must not be empty"),
            Self::DuplicateThreadsanitizerItem(id) => write!(f, "duplicate threadsanitizer_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ThreadsanitizerError {}

pub struct Threadsanitizer {
    pub name: String,
    threadsanitizer_items: BTreeMap<String, ThreadsanitizerItem>,
    events: BTreeMap<String, Event>,
}

impl Threadsanitizer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            threadsanitizer_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_threadsanitizer_item(&mut self, item: ThreadsanitizerItem) -> Result<(), ThreadsanitizerError> {
        if item.name.trim().is_empty() {
            return Err(ThreadsanitizerError::EmptyThreadsanitizerItemName);
        }
        let id = item.id.clone();
        if self.threadsanitizer_items.contains_key(&id) {
            return Err(ThreadsanitizerError::DuplicateThreadsanitizerItem(id.clone()));
        }
        self.threadsanitizer_items.insert(id, item);
        Ok(())
    }

    pub fn threadsanitizer_item_count(&self) -> usize {
        self.threadsanitizer_items.len()
    }

    pub fn get_threadsanitizer_item(&self, id: &str) -> Option<&ThreadsanitizerItem> {
        self.threadsanitizer_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ThreadsanitizerError> {
        if event.subject.trim().is_empty() {
            return Err(ThreadsanitizerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_threadsanitizer_item(id: &str, name: &str) -> ThreadsanitizerItem {
        ThreadsanitizerItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Threadsanitizer::new("Threadsanitizer");
        assert_eq!(m.name, "Threadsanitizer");
        assert_eq!(m.threadsanitizer_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_threadsanitizer_item() {
        let mut m = Threadsanitizer::new("Threadsanitizer");
        m.add_threadsanitizer_item(sample_threadsanitizer_item("a1", "Alpha")).unwrap();
        assert_eq!(m.threadsanitizer_item_count(), 1);
        assert!(m.get_threadsanitizer_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Threadsanitizer::new("Threadsanitizer");
        assert!(m.add_threadsanitizer_item(sample_threadsanitizer_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Threadsanitizer::new("Threadsanitizer");
        m.add_threadsanitizer_item(sample_threadsanitizer_item("x", "One")).unwrap();
        assert!(m.add_threadsanitizer_item(sample_threadsanitizer_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Threadsanitizer::new("Threadsanitizer");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
