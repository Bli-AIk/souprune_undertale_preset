//! Shared helper modules for this content guest.
//!
//! 当前内容 guest 的共享辅助模块。

use souprune_schema::fre::RuleEventDef;

/// Build the semantic FRE input event for a configured action.
///
/// 为配置动作构造语义 FRE 输入事件。
pub fn input_event(action: &str) -> RuleEventDef {
    RuleEventDef::Event(input_event_id(action).into())
}

fn input_event_id(action: &str) -> &'static str {
    match action {
        "Up" => "input:navigate_up",
        "Down" => "input:navigate_down",
        "Left" => "input:navigate_left",
        "Right" => "input:navigate_right",
        "Confirm" => "input:confirm",
        "Cancel" => "input:cancel",
        "Menu" => "input:menu",
        other => panic!("unsupported semantic input action: {other}"),
    }
}
