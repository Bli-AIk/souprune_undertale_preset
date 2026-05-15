//! Undertale battle box helpers built from generic host primitives.
//!
//! 基于通用宿主 primitive 组合出的 Undertale 战斗框辅助逻辑。

use std::collections::HashMap;

use souprune_sdk::prelude::*;

const DEFAULT_BOX_ID: &str = "main";
const DEFAULT_CENTER: Vec2 = Vec2 { x: -0.5, y: -80.0 };
const DEFAULT_SIZE: Vec2 = Vec2 { x: 565.0, y: 130.0 };
const BOX_IDS_FACT: &str = "ut:battle_box:ids";

/// Runtime bounds for one battle box.
///
/// 单个战斗框的运行时边界。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BattleBoxBounds {
    pub center: Vec2,
    pub size: Vec2,
}

impl BattleBoxBounds {
    /// Return half-size for collision-region creation.
    ///
    /// 返回用于创建碰撞区域的半尺寸。
    pub fn half_size(self) -> Vec2 {
        self.size * 0.5
    }

    /// Interpolate center and size linearly.
    ///
    /// 线性插值中心点和尺寸。
    pub fn lerp(self, target: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            center: self.center + (target.center - self.center) * t,
            size: self.size + (target.size - self.size) * t,
        }
    }
}

impl Default for BattleBoxBounds {
    fn default() -> Self {
        Self {
            center: DEFAULT_CENTER,
            size: DEFAULT_SIZE,
        }
    }
}

/// Project-level battle box command parsed from custom actions.
///
/// 从自定义 action 解析出的项目级战斗框命令。
#[derive(Debug, Clone, PartialEq)]
pub enum BattleBoxCommand {
    Spawn {
        id: String,
        bounds: BattleBoxBounds,
    },
    SetBounds {
        id: String,
        bounds: BattleBoxBounds,
        duration: f32,
    },
    BindPlayer {
        id: String,
    },
    Split {
        source: String,
        first: String,
        second: String,
        axis: SplitAxis,
        position: f32,
        gap: f32,
        gap_policy: GapPolicy,
        duration: f32,
    },
    Merge {
        first: String,
        second: String,
        result: String,
        duration: f32,
    },
}

/// Axis used to split a battle box.
///
/// 分裂战斗框时使用的轴。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitAxis {
    Vertical,
    Horizontal,
}

/// Whether a split gap expands the outer bounds or is included inside them.
///
/// 分裂间隙是扩展外边界，还是包含在原边界内。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapPolicy {
    Expands,
    Includes,
}

#[derive(Clone, Copy)]
struct ActiveBattleBox {
    region: RegionHandle,
    bounds: BattleBoxBounds,
    seen_revision: i64,
    tween: Option<BoundsTween>,
}

#[derive(Clone, Copy)]
struct BoundsTween {
    start: BattleBoxBounds,
    end: BattleBoxBounds,
    elapsed: f32,
    duration: f32,
}

/// Behavior that owns Undertale battle collision regions.
///
/// 持有 Undertale 战斗碰撞区域的行为。
pub struct BattleArea {
    boxes: HashMap<String, ActiveBattleBox>,
}

impl BattleArea {
    pub fn new() -> Self {
        Self {
            boxes: HashMap::new(),
        }
    }

    fn ensure_box(&mut self, ctx: &mut Context, id: &str, bounds: BattleBoxBounds) {
        if self.boxes.contains_key(id) {
            return;
        }
        let Some(region) = ctx
            .collision()
            .create_region(bounds.center, bounds.half_size())
        else {
            ctx.warn("BattleArea failed to create collision region");
            return;
        };
        ctx.set_fact_int(&region_fact(id), region.0 as i64);
        self.boxes.insert(
            id.to_string(),
            ActiveBattleBox {
                region,
                bounds,
                seen_revision: read_i64(ctx, &revision_fact(id), 0),
                tween: None,
            },
        );
    }

