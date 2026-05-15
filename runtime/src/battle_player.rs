//! Undertale battle player helpers built from generic host primitives.
//!
//! 基于通用宿主 primitive 组合出的 Undertale 战斗玩家辅助逻辑。

use souprune_sdk::prelude::*;

use crate::battle_box::player_box_fact;

const DAMAGE_DURATION_FACT: &str = "battle:damage_target:invincibility_duration";
const FLASH_INTERVAL_FACT: &str = "battle:damage_target:flash_interval";
const DAMAGE_SOUND_FACT: &str = "battle:damage_target:damage_sound";
const NORMAL_COLOR_PREFIX: &str = "battle:damage_target:normal_color";
const FLASH_COLOR_PREFIX: &str = "battle:damage_target:flash_color";
const PLAYER_ENTITY_FACT: &str = "ut:battle_player:entity";

/// Project-level battle player spawn settings.
///
/// 项目级战斗玩家生成设置。
#[derive(Debug, Clone, PartialEq)]
pub struct BattlePlayerSpawn {
    pub texture: String,
    pub position: Vec2,
    pub z: f32,
    pub color: Rgba,
    pub physics_radius: f32,
    pub trigger_half_size: Vec2,
    pub behavior_id: String,
    pub behavior_context: String,
    pub mode_scope: String,
    pub box_id: String,
    pub invincibility: PlayerInvincibilitySettings,
}

/// Project-level battle damage flash settings.
///
/// 项目级战斗受击闪烁设置。
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInvincibilitySettings {
    pub duration: f32,
    pub flash_interval: f32,
    pub normal_color: Rgba,
    pub flash_color: Rgba,
    pub damage_sound: Option<String>,
}

impl BattlePlayerSpawn {
    fn spawn(self, ctx: &Context) -> bool {
        write_invincibility_settings(ctx, &self.invincibility);
        ctx.set_fact_string(player_box_fact(), &self.box_id);

        let mut config = SpriteEntityConfig::new(self.texture, self.position);
        config.z = self.z;
        config.color = self.color;
        config.physics_collider = Some(ColliderShape::Circle {
            radius: self.physics_radius,
        });
        config.trigger_collider = Some(ColliderShape::Rectangle {
            half_size: self.trigger_half_size,
        });
        config.behavior_id = Some(self.behavior_id);
        config.behavior_context = Some(self.behavior_context);
        config.mode_scope = Some(self.mode_scope);
        config.bullet_target = true;
        config.name = Some("Player".into());

        let Some(handle) = ctx.entity().spawn_sprite(config) else {
            ctx.warn("SpawnBattlePlayer failed to create host sprite entity");
            return false;
        };
        ctx.set_fact_int(PLAYER_ENTITY_FACT, handle.0 as i64);
        true
    }
}

/// Return custom action IDs handled by this module.
///
/// 返回本模块处理的自定义 action ID。
pub fn handled_actions() -> Vec<String> {
    vec!["SpawnBattlePlayer".into()]
}

/// Handle a project-level battle player custom action.
///
/// 处理项目级战斗玩家自定义 action。
pub fn handle_action(ctx: &Context, action_type: &str, params: &[ActionParam]) -> bool {
    if action_type != "SpawnBattlePlayer" {
        return false;
    }
    let pairs: Vec<(&str, &str)> = params
        .iter()
        .map(|param| (param.name.as_str(), param.value.as_str()))
        .collect();
    let Some(spawn) = parse_spawn_battle_player(&pairs) else {
        ctx.warn("SpawnBattlePlayer missing required numeric parameter");
        return false;
    };
    spawn.spawn(ctx)
}

