use crate::WallPanel;

/// Translates between shadow format (raw agent data) and panel format (human-readable).
#[derive(Debug, Clone)]
pub struct WallTranslator {
    /// Module-specific formatting rules.
    formatters: std::collections::HashMap<String, fn(&str) -> String>,
}

impl WallTranslator {
    pub fn new() -> Self {
        let mut formatters = std::collections::HashMap::new();
        formatters.insert("vision".to_string(), format_vision as fn(&str) -> String);
        formatters.insert("sonar".to_string(), format_sonar as fn(&str) -> String);
        formatters.insert("manus".to_string(), format_manus as fn(&str) -> String);
        formatters.insert("audio".to_string(), format_audio as fn(&str) -> String);
        Self { formatters }
    }

    /// Translate a raw shadow string into a WallPanel.
    pub fn translate_shadow(&self, module: &str, shadow_data: &str) -> WallPanel {
        let title = format!("{} panel", module);
        let content = match self.formatters.get(module) {
            Some(fmt) => fmt(shadow_data),
            None => shadow_data.to_string(),
        };
        WallPanel::new(module, &title).with_content(&content)
    }
}

fn format_vision(data: &str) -> String {
    format!("👁 {}", data)
}

fn format_sonar(data: &str) -> String {
    // Parse distance readings like "D:2.5m D:3.1m"
    let readings: Vec<&str> = data.split_whitespace().collect();
    format!("📡 {} ({})", data, readings.len())
}

fn format_manus(data: &str) -> String {
    format!("🤚 {}", data)
}

fn format_audio(data: &str) -> String {
    format!("🔊 {}", data)
}