    fn sync_target(&mut self, ctx: &mut Context, id: &str) {
        let revision = read_i64(ctx, &revision_fact(id), 0);
        let Some(active) = self.boxes.get_mut(id) else {
            return;
        };
        if active.seen_revision == revision {
            return;
        }
        active.seen_revision = revision;

        let target = read_bounds(ctx, id).unwrap_or(active.bounds);
        let duration = read_f32(ctx, &duration_fact(id), 0.0).max(0.0);
        if duration > 0.0 {
            active.tween = Some(BoundsTween {
                start: active.bounds,
                end: target,
                elapsed: 0.0,
                duration,
            });
        } else {
            active.bounds = target;
            active.tween = None;
            ctx.collision()
                .set_region_bounds(active.region, target.center, target.half_size());
        }
    }

    fn advance_tweens(&mut self, ctx: &mut Context, delta_time: f32) {
        for active in self.boxes.values_mut() {
            let Some(mut tween) = active.tween else {
                continue;
            };
            tween.elapsed += delta_time.max(0.0);
            let progress = if tween.duration <= f32::EPSILON {
                1.0
            } else {
                tween.elapsed / tween.duration
            };
            let next = tween.start.lerp(tween.end, progress);
            active.bounds = next;
            ctx.collision()
                .set_region_bounds(active.region, next.center, next.half_size());
            if progress >= 1.0 {
                active.tween = None;
            } else {
                active.tween = Some(tween);
            }
        }
    }

    fn remove_missing_boxes(&mut self, ctx: &mut Context, ids: &[String]) {
        let stale: Vec<String> = self
            .boxes
            .keys()
            .filter(|id| !ids.iter().any(|active| active == *id))
            .cloned()
            .collect();
        for id in stale {
            if let Some(active) = self.boxes.remove(&id) {
                ctx.collision().remove_region(active.region);
            }
        }
    }
}

impl Default for BattleArea {
    fn default() -> Self {
        Self::new()
    }
}

impl Behavior for BattleArea {
    fn on_enter(&mut self, ctx: &mut Context) {
        self.ensure_box(ctx, DEFAULT_BOX_ID, BattleBoxBounds::default());
        ensure_box_id_fact(ctx, DEFAULT_BOX_ID);
    }

    fn on_update(&mut self, ctx: &mut Context, delta_time: f32) {
        let ids = battle_box_ids(ctx);
        self.remove_missing_boxes(ctx, &ids);
        for id in &ids {
            let bounds = read_bounds(ctx, id).unwrap_or_else(BattleBoxBounds::default);
            self.ensure_box(ctx, id, bounds);
            self.sync_target(ctx, id);
        }
        self.advance_tweens(ctx, delta_time);
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        for (_, active) in self.boxes.drain() {
            ctx.collision().remove_region(active.region);
        }
    }
}

/// Custom action handler for project-level battle box commands.
///
/// 项目级战斗框命令的自定义 action 处理器。
#[derive(Default)]
pub struct BattleBoxActionHandler;

impl CustomActionHandler for BattleBoxActionHandler {
    fn handled_actions() -> Vec<String> {
        vec![
            "SpawnBattleBox".into(),
            "SetBattleBoxBounds".into(),
            "BindPlayerToBox".into(),
            "SplitBattleBox".into(),
            "MergeBattleBoxes".into(),
        ]
    }

    fn handle_action(&self, ctx: &Context, action_type: &str, params: &[ActionParam]) -> bool {
        let pairs: Vec<(&str, &str)> = params
            .iter()
            .map(|param| (param.name.as_str(), param.value.as_str()))
            .collect();
        let Some(command) = parse_battle_box_command(action_type, &pairs) else {
            return false;
        };

        match command {
            BattleBoxCommand::Spawn { id, bounds } => {
                write_target_bounds(ctx, &id, bounds, 0.0);
                ensure_box_id_fact(ctx, &id);
            }
            BattleBoxCommand::SetBounds {
                id,
                bounds,
                duration,
            } => {
                write_target_bounds(ctx, &id, bounds, duration);
                ensure_box_id_fact(ctx, &id);
            }
            BattleBoxCommand::BindPlayer { id } => {
                ctx.set_fact_string(player_box_fact(), &id);
            }
            BattleBoxCommand::Split {
                source,
                first,
                second,
                axis,
                position,
                gap,
                gap_policy,
                duration,
            } => {
                let bounds = read_bounds(ctx, &source).unwrap_or_default();
                let (first_bounds, second_bounds) =
                    split_bounds(bounds, axis, position, gap, gap_policy);
                write_target_bounds(ctx, &first, first_bounds, duration);
                write_target_bounds(ctx, &second, second_bounds, duration);
                replace_box_ids(ctx, &[source], &[first, second]);
            }
            BattleBoxCommand::Merge {
                first,
                second,
                result,
                duration,
            } => {
                let first_bounds = read_bounds(ctx, &first).unwrap_or_default();
                let second_bounds = read_bounds(ctx, &second).unwrap_or_default();
                write_target_bounds(
                    ctx,
                    &result,
                    merge_bounds(first_bounds, second_bounds),
                    duration,
                );
                replace_box_ids(ctx, &[first, second], &[result]);
            }
        }

        true
    }
}

