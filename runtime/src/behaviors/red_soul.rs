//! Red Soul behavior - standard movement mode
//! 红魂行为 - 标准移动模式

use souprune_sdk::prelude::*;

use super::super::battle_box::{player_box_fact, region_fact};

const DEFAULT_BOX_ID: &str = "main";
const DEFAULT_COLLIDER_RADIUS: f32 = 8.0;

pub struct RedSoul {
    speed: f32,
    focus_ratio: f32,
    movement_constraint: Option<PlayerMovementConstraint>,
}

#[derive(Debug, Clone, Copy)]
struct PlayerMovementConstraint {
    region: RegionHandle,
    constraint: ConstraintHandle,
}

impl RedSoul {
    pub fn new() -> Self {
        Self {
            speed: 150.0,
            focus_ratio: 0.5,
            movement_constraint: None,
        }
    }

    fn constrained_velocity(&mut self, context: &mut Context, velocity: Vec2, dt: f32) -> Vec2 {
        if dt <= f32::EPSILON {
            return velocity;
        }
        let current = context.entity_position();
        let desired = current + velocity * dt;
        let Some(constraint) = self.ensure_movement_constraint(context) else {
            return velocity;
        };
        let Some(constrained) = context
            .collision()
            .constrain_movement(constraint.constraint, desired)
        else {
            self.clear_movement_constraint(context);
            return velocity;
        };
        Vec2::new(
            (constrained.x - current.x) / dt,
            (constrained.y - current.y) / dt,
        )
    }

    fn ensure_movement_constraint(
        &mut self,
        context: &mut Context,
    ) -> Option<PlayerMovementConstraint> {
        let box_id = context
            .get_fact_string(player_box_fact())
            .unwrap_or_else(|| DEFAULT_BOX_ID.into());
        let raw_region = context.get_fact_int(&region_fact(&box_id))?;
        let region = RegionHandle(raw_region as u64);
        if let Some(current) = self.movement_constraint
            && current.region == region
        {
            return Some(current);
        }

        self.clear_movement_constraint(context);
        let constraint = context.collision().create_movement_constraint(
            region,
            ColliderShape::Circle {
                radius: DEFAULT_COLLIDER_RADIUS,
            },
        )?;
        let next = PlayerMovementConstraint { region, constraint };
        self.movement_constraint = Some(next);
        Some(next)
    }

    fn clear_movement_constraint(&mut self, context: &mut Context) {
        if let Some(current) = self.movement_constraint.take() {
            context
                .collision()
                .remove_movement_constraint(current.constraint);
        }
    }
}

impl Default for RedSoul {
    fn default() -> Self {
        Self::new()
    }
}

impl Behavior for RedSoul {
    fn on_enter(&mut self, context: &mut Context) {
        context.log("Red Soul Mode Activated!");
    }

    fn on_update(&mut self, context: &mut Context, dt: f32) {
        let mut velocity_x: f32 = 0.0;
        let mut velocity_y: f32 = 0.0;

        let input = context.input();

        if input.pressed(Action::Left) {
            velocity_x -= 1.0;
        }
        if input.pressed(Action::Right) {
            velocity_x += 1.0;
        }
        if input.pressed(Action::Up) {
            velocity_y += 1.0;
        }
        if input.pressed(Action::Down) {
            velocity_y -= 1.0;
        }

        // Normalize vector if moving diagonally
        if velocity_x != 0.0 || velocity_y != 0.0 {
            let length = (velocity_x * velocity_x + velocity_y * velocity_y).sqrt();
            velocity_x /= length;
            velocity_y /= length;

            let mut current_speed = self.speed;

            // Check for Focus (Cancel button usually maps to Shift/X)
            if input.pressed(Action::Cancel) {
                current_speed *= self.focus_ratio;
            }

            velocity_x *= current_speed;
            velocity_y *= current_speed;
        }

        let velocity = self.constrained_velocity(context, Vec2::new(velocity_x, velocity_y), dt);
        context.kinematics().set_velocity(velocity.x, velocity.y);
    }

    fn on_exit(&mut self, context: &mut Context) {
        context.log("Red Soul Mode Deactivated.");
        self.clear_movement_constraint(context);
        context.kinematics().set_velocity(0.0, 0.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::battle_box::{BattleBoxBounds, BattleBoxCommand, parse_battle_box_command};
    use souprune_sdk::prelude::Vec2;

    #[test]
    fn parses_set_bounds_custom_action_params() {
        let params = [
            ("id", "main"),
            ("center_x", "-0.5"),
            ("center_y", "-80.0"),
            ("width", "565.0"),
            ("height", "130.0"),
            ("duration", "0.85"),
        ];

        let command =
            parse_battle_box_command("SetBattleBoxBounds", &params).expect("set bounds command");

        assert_eq!(
            command,
            BattleBoxCommand::SetBounds {
                id: "main".into(),
                bounds: BattleBoxBounds {
                    center: Vec2::new(-0.5, -80.0),
                    size: Vec2::new(565.0, 130.0),
                },
                duration: 0.85,
            }
        );
    }

    #[test]
    fn battle_box_bounds_interpolate_center_and_size() {
        let start = BattleBoxBounds {
            center: Vec2::new(-0.5, -80.0),
            size: Vec2::new(565.0, 130.0),
        };
        let end = BattleBoxBounds {
            center: Vec2::new(-0.5, -55.0),
            size: Vec2::new(175.0, 180.0),
        };

        let mid = start.lerp(end, 0.5);

        assert_eq!(mid.center, Vec2::new(-0.5, -67.5));
        assert_eq!(mid.size, Vec2::new(370.0, 155.0));
    }
}
