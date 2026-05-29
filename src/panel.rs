use serde::{Deserialize, Serialize};

/// A section of the cave wall showing one sense module's output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallPanel {
    pub module: String,
    pub title: String,
    pub content: String,
    pub priority: u32,
}

impl WallPanel {
    pub fn new(module: &str, title: &str) -> Self {
        Self {
            module: module.to_string(),
            title: title.to_string(),
            content: String::new(),
            priority: 0,
        }
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}