/// Parse a battle box custom action into a typed command.
///
/// 将战斗框自定义 action 解析为类型化命令。
pub fn parse_battle_box_command(
    action_type: &str,
    params: &[(&str, &str)],
) -> Option<BattleBoxCommand> {
    match action_type {
        "SpawnBattleBox" => Some(BattleBoxCommand::Spawn {
            id: param(params, "id").unwrap_or(DEFAULT_BOX_ID).to_string(),
            bounds: parse_bounds(params)?,
        }),
        "SetBattleBoxBounds" => Some(BattleBoxCommand::SetBounds {
            id: param(params, "id").unwrap_or(DEFAULT_BOX_ID).to_string(),
            bounds: parse_bounds(params)?,
            duration: parse_f32_param(params, "duration").unwrap_or(0.0),
        }),
        "BindPlayerToBox" => Some(BattleBoxCommand::BindPlayer {
            id: param(params, "id").unwrap_or(DEFAULT_BOX_ID).to_string(),
        }),
        "SplitBattleBox" => Some(BattleBoxCommand::Split {
            source: param(params, "source")
                .unwrap_or(DEFAULT_BOX_ID)
                .to_string(),
            first: param(params, "first")
                .or_else(|| param(params, "left"))
                .unwrap_or("left")
                .to_string(),
            second: param(params, "second")
                .or_else(|| param(params, "right"))
                .unwrap_or("right")
                .to_string(),
            axis: parse_split_axis(param(params, "axis").unwrap_or("Vertical")),
            position: parse_f32_param(params, "position").unwrap_or(0.0),
            gap: parse_f32_param(params, "gap").unwrap_or(0.0),
            gap_policy: parse_gap_policy(param(params, "gap_policy").unwrap_or("Expands")),
            duration: parse_f32_param(params, "duration").unwrap_or(0.0),
        }),
        "MergeBattleBoxes" => Some(BattleBoxCommand::Merge {
            first: param(params, "first")
                .or_else(|| param(params, "box_a"))
                .unwrap_or("left")
                .to_string(),
            second: param(params, "second")
                .or_else(|| param(params, "box_b"))
                .unwrap_or("right")
                .to_string(),
            result: param(params, "result")
                .unwrap_or(DEFAULT_BOX_ID)
                .to_string(),
            duration: parse_f32_param(params, "duration").unwrap_or(0.0),
        }),
        _ => None,
    }
}

/// Fact key for the currently selected player battle box.
///
/// 当前玩家绑定战斗框的 fact 键。
pub fn player_box_fact() -> &'static str {
    "ut:battle_player:box_id"
}

/// Fact key for a battle box collision region handle.
///
/// 战斗框碰撞区域句柄的 fact 键。
pub fn region_fact(id: &str) -> String {
    format!("ut:battle_box:{id}:region")
}

fn revision_fact(id: &str) -> String {
    format!("ut:battle_box:{id}:revision")
}

fn duration_fact(id: &str) -> String {
    format!("ut:battle_box:{id}:duration")
}

fn center_x_fact(id: &str) -> String {
    format!("ut:battle_box:{id}:center_x")
}

fn center_y_fact(id: &str) -> String {
    format!("ut:battle_box:{id}:center_y")
}

