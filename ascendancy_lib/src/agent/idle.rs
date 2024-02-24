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
pub struct Idle {
    /// The position to wander to.
    pub target: Option<Transform>, // the destination to wander to
}

/// The implementation of the `Wander` action.
impl Idle {
    /// Creates a new `Wander` action.
    pub fn new() -> Self {
        Self { target: None }
    }
}

impl Default for Idle {
    fn default() -> Self {
        Self::new()
    }
}

/// Combined system for deciding on, moving towards, and rotating towards a wander target.
pub fn idle_action_system(
    time: Res<Time>,
    solar_systems: Query<(Entity, &SystemAttributes, &Transform), Without<Agent>>,
    mut agent_query: Query<(&Agent, &mut Transform), With<Agent>>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut Idle, &ActionSpan)>,
    mut fly_to_system_query: Query<(&Agent, &mut FlyToSystem), Without<Actor>>,
) {
    let rng = rand::thread_rng();

    for (actor, mut action_state, mut idle, span) in &mut action_query {
        let _guard = span.span().enter();
        let mut agent = agent_query.get_mut(actor.0).unwrap();

        match *action_state {
            ActionState::Requested => {
                // Determine the agent's current solar system and generate a random position within it
                let current_system_id = agent.0.current_system.system.id; // Assuming Agent has a current_system_id field
                let solar_system_transform = solar_systems
                    .iter()
                    .find(|(_, system_attributes, _)| system_attributes.id == current_system_id)
                    .map(|(_, _, transform)| transform)
                    .unwrap();

                let rand_position = random_position_in_system(
                    Vec2::splat(512.),
                    solar_system_transform.translation,
                );

                // Randomly choose between flying to a new system or wandering
                let should_fly_to_system = rand::random::<bool>();

                if should_fly_to_system {
                    // Fly to a new system
                    if let Ok((_, mut fly_to_system)) = fly_to_system_query.get_mut(actor.0) {
                        fly_to_system.increase_desire(100.0);
                    }
                    *action_state = ActionState::Success;
                } else {
                    // Wander
                    idle.target = Some(Transform::from_translation(Vec3::new(
                        rand_position.x,
                        rand_position.y,
                        0.1,
                    )));
                    *action_state = ActionState::Executing;
                }
            }
            ActionState::Executing => {
                if let Some(target) = idle.target {
                    // Movement towards the target
                    let delta = target.translation - agent.1.translation;
                    let distance = delta.length();

                    if distance > MAX_DISTANCE {
                        let step_size = time.delta_seconds() * agent.0.speed;
                        let step = delta.normalize() * step_size.min(distance);
                        agent.1.translation += step;

                        // Rotation towards the target
                        let angle = delta.y.atan2(delta.x);
                        let target_rotation = Quat::from_rotation_z(angle);
                        let rotation_speed = 3.0; // Define a rotation speed
                        agent.1.rotation = agent
                            .1
                            .rotation
                            .slerp(target_rotation, rotation_speed * time.delta_seconds());
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

/// Scorers are the same as in the thirst example.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct WantToWander;

/// This is our `WantToWander` scorer system
pub fn idle_scorer_system(
    wander_targets: Query<&Idle>,
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
            score.set(0.3);
            span.span()
                .in_scope(|| debug!("Want to wander! Score: {}", 0.3));
        }
    }
}
