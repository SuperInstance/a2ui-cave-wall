use crate::WallLayout;
use serde::{Deserialize, Serialize};

/// A serialized snapshot of the cave wall state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallSnapshot {
    pub panels: Vec<SnapshotPanel>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotPanel {
    pub module: String,
    pub title: String,
    pub content: String,
}

impl WallSnapshot {
    pub fn from_layout(layout: &WallLayout) -> Self {
        let panels = layout.panels().iter().map(|p| SnapshotPanel {
            module: p.module.clone(),
            title: p.title.clone(),
            content: p.content.clone(),
        }).collect();
        Self {
            panels,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Compare two snapshots and return descriptions of differences.
    pub fn diff(&self, other: &WallSnapshot) -> Vec<String> {
        let mut diffs = Vec::new();
        for sp in &self.panels {
            if let Some(other_p) = other.panels.iter().find(|p| p.module == sp.module) {
                if sp.content != other_p.content {
                    diffs.push(format!("{}: '{}' → '{}'", sp.module, sp.content, other_p.content));
                }
            } else {
                diffs.push(format!("{}: removed", sp.module));
            }
        }
        for sp in &other.panels {
            if !self.panels.iter().any(|p| p.module == sp.module) {
                diffs.push(format!("{}: added", sp.module));
            }
        }
        diffs
    }
}