fn width_fact(id: &str) -> String {
    format!("ut:battle_box:{id}:width")
}

fn height_fact(id: &str) -> String {
    format!("ut:battle_box:{id}:height")
}

fn parse_bounds(params: &[(&str, &str)]) -> Option<BattleBoxBounds> {
    Some(BattleBoxBounds {
        center: Vec2::new(
            parse_f32_param(params, "center_x")?,
            parse_f32_param(params, "center_y")?,
        ),
        size: Vec2::new(
            parse_f32_param(params, "width")?,
            parse_f32_param(params, "height")?,
        ),
    })
}

fn param<'a>(params: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
    params
        .iter()
        .find_map(|(name, value)| (*name == key).then_some(*value))
}

fn parse_f32_param(params: &[(&str, &str)], key: &str) -> Option<f32> {
    param(params, key)?.parse().ok()
}

fn parse_split_axis(value: &str) -> SplitAxis {
    match value {
        "Horizontal" | "horizontal" => SplitAxis::Horizontal,
        _ => SplitAxis::Vertical,
    }
}

fn parse_gap_policy(value: &str) -> GapPolicy {
    match value {
        "Includes" | "includes" => GapPolicy::Includes,
        _ => GapPolicy::Expands,
    }
}

fn write_target_bounds(ctx: &Context, id: &str, bounds: BattleBoxBounds, duration: f32) {
    ctx.set_fact_float(&center_x_fact(id), bounds.center.x as f64);
    ctx.set_fact_float(&center_y_fact(id), bounds.center.y as f64);
    ctx.set_fact_float(&width_fact(id), bounds.size.x as f64);
    ctx.set_fact_float(&height_fact(id), bounds.size.y as f64);
    ctx.set_fact_float(&duration_fact(id), duration as f64);
    let revision = ctx.get_fact_int(&revision_fact(id)).unwrap_or(0) + 1;
    ctx.set_fact_int(&revision_fact(id), revision);
}

fn read_bounds(ctx: &Context, id: &str) -> Option<BattleBoxBounds> {
    Some(BattleBoxBounds {
        center: Vec2::new(
            read_f32(ctx, &center_x_fact(id), DEFAULT_CENTER.x),
            read_f32(ctx, &center_y_fact(id), DEFAULT_CENTER.y),
        ),
        size: Vec2::new(
            read_f32(ctx, &width_fact(id), DEFAULT_SIZE.x),
            read_f32(ctx, &height_fact(id), DEFAULT_SIZE.y),
        ),
    })
}

fn read_f32(ctx: &Context, key: &str, default: f32) -> f32 {
    ctx.get_fact_float(key)
        .map_or(default, |value| value as f32)
}

fn read_i64(ctx: &Context, key: &str, default: i64) -> i64 {
    ctx.get_fact_int(key).unwrap_or(default)
}

fn battle_box_ids(ctx: &Context) -> Vec<String> {
    ctx.get_fact_string(BOX_IDS_FACT)
        .unwrap_or_else(|| DEFAULT_BOX_ID.into())
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
        .collect()
}

fn ensure_box_id_fact(ctx: &Context, id: &str) {
    let mut ids = battle_box_ids(ctx);
    if ids.iter().any(|existing| existing == id) {
        return;
    }
    ids.push(id.to_string());
    ctx.set_fact_string(BOX_IDS_FACT, &ids.join(","));
}

fn replace_box_ids(ctx: &Context, remove: &[String], add: &[String]) {
    let mut ids: Vec<String> = battle_box_ids(ctx)
        .into_iter()
        .filter(|id| !remove.iter().any(|removed| removed == id))
        .collect();
    for id in add {
        if !ids.iter().any(|existing| existing == id) {
            ids.push(id.clone());
        }
    }
    ctx.set_fact_string(BOX_IDS_FACT, &ids.join(","));
}

fn split_bounds(
    bounds: BattleBoxBounds,
    axis: SplitAxis,
    split_position: f32,
    gap: f32,
    gap_policy: GapPolicy,
) -> (BattleBoxBounds, BattleBoxBounds) {
    match axis {
        SplitAxis::Vertical => split_vertical(bounds, split_position, gap, gap_policy),
        SplitAxis::Horizontal => split_horizontal(bounds, split_position, gap, gap_policy),
    }
}

