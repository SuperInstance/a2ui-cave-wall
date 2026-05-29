//! Integration tests for a2ui-cave-wall

use a2ui_cave_wall::*;

#[test]
fn test_panel_creation_and_content() {
    let panel = WallPanel::new("vision", "Vision Output")
        .with_content("A red ball on a table")
        .with_priority(5);
    assert_eq!(panel.module, "vision");
    assert_eq!(panel.title, "Vision Output");
    assert_eq!(panel.content, "A red ball on a table");
    assert_eq!(panel.priority, 5);
}

#[test]
fn test_dashboard_layout() {
    let panels = vec![
        WallPanel::new("vision", "Vision").with_content("see"),
        WallPanel::new("sonar", "Sonar").with_content("ping"),
        WallPanel::new("manus", "Manus").with_content("grab"),
    ];
    let layout = WallLayout::dashboard(panels);
    let rendered = layout.render();
    assert!(rendered.contains("Dashboard"));
    assert!(rendered.contains("vision"));
}

#[test]
fn test_stacked_layout() {
    let panels = vec![
        WallPanel::new("a", "A").with_content("content-a"),
        WallPanel::new("b", "B").with_content("content-b"),
    ];
    let layout = WallLayout::stacked(panels);
    let rendered = layout.render();
    assert!(rendered.contains("Stacked"));
    assert!(rendered.contains("content-a"));
    assert!(rendered.contains("content-b"));
}

#[test]
fn test_side_by_side_layout() {
    let panels = vec![
        WallPanel::new("x", "X").with_content("left"),
        WallPanel::new("y", "Y").with_content("right"),
    ];
    let layout = WallLayout::side_by_side(panels);
    let rendered = layout.render();
    assert!(rendered.contains("SideBySide"));
    assert!(rendered.contains("left"));
    assert!(rendered.contains("right"));
}

#[test]
fn test_translator_vision() {
    let t = WallTranslator::new();
    let panel = t.translate_shadow("vision", "A cat sleeping");
    assert_eq!(panel.module, "vision");
    assert!(panel.content.contains("cat sleeping"));
}

#[test]
fn test_translator_sonar() {
    let t = WallTranslator::new();
    let panel = t.translate_shadow("sonar", "D:2.5m D:3.1m");
    assert!(panel.content.contains("2.5m"));
    // format_sonar appends (N) where N = whitespace-split token count
    assert!(panel.content.contains("(2)"));
}

#[test]
fn test_translator_manus() {
    let t = WallTranslator::new();
    let panel = t.translate_shadow("manus", "GRIP:0.8 TEMP:22C");
    assert!(panel.content.contains("GRIP"));
}

#[test]
fn test_translator_unknown_module() {
    let t = WallTranslator::new();
    let panel = t.translate_shadow("custom", "raw data here");
    assert_eq!(panel.content, "raw data here");
}

#[test]
fn test_interaction_click_extracts_item() {
    let mut wi = WallInteraction::new();
    let panel = WallPanel::new("vision", "V").with_content("item:sword at (50,30)");
    let cmd = wi.click(&panel, 50, 30);
    assert!(cmd.contains("examine sword"));
    assert_eq!(wi.history().len(), 1);
}

#[test]
fn test_interaction_click_no_item() {
    let mut wi = WallInteraction::new();
    let panel = WallPanel::new("map", "Map").with_content("plain content");
    let cmd = wi.click(&panel, 10, 20);
    assert!(cmd.contains("look map"));
}

#[test]
fn test_interaction_slider() {
    let mut wi = WallInteraction::new();
    let panel = WallPanel::new("sonar", "Sonar").with_content("range");
    let cmd = wi.slider(&panel, "range", 50);
    assert!(cmd.contains("set sonar range 50"));
    assert_eq!(wi.history().len(), 1);
}

#[test]
fn test_snapshot_json_roundtrip() {
    let panel = WallPanel::new("test", "Test").with_content("hello");
    let layout = WallLayout::stacked(vec![panel]);
    let snap = WallSnapshot::from_layout(&layout);

    let json = snap.to_json().unwrap();
    assert!(json.contains("test"));
    assert!(json.contains("hello"));

    let restored = WallSnapshot::from_json(&json).unwrap();
    assert_eq!(restored.panels.len(), 1);
    assert_eq!(restored.panels[0].module, "test");
}

#[test]
fn test_snapshot_diff_detects_changes() {
    let p1 = WallPanel::new("a", "A").with_content("old");
    let p2 = WallPanel::new("a", "A").with_content("new");

    let s1 = WallSnapshot::from_layout(&WallLayout::stacked(vec![p1]));
    let s2 = WallSnapshot::from_layout(&WallLayout::stacked(vec![p2]));

    let diff = s1.diff(&s2);
    assert_eq!(diff.len(), 1);
    assert!(diff[0].contains("old"));
    assert!(diff[0].contains("new"));
}

#[test]
fn test_snapshot_diff_detects_additions_and_removals() {
    let p1 = WallPanel::new("a", "A").with_content("x");
    let p2 = WallPanel::new("b", "B").with_content("y");

    let s1 = WallSnapshot::from_layout(&WallLayout::stacked(vec![p1]));
    let s2 = WallSnapshot::from_layout(&WallLayout::stacked(vec![p2]));

    let diff = s1.diff(&s2);
    assert!(diff.iter().any(|d| d.contains("removed")));
    assert!(diff.iter().any(|d| d.contains("added")));
}

#[test]
fn test_layout_panels_accessor() {
    let panels = vec![
        WallPanel::new("a", "A"),
        WallPanel::new("b", "B"),
    ];
    let layout = WallLayout::dashboard(panels);
    assert_eq!(layout.panels().len(), 2);
}
