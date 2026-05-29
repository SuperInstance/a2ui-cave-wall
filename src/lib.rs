mod panel;
mod layout;
mod translator;
mod interaction;
mod snapshot;

pub use panel::*;
pub use layout::*;
pub use translator::*;
pub use interaction::*;
pub use snapshot::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_panel() {
        let panel = WallPanel::new("vision", "Visual perception output");
        assert_eq!(panel.module, "vision");
        assert_eq!(panel.title, "Visual perception output");
        assert!(panel.content.is_empty());
    }

    #[test]
    fn test_layout_panels() {
        let p1 = WallPanel::new("vision", "Vision");
        let p2 = WallPanel::new("sonar", "Sonar");
        let layout = WallLayout::dashboard(vec![p1, p2]);
        let rendered = layout.render();
        assert!(rendered.contains("vision"));
        assert!(rendered.contains("sonar"));
    }

    #[test]
    fn test_translate_vision_shadow() {
        let t = WallTranslator::new();
        let panel = t.translate_shadow("vision", "A red ball sits on a table. Bright overhead light.");
        assert_eq!(panel.module, "vision");
        assert!(panel.content.contains("red ball"));
    }

    #[test]
    fn test_translate_sonar_shadow() {
        let t = WallTranslator::new();
        let panel = t.translate_shadow("sonar", "D:2.5m D:3.1m D:1.0m");
        assert_eq!(panel.module, "sonar");
        assert!(panel.content.contains("2.5m"));
    }

    #[test]
    fn test_translate_manus_shadow() {
        let t = WallTranslator::new();
        let panel = t.translate_shadow("manus", "GRIP:0.8 TEMP:22C");
        assert_eq!(panel.module, "manus");
        assert!(panel.content.contains("GRIP"));
    }

    #[test]
    fn test_click_generates_mud_command() {
        let mut wi = WallInteraction::new();
        let panel = WallPanel::new("vision", "Vision").with_content("item:cup at (120,45)");
        let cmd = wi.click(&panel, 120, 45);
        assert!(cmd.contains("examine cup"));
    }

    #[test]
    fn test_slider_adjustment_changes_policy() {
        let mut wi = WallInteraction::new();
        let panel = WallPanel::new("sonar", "Sonar Range").with_content("range:10");
        let cmd = wi.slider(&panel, "range", 25);
        assert!(cmd.contains("set sonar range 25"));
    }

    #[test]
    fn test_snapshot_json() {
        let panel = WallPanel::new("vision", "Vision").with_content("test content");
        let layout = WallLayout::stacked(vec![panel]);
        let snap = WallSnapshot::from_layout(&layout);
        let json = snap.to_json().unwrap();
        assert!(json.contains("vision"));
        assert!(json.contains("test content"));
    }

    #[test]
    fn test_snapshot_comparison() {
        let p1 = WallPanel::new("vision", "V").with_content("a");
        let p2 = WallPanel::new("vision", "V").with_content("b");
        let s1 = WallSnapshot::from_layout(&WallLayout::stacked(vec![p1]));
        let s2 = WallSnapshot::from_layout(&WallLayout::stacked(vec![p2]));
        let diff = s1.diff(&s2);
        assert!(!diff.is_empty());
    }

    #[test]
    fn test_multiple_panels() {
        let panels = vec![
            WallPanel::new("vision", "Vision").with_content("see"),
            WallPanel::new("sonar", "Sonar").with_content("ping"),
            WallPanel::new("manus", "Manus").with_content("grab"),
        ];
        let layout = WallLayout::side_by_side(panels);
        let rendered = layout.render();
        assert!(rendered.contains("vision"));
        assert!(rendered.contains("sonar"));
        assert!(rendered.contains("manus"));
    }
}
