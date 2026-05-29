use crate::WallPanel;

/// How to arrange panels on the cave wall.
#[derive(Debug, Clone)]
pub enum WallLayout {
    Dashboard { panels: Vec<WallPanel>, cols: usize, rows: usize },
    Stacked { panels: Vec<WallPanel> },
    SideBySide { panels: Vec<WallPanel> },
}

impl WallLayout {
    pub fn dashboard(panels: Vec<WallPanel>) -> Self {
        let n = panels.len();
        let cols = if n <= 1 { 1 } else { 2 };
        let rows = (n + cols - 1) / cols;
        WallLayout::Dashboard { panels, cols, rows }
    }

    pub fn stacked(panels: Vec<WallPanel>) -> Self {
        WallLayout::Stacked { panels }
    }

    pub fn side_by_side(panels: Vec<WallPanel>) -> Self {
        WallLayout::SideBySide { panels }
    }

    pub fn panels(&self) -> &[WallPanel] {
        match self {
            WallLayout::Dashboard { panels, .. } => panels,
            WallLayout::Stacked { panels } => panels,
            WallLayout::SideBySide { panels } => panels,
        }
    }

    pub fn render(&self) -> String {
        match self {
            WallLayout::Dashboard { panels, cols, rows } => {
                let mut s = format!("╔═ Dashboard ({}x{}) ═╗\n", cols, rows);
                for (i, panel) in panels.iter().enumerate() {
                    if i > 0 && i % cols == 0 { s.push('\n'); }
                    s.push_str(&format!("┌─ {} ─┐ ", panel.module));
                }
                s.push('\n');
                for panel in panels {
                    s.push_str(&format!("│ {} │ {}\n", panel.module, panel.content));
                }
                s.push('╚');
                s
            }
            WallLayout::Stacked { panels } => {
                let mut s = String::from("╔═ Stacked ═╗\n");
                for panel in panels {
                    s.push_str(&format!("┌─ {} ─ {}\n│ {}\n└──────────\n",
                        panel.title, panel.module, panel.content));
                }
                s
            }
            WallLayout::SideBySide { panels } => {
                let mut s = String::from("╔═ SideBySide ═╗\n");
                let headers: Vec<String> = panels.iter()
                    .map(|p| format!("[{}]", p.module))
                    .collect();
                s.push_str(&headers.join(" | "));
                s.push('\n');
                for panel in panels {
                    s.push_str(&format!("{}: {}  ", panel.module, panel.content));
                }
                s.push('\n');
                s
            }
        }
    }
}