/// Parse a `SpawnBattlePlayer` custom action into typed settings.
///
/// 将 `SpawnBattlePlayer` 自定义 action 解析为类型化设置。
pub fn parse_spawn_battle_player(params: &[(&str, &str)]) -> Option<BattlePlayerSpawn> {
    Some(BattlePlayerSpawn {
        texture: param(params, "texture")
            .unwrap_or("assets/textures/common/view/heart.png")
            .to_string(),
        position: Vec2::new(
            parse_f32_param(params, "x").unwrap_or(0.0),
            parse_f32_param(params, "y").unwrap_or(-80.0),
        ),
        z: parse_f32_param(params, "z").unwrap_or(10.0),
        color: parse_color(params, "color", Rgba::new(1.0, 0.0, 0.0, 1.0)),
        physics_radius: parse_f32_param(params, "physics_radius").unwrap_or(8.0),
        trigger_half_size: Vec2::new(
            parse_f32_param(params, "trigger_half_width").unwrap_or(2.0),
            parse_f32_param(params, "trigger_half_height").unwrap_or(2.0),
        ),
        behavior_id: param(params, "behavior_id")
            .unwrap_or("soul_red")
            .to_string(),
        behavior_context: param(params, "behavior_context")
            .unwrap_or("battle")
            .to_string(),
        mode_scope: param(params, "mode_scope").unwrap_or("battle").to_string(),
        box_id: param(params, "box_id").unwrap_or("main").to_string(),
        invincibility: PlayerInvincibilitySettings {
            duration: parse_f32_param(params, "invincibility_duration").unwrap_or(1.0),
            flash_interval: parse_f32_param(params, "flash_interval").unwrap_or(0.1),
            normal_color: parse_color(params, "normal_color", Rgba::new(1.0, 0.0, 0.0, 1.0)),
            flash_color: parse_color(params, "flash_color", Rgba::new(0.5, 0.0, 0.0, 1.0)),
            damage_sound: param(params, "damage_sound").map(ToString::to_string),
        },
    })
}

fn write_invincibility_settings(ctx: &Context, settings: &PlayerInvincibilitySettings) {
    ctx.set_fact_float(DAMAGE_DURATION_FACT, settings.duration as f64);
    ctx.set_fact_float(FLASH_INTERVAL_FACT, settings.flash_interval as f64);
    write_color(ctx, NORMAL_COLOR_PREFIX, settings.normal_color);
    write_color(ctx, FLASH_COLOR_PREFIX, settings.flash_color);
    if let Some(sound) = &settings.damage_sound {
        ctx.set_fact_string(DAMAGE_SOUND_FACT, sound);
    }
}

fn write_color(ctx: &Context, prefix: &str, color: Rgba) {
    ctx.set_fact_float(&format!("{prefix}:red"), color.red as f64);
    ctx.set_fact_float(&format!("{prefix}:green"), color.green as f64);
    ctx.set_fact_float(&format!("{prefix}:blue"), color.blue as f64);
    ctx.set_fact_float(&format!("{prefix}:alpha"), color.alpha as f64);
}

fn parse_color(params: &[(&str, &str)], prefix: &str, default: Rgba) -> Rgba {
    Rgba::new(
        parse_f32_param(params, &format!("{prefix}_red")).unwrap_or(default.red),
        parse_f32_param(params, &format!("{prefix}_green")).unwrap_or(default.green),
        parse_f32_param(params, &format!("{prefix}_blue")).unwrap_or(default.blue),
        parse_f32_param(params, &format!("{prefix}_alpha")).unwrap_or(default.alpha),
    )
}

fn param<'a>(params: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
    params
        .iter()
        .find_map(|(name, value)| (*name == key).then_some(*value))
}

fn parse_f32_param(params: &[(&str, &str)], key: &str) -> Option<f32> {
    param(params, key)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_spawn_battle_player_custom_action_params() {
        let params = [
            ("texture", "assets/textures/common/view/heart.png"),
            ("x", "0.0"),
            ("y", "-80.0"),
            ("z", "10.0"),
            ("behavior_id", "soul_red"),
            ("box_id", "main"),
            ("physics_radius", "8.0"),
            ("trigger_half_width", "2.0"),
            ("trigger_half_height", "2.0"),
            ("invincibility_duration", "1.0"),
            ("flash_interval", "0.1"),
            ("damage_sound", "assets/audios/sfx/hurtsound.wav"),
        ];

        let spawn = parse_spawn_battle_player(&params).expect("spawn battle player");

        assert_eq!(spawn.position, Vec2::new(0.0, -80.0));
        assert_eq!(spawn.behavior_id, "soul_red");
        assert_eq!(spawn.box_id, "main");
        assert_eq!(spawn.physics_radius, 8.0);
        assert_eq!(spawn.trigger_half_size, Vec2::new(2.0, 2.0));
        assert_eq!(
            spawn.invincibility.damage_sound.as_deref(),
            Some("assets/audios/sfx/hurtsound.wav")
        );
    }
}
