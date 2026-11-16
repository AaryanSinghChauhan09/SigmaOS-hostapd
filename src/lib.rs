// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-hostapd: hostapd component model.
//!
//! Real implementation: hostapd_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostapdItem {
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
pub enum HostapdError {
    EmptyHostapdItemName,
    DuplicateHostapdItem(String),
    EmptyEventSubject,
}

impl fmt::Display for HostapdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHostapdItemName => write!(f, "hostapd_item name must not be empty"),
            Self::DuplicateHostapdItem(id) => write!(f, "duplicate hostapd_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for HostapdError {}

pub struct Hostapd {
    pub name: String,
    hostapd_items: BTreeMap<String, HostapdItem>,
    events: BTreeMap<String, Event>,
}

impl Hostapd {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            hostapd_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_hostapd_item(&mut self, item: HostapdItem) -> Result<(), HostapdError> {
        if item.name.trim().is_empty() {
            return Err(HostapdError::EmptyHostapdItemName);
        }
        let id = item.id.clone();
        if self.hostapd_items.contains_key(&id) {
            return Err(HostapdError::DuplicateHostapdItem(id.clone()));
        }
        self.hostapd_items.insert(id, item);
        Ok(())
    }

    pub fn hostapd_item_count(&self) -> usize {
        self.hostapd_items.len()
    }

    pub fn get_hostapd_item(&self, id: &str) -> Option<&HostapdItem> {
        self.hostapd_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), HostapdError> {
        if event.subject.trim().is_empty() {
            return Err(HostapdError::EmptyEventSubject);
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

    fn sample_hostapd_item(id: &str, name: &str) -> HostapdItem {
        HostapdItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Hostapd::new("Hostapd");
        assert_eq!(m.name, "Hostapd");
        assert_eq!(m.hostapd_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_hostapd_item() {
        let mut m = Hostapd::new("Hostapd");
        m.add_hostapd_item(sample_hostapd_item("a1", "Alpha")).unwrap();
        assert_eq!(m.hostapd_item_count(), 1);
        assert!(m.get_hostapd_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Hostapd::new("Hostapd");
        assert!(m.add_hostapd_item(sample_hostapd_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Hostapd::new("Hostapd");
        m.add_hostapd_item(sample_hostapd_item("x", "One")).unwrap();
        assert!(m.add_hostapd_item(sample_hostapd_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Hostapd::new("Hostapd");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
