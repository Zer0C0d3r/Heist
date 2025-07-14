//! Data models for commands, sessions, and history entries
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: Option<DateTime<Local>>,
    pub command: String,
    pub session_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: u64,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub commands: Vec<HistoryEntry>,
}
