use crate::WallPanel;

/// Handles human interactions with the cave wall — clicks, sliders, etc.
/// Translates them back into MUD commands for the agent.
#[derive(Debug, Clone)]
pub struct WallInteraction {
    history: Vec<String>,
}

impl WallInteraction {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    /// Handle a click at (x, y) on a panel. Returns a MUD command string.
    pub fn click(&mut self, panel: &WallPanel, x: u32, y: u32) -> String {
        // Parse content for clickable items like "item:name at (x,y)"
        let cmd = if let Some(item) = extract_item_at(&panel.content, x, y) {
            format!("examine {}", item)
        } else {
            format!("look {} ({},{})", panel.module, x, y)
        };
        self.history.push(cmd.clone());
        cmd
    }

    /// Handle a slider adjustment on a panel parameter. Returns a MUD command.
    pub fn slider(&mut self, panel: &WallPanel, param: &str, value: u32) -> String {
        let cmd = format!("set {} {} {}", panel.module, param, value);
        self.history.push(cmd.clone());
        cmd
    }

    pub fn history(&self) -> &[String] {
        &self.history
    }
}

fn extract_item_at(content: &str, x: u32, y: u32) -> Option<String> {
    // Look for patterns like "item:name at (x,y)"
    for part in content.split(',') {
        if part.starts_with("item:") {
            let name = part.split_at(5).1;
            return Some(name.to_string());
        }
    }
    None
}