fn split_vertical(
    bounds: BattleBoxBounds,
    split_position: f32,
    gap: f32,
    gap_policy: GapPolicy,
) -> (BattleBoxBounds, BattleBoxBounds) {
    let half = bounds.half_size();
    let left_width = (half.x + split_position).max(f32::EPSILON);
    let right_width = (half.x - split_position).max(f32::EPSILON);
    let half_gap = gap * 0.5;
    let min_x = bounds.center.x - half.x;
    let max_x = bounds.center.x + half.x;
    match gap_policy {
        GapPolicy::Expands => (
            BattleBoxBounds {
                center: Vec2::new(min_x + left_width * 0.5 - half_gap, bounds.center.y),
                size: Vec2::new(left_width, bounds.size.y),
            },
            BattleBoxBounds {
                center: Vec2::new(max_x - right_width * 0.5 + half_gap, bounds.center.y),
                size: Vec2::new(right_width, bounds.size.y),
            },
        ),
        GapPolicy::Includes => {
            let total_width = bounds.size.x;
            let scale = ((total_width - gap) / total_width).max(0.0);
            let left_scaled = left_width * scale;
            let right_scaled = right_width * scale;
            (
                BattleBoxBounds {
                    center: Vec2::new(min_x + left_scaled * 0.5, bounds.center.y),
                    size: Vec2::new(left_scaled, bounds.size.y),
                },
                BattleBoxBounds {
                    center: Vec2::new(max_x - right_scaled * 0.5, bounds.center.y),
                    size: Vec2::new(right_scaled, bounds.size.y),
                },
            )
        }
    }
}

fn split_horizontal(
    bounds: BattleBoxBounds,
    split_position: f32,
    gap: f32,
    gap_policy: GapPolicy,
) -> (BattleBoxBounds, BattleBoxBounds) {
    let half = bounds.half_size();
    let top_height = (half.y - split_position).max(f32::EPSILON);
    let bottom_height = (half.y + split_position).max(f32::EPSILON);
    let half_gap = gap * 0.5;
    let min_y = bounds.center.y - half.y;
    let max_y = bounds.center.y + half.y;
    match gap_policy {
        GapPolicy::Expands => (
            BattleBoxBounds {
                center: Vec2::new(bounds.center.x, max_y + half_gap - top_height * 0.5),
                size: Vec2::new(bounds.size.x, top_height),
            },
            BattleBoxBounds {
                center: Vec2::new(bounds.center.x, min_y - half_gap + bottom_height * 0.5),
                size: Vec2::new(bounds.size.x, bottom_height),
            },
        ),
        GapPolicy::Includes => {
            let total_height = bounds.size.y;
            let scale = ((total_height - gap) / total_height).max(0.0);
            let top_scaled = top_height * scale;
            let bottom_scaled = bottom_height * scale;
            (
                BattleBoxBounds {
                    center: Vec2::new(bounds.center.x, max_y - top_scaled * 0.5),
                    size: Vec2::new(bounds.size.x, top_scaled),
                },
                BattleBoxBounds {
                    center: Vec2::new(bounds.center.x, min_y + bottom_scaled * 0.5),
                    size: Vec2::new(bounds.size.x, bottom_scaled),
                },
            )
        }
    }
}

fn merge_bounds(first: BattleBoxBounds, second: BattleBoxBounds) -> BattleBoxBounds {
    let first_half = first.half_size();
    let second_half = second.half_size();
    let min = Vec2::new(
        (first.center.x - first_half.x).min(second.center.x - second_half.x),
        (first.center.y - first_half.y).min(second.center.y - second_half.y),
    );
    let max = Vec2::new(
        (first.center.x + first_half.x).max(second.center.x + second_half.x),
        (first.center.y + first_half.y).max(second.center.y + second_half.y),
    );
    BattleBoxBounds {
        center: (min + max) * 0.5,
        size: max - min,
    }
}
