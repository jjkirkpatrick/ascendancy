use crate::solar_system::SolarSystem;

use crate::structures::stargate::Stargate;
use bevy::prelude::*;
use big_brain::prelude::*;

use super::{agent::Agent, pathfinding::SystemGraph};

/// The distance required to jump a stargate
const DISTANCE_REQUIRED_TO_JUMP_STARGATE: f32 = 0.1;

/// An action where the actor moves to the closest water source
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct FlyToSystem {
    /// The position to wander to.
    pub target: Option<SolarSystem>,

    /// The Desire to  travel to a system
    pub desire: f32,
}

impl Default for FlyToSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// The implementation of the `Wander` action.
impl FlyToSystem {
    /// Creates a new `Wander` action.
    pub fn new() -> Self {
        Self {
            target: None,
            desire: 0.0,
        }
    }

    /// Increases the `FlyToSystem` desire.
    pub fn increase_desire(&mut self, increase_by: f32) {
        self.desire += increase_by;
    }

    /// reset desire back to 0
    pub fn reset_desire(&mut self) {
        self.desire = 0.0;
    }
}

/// The decision system for the `Wander` action.
pub fn fly_to_system(
    time: Res<Time>,
    mut action_query: Query<(&Actor, &mut ActionState, &ActionSpan), With<FlyToSystem>>,
    mut fly_to_system_query: Query<(&mut Agent, &mut FlyToSystem, &mut Transform)>,
    system_graph: Res<SystemGraph>,
    star_gates: Query<(&Stargate, &Transform), Without<FlyToSystem>>,
    solar_systems: Query<(&SolarSystem, &Transform), Without<Agent>>,
) {
    for (actor, mut action_state, span) in &mut action_query {
        let _guard = span.span().enter();
        match *action_state {
            ActionState::Requested => {
                let (mut agent, _, _) = fly_to_system_query.get_mut(actor.0).unwrap();
                let current_system = &agent.current_system;

                match system_graph.get_pathfinding_to_random_system(current_system) {
                    Ok(path) => {
                        if !path.is_empty() {
                            agent.set_stargate_path(path);
                        }
                    }
                    Err(_) => {
                        *action_state = ActionState::Failure;
                    }
                }

                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (mut agent, _, mut transform) = fly_to_system_query.get_mut(actor.0).unwrap();
                let first_stargate_opt = agent.stargate_path.path.first().cloned(); // Clone the first stargate

                if let Some(first_stargate) = first_stargate_opt {
                    let matching_stargate_transform = star_gates
                        .iter()
                        .find(|(gate, _)| {
                            gate.origin_system_id() == first_stargate.origin_system_id()
                                && gate.destination_system_id()
                                    == first_stargate.destination_system_id()
                        })
                        .map(|(_, transform)| transform)
                        .unwrap();

                    agent.target_destination = Some(matching_stargate_transform.translation);

                    if let Some(target_destination) = agent.target_destination {
                        let delta = target_destination - transform.translation;
                        let distance = delta.length();

                        if distance > DISTANCE_REQUIRED_TO_JUMP_STARGATE {
                            let step_size = time.delta_seconds() * agent.speed;
                            let step = delta.normalize() * step_size.min(distance);
                            transform.translation += step;
                        } else {
                            // Find the destination stargate's transform (not the destination solar system's transform)
                            let destination_stargate_transform = star_gates
                                .iter()
                                .find(|(gate, _)| {
                                    gate.origin_system_id()
                                        == first_stargate.destination_system_id()
                                        && gate.destination_system_id()
                                            == first_stargate.origin_system_id()
                                })
                                .map(|(_, transform)| transform)
                                .unwrap();

                            // Teleport the agent to the destination stargate
                            transform.translation = destination_stargate_transform.translation;

                            // Update the agent's current system
                            agent.current_system = solar_systems
                                .iter()
                                .find(|(system, _)| {
                                    system.attributes.id == first_stargate.destination_system_id()
                                })
                                .map(|(system, _)| system)
                                .unwrap()
                                .clone();

                            // Remove the stargate from the path
                            agent.stargate_path.path.remove(0);

                            // Check if there are more stargates in the path. If not, set the action state to Success.
                            if agent.stargate_path.path.is_empty() {
                                if let Ok((_, mut fly_to_system, _)) =
                                    fly_to_system_query.get_mut(actor.0)
                                {
                                    fly_to_system.reset_desire();
                                }
                                *action_state = ActionState::Success;
                            }
                        }
                    }
                } else {
                    if let Ok((_, mut fly_to_system, _)) = fly_to_system_query.get_mut(actor.0) {
                        fly_to_system.reset_desire();
                    }
                    *action_state = ActionState::Failure
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
pub struct WantToFlyToSystem;

/// This is our `WantToFlyToSystem` scorer system
pub fn want_to_fly_to_system_scorer_system(
    fly_to_system_targets: Query<&FlyToSystem>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<WantToFlyToSystem>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(fly_to_system) = fly_to_system_targets.get(*actor) {
            let desire = fly_to_system.desire / 100.0;
            // clamp the desire between 0.0 and 1.0
            let desire = desire.max(0.0).min(1.0);
            score.set(desire);
            span.span()
                .in_scope(|| debug!("Want to wander! Score: {}", 0.8));
        }
    }
}
