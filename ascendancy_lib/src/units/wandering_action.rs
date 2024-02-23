use crate::{
    solar_system::attributes::SystemAttributes,
    world_gen::npc_generation::random_position_in_system,
};
use bevy::prelude::*;
use big_brain::prelude::*;

use super::{agent::Agent, fly_to_system_action::FlyToSystem};

/// The maximum distance to the target before the action is considered a success.
const MAX_DISTANCE: f32 = 0.1;

/// An action where the actor moves to the closest water source
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Wander {
    /// The position to wander to.
    pub target: Option<Transform>, // the destination to wander to
}

/// The implementation of the `Wander` action.
impl Wander {
    /// Creates a new `Wander` action.
    pub fn new() -> Self {
        Self { target: None }
    }
}

impl Default for Wander {
    fn default() -> Self {
        Self::new()
    }
}

/// The decision system for the `Wander` action.
pub fn wander_decision_system(
    solar_systems: Query<(Entity, &SystemAttributes, &Transform)>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut Wander, &ActionSpan)>,
    position: Query<(&Agent, &Transform), With<Agent>>,
) {
    for (actor, mut action_state, mut wander, span) in &mut action_query {
        let _guard = span.span().enter();
        let agent = position.get(actor.0).unwrap();

        let current_system_id = agent.0.current_system.system.id;
        let solar_system_transform = solar_systems
            .iter()
            .find(|(_, system_attributes, _)| system_attributes.id == current_system_id)
            .map(|(_, _, transform)| transform)
            .unwrap();

        if *action_state == ActionState::Requested {
            let mut rand_position: Vec3 =
                random_position_in_system(Vec2::splat(512.), solar_system_transform.translation);
            rand_position.z = 0.1;
            let transform = Transform::from_translation(rand_position);
            wander.target = Some(transform);
            *action_state = ActionState::Executing;
        }
    }
}

/// The movement system for the `Wander` action.
pub fn wander_movement_system(
    time: Res<Time>,
    mut agent: Query<(&Agent, &mut Transform), With<Agent>>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut Wander, &ActionSpan)>,
    mut fly_to_system_query: Query<(&Agent, &mut FlyToSystem), Without<Actor>>,
) {
    for (actor, mut action_state, wander, span) in &mut action_query {
        let _guard = span.span().enter();
        let mut agent = agent.get_mut(actor.0).unwrap();

        match *action_state {
            ActionState::Executing => {
                if let Some(target) = wander.target {
                    let delta = target.translation - agent.1.translation;
                    let distance = delta.length();

                    if distance > MAX_DISTANCE {
                        let step_size = time.delta_seconds() * agent.0.speed;
                        let step = delta.normalize() * step_size.min(distance);
                        agent.1.translation += step;
                    } else {
                        if let Ok((_, mut fly_to_system)) = fly_to_system_query.get_mut(actor.0) {
                            fly_to_system.increase_desire(15.0);
                        }
                        *action_state = ActionState::Success;
                    }
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

/// The rotation system for the `Wander` action.
pub fn wander_rotation_system(
    time: Res<Time>,
    mut position: Query<(&Agent, &mut Transform), With<Agent>>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut Wander, &ActionSpan)>,
) {
    for (actor, action_state, wander, span) in &mut action_query {
        let _guard = span.span().enter();
        let mut agent = position.get_mut(actor.0).unwrap();

        if *action_state == ActionState::Executing {
            if let Some(target) = wander.target {
                let direction = target.translation - agent.1.translation;
                let angle = direction.y.atan2(direction.x); // Adjust the angle by -90 degrees
                let target_rotation = Quat::from_rotation_z(angle);
                let rotation_speed = 3.0; // Define a rotation speed
                                          // Interpolate the rotation over time for a smooth transition
                agent.1.rotation = agent
                    .1
                    .rotation
                    .slerp(target_rotation, rotation_speed * time.delta_seconds());
            }
        }
    }
}

/// Scorers are the same as in the thirst example.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct WantToWander;

/// This is our `WantToWander` scorer system
pub fn want_to_wander_scorer_system(
    wander_targets: Query<&Wander>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<WantToWander>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if wander_targets.get(*actor).is_ok() {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            // In this case, we're just setting a constant score as the desire to wander is always present.
            score.set(0.8);
            span.span()
                .in_scope(|| debug!("Want to wander! Score: {}", 1.0));
        }
    }
}
