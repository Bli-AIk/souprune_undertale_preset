//! FightBar behavior — WASM-side attack bar sweep logic
//!
//! 攻击条扫描行为 — WASM 侧实现

use souprune_sdk::prelude::*;

/// Per-frame attack bar behavior driven entirely by facts.
///
/// Configuration facts (read):
///   `fight:bar_speed`      — sweep speed in px/s (default: 330.0)
///   `fight:bar_right_edge` — X coordinate where the bar stops (default: 272.0)
///   `fight:bar_start_x`    — starting X offset (default: -274.0)
///   `fight:flash_interval` — seconds between flash color swaps (default: 0.083 ≈ 6Hz)
///
/// State facts (written):
///   `fight:bar_active`   — set externally to start sweep; cleared when done
///   `fight:bar_x`        — current bar X position (drives View offset)
///   `fight:bar_done`     — true when sweep finishes (hit or miss)
///   `fight:confirmed`    — true if player pressed Confirm (hit), false if missed
///   `fight:bar_flash_on` — toggles for SDF FactToggle color swap after hit
///
/// Events emitted:
///   `fight:hit` — when player presses Confirm during sweep
///
/// Flash lasts `FLASH_DURATION` seconds after a hit, then the behavior
/// signals full completion via `fight:bar_complete`.
const FLASH_DURATION: f32 = 0.5;

pub struct FightBarBehavior {
    sweep_x: f32,
    flash_elapsed: f32,
    flash_active: bool,
    sweep_done: bool,
}

impl FightBarBehavior {
    pub fn new() -> Self {
        Self {
            sweep_x: 0.0,
            flash_elapsed: 0.0,
            flash_active: false,
            sweep_done: false,
        }
    }
}

impl Behavior for FightBarBehavior {
    fn on_enter(&mut self, ctx: &mut Context) {
        let start_x = ctx
            .get_fact_float("fight:bar_start_x")
            .unwrap_or(-274.0) as f32;
        self.sweep_x = start_x;
        self.flash_elapsed = 0.0;
        self.flash_active = false;
        self.sweep_done = false;
    }

    fn on_update(&mut self, ctx: &mut Context, dt: f32) {
        let active = ctx.get_fact_bool("fight:bar_active").unwrap_or(false);

        if !active && !self.flash_active {
            return;
        }

        // Flash phase (after hit): toggle SDF colors for FLASH_DURATION
        if self.flash_active {
            if !active {
                self.flash_active = false;
                ctx.set_fact_bool("fight:bar_flash_on", false);
                ctx.set_fact_bool("fight:bar_complete", true);
                return;
            }

            let flash_interval = ctx
                .get_fact_float("fight:flash_interval")
                .unwrap_or(0.083) as f32;
            self.flash_elapsed += dt;

            if self.flash_elapsed >= FLASH_DURATION {
                self.flash_active = false;
                ctx.set_fact_bool("fight:bar_flash_on", false);
                ctx.set_fact_bool("fight:bar_complete", true);
                return;
            }

            let cycle = (self.flash_elapsed / flash_interval) as u32;
            let on = cycle % 2 != 0;
            ctx.set_fact_bool("fight:bar_flash_on", on);
            return;
        }

        if self.sweep_done {
            return;
        }

        // Read configurable parameters from facts
        let speed = ctx.get_fact_float("fight:bar_speed").unwrap_or(330.0) as f32;
        let right_edge = ctx
            .get_fact_float("fight:bar_right_edge")
            .unwrap_or(272.0) as f32;
        let start_x = ctx
            .get_fact_float("fight:bar_start_x")
            .unwrap_or(-274.0) as f32;

        // Initialize sweep position on first active frame
        if self.sweep_x < start_x + 0.01 {
            self.sweep_x = start_x;
        }

        self.sweep_x += speed * dt;
        ctx.set_fact_float("fight:bar_x", self.sweep_x as f64);

        // Miss: reached right edge without input
        if self.sweep_x >= right_edge {
            self.sweep_x = right_edge;
            self.sweep_done = true;
            ctx.set_fact_float("fight:bar_x", right_edge as f64);
            ctx.set_fact_bool("fight:bar_done", true);
            ctx.set_fact_bool("fight:confirmed", false);
            ctx.set_fact_bool("fight:bar_complete", true);
            return;
        }

        // Hit: player pressed Confirm
        if ctx.input().just_pressed(Action::Confirm) {
            self.sweep_done = true;
            self.flash_active = true;
            self.flash_elapsed = 0.0;
            ctx.set_fact_bool("fight:bar_done", true);
            ctx.set_fact_bool("fight:confirmed", true);
            ctx.emit_event("fight:hit");
        }
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        ctx.set_fact_bool("fight:bar_flash_on", false);
    }
}
