use super::*;

fn commit_text(result: &EngineResult) -> Option<String> {
    result.actions.iter().find_map(|action| {
        if let EngineAction::Commit(text) = action {
            Some(text.clone())
        } else {
            None
        }
    })
}

fn has_hide_candidates(result: &EngineResult) -> bool {
    result
        .actions
        .iter()
        .any(|action| matches!(action, EngineAction::HideCandidates))
}

fn enter_conversion_with_ai() -> InputMethodEngine {
    let mut engine = InputMethodEngine::new();
    engine.process_key(&press('a'));
    engine.process_key(&press('i'));
    engine.process_key(&press_key(Keysym::SPACE));
    assert!(matches!(engine.state(), InputState::Conversion { .. }));
    engine
}

#[test]
fn function_key_does_not_apply_while_composing() {
    let mut engine = InputMethodEngine::new();
    engine.process_key(&press('a'));
    assert!(matches!(engine.state(), InputState::Composing { .. }));

    let result = engine.process_key(&press_key(Keysym::F7));

    assert!(!result.consumed);
    assert_eq!(engine.preedit().unwrap().text(), "あ");
    assert!(matches!(engine.state(), InputState::Composing { .. }));
}

#[test]
fn f6_converts_explicit_conversion_to_hiragana() {
    let mut engine = enter_conversion_with_ai();

    let result = engine.process_key(&press_key(Keysym::F6));

    assert!(result.consumed);
    assert!(has_hide_candidates(&result));
    assert_eq!(engine.preedit().unwrap().text(), "あい");

    let enter = engine.process_key(&press_key(Keysym::RETURN));
    assert_eq!(commit_text(&enter).as_deref(), Some("あい"));
}

#[test]
fn f7_converts_explicit_conversion_to_fullwidth_katakana() {
    let mut engine = enter_conversion_with_ai();

    let result = engine.process_key(&press_key(Keysym::F7));

    assert!(result.consumed);
    assert!(has_hide_candidates(&result));
    assert_eq!(engine.preedit().unwrap().text(), "アイ");

    let enter = engine.process_key(&press_key(Keysym::RETURN));
    assert_eq!(commit_text(&enter).as_deref(), Some("アイ"));
}

#[test]
fn f8_converts_explicit_conversion_to_halfwidth_katakana() {
    let mut engine = enter_conversion_with_ai();

    let result = engine.process_key(&press_key(Keysym::F8));

    assert!(result.consumed);
    assert!(has_hide_candidates(&result));
    assert_eq!(engine.preedit().unwrap().text(), "ｱｲ");

    let enter = engine.process_key(&press_key(Keysym::RETURN));
    assert_eq!(commit_text(&enter).as_deref(), Some("ｱｲ"));
}

#[test]
fn f9_converts_explicit_conversion_to_fullwidth_alnum() {
    let mut engine = enter_conversion_with_ai();

    let result = engine.process_key(&press_key(Keysym::F9));

    assert!(result.consumed);
    assert!(has_hide_candidates(&result));
    assert_eq!(engine.preedit().unwrap().text(), "ａｉ");

    let enter = engine.process_key(&press_key(Keysym::RETURN));
    assert_eq!(commit_text(&enter).as_deref(), Some("ａｉ"));
}

#[test]
fn f10_converts_explicit_conversion_to_halfwidth_alnum() {
    let mut engine = enter_conversion_with_ai();

    let result = engine.process_key(&press_key(Keysym::F10));

    assert!(result.consumed);
    assert!(has_hide_candidates(&result));
    assert_eq!(engine.preedit().unwrap().text(), "ai");

    let enter = engine.process_key(&press_key(Keysym::RETURN));
    assert_eq!(commit_text(&enter).as_deref(), Some("ai"));
}
