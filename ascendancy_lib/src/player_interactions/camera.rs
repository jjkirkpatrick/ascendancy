//! Code needed to run the game camera

use self::speed::Speed;
use super::InteractionSystem;
use super::PlayerAction;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
/// Camera logic
use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

/// The plugin that handles camera movement.
pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            (
                zoom_camera,
                pan_camera.before(InteractionSystem::MoveCamera),
                rotate_camera,
                reset_camera_positon,
            ),
        );
    }
}

/// The maximum amount of time that can be treated as a single frame.
///
/// This prevents the camera from moving too far in a single frame when the game is lagging.
const MAX_FRAME_TIME: f32 = 1. / 20.;

/// Spawns a [`Camera2dBundle`] and associated camera components.
fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let settings = CameraSettings::default();
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 200.0),
            ..default()
        },
        settings,
        InputManagerBundle::<PlayerAction> {
            input_map: PlayerAction::default_input_map(),
            action_state: ActionState::default(),
        },
        //RaycastPickCamera::default(),
    ));
}

/// Configure how the camera moves and feels.
#[derive(Component)]
pub(crate) struct CameraSettings {
    /// How should this camera behave?
    pub(crate) camera_mode: CameraMode,
    /// Controls how fast the camera zooms in and out.
    zoom_speed: Speed,
    /// Controls the rate that the camera can moves from side to side.
    pan_speed: Speed,
    /// Controls how fast the camera rotates around the vertical axis.
    ///
    /// Units are in radians per second.
    rotation_speed: Speed,
    /// The minimum distance that the camera can be from its focus.
    ///
    /// Should always be positive, and less than `max_zoom`.
    pub(crate) min_zoom: f32,
    /// The maximum distance that the camera can be from its focus.
    ///
    /// Should always be positive, and less than `max_zoom`.
    pub(crate) max_zoom: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            camera_mode: CameraMode::Free,
            zoom_speed: Speed::new(0.5, 0.5, 50.0),
            pan_speed: Speed::new(20., 300.0, 200.0),
            rotation_speed: Speed::new(1.0, 2.0, 4.0),
            min_zoom: 1.,
            max_zoom: 30.,
        }
    }
}

/// Controls how the camera moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CameraMode {
    /// The camera is free to move around the map.
    Free,
    /// The camera is following the selected unit.
    _FollowUnit,
}

/// Contains the [`Speed`] struct.
///
/// Lives in a dedicated module to enforce privacy.
mod speed {
    use bevy::utils::Duration;

    use super::MAX_FRAME_TIME;

    /// Controls the rate of camera movement.
    ///
    /// Minimum speed is greater than
    pub(super) struct Speed {
        /// The minimum speed, in units per second
        min: f32,
        /// The current speed, in units per second
        current_speed: f32,
        /// The rate at which speed change, in units per second squared
        acceleration: f32,
        /// The maximum speed, in units per second
        max: f32,
    }

    impl Speed {
        /// Creates a new [`Speed`]
        ///
        /// # Panics
        ///
        /// Improper parameters will panic on construction.
        pub(super) fn new(min: f32, acceleration: f32, max: f32) -> Self {
            assert!(min > 0.);
            assert!(acceleration > 0.);
            assert!(max > 0.);

            assert!(min <= max);

            Speed {
                min,
                current_speed: min,
                acceleration,
                max,
            }
        }

        /// The amount that has changed in the elapsed `delta_time`.
        pub(super) fn delta(&mut self, delta_time: Duration) -> f32 {
            let delta_time = delta_time.as_secs_f32().min(MAX_FRAME_TIME);

            let delta_v = self.acceleration * delta_time;
            let proposed = self.current_speed + delta_v;
            self.current_speed = proposed.clamp(self.min, self.max);

            self.current_speed * delta_time
        }

        /// Resets the current speed to the minimum value
        pub(super) fn reset_speed(&mut self) {
            self.current_speed = self.min;
        }
    }
}

/// Moves the camera around the map.
fn pan_camera(
    mut query: Query<
        (
            &mut Transform,
            &mut CameraSettings,
            &ActionState<PlayerAction>,
        ),
        With<Camera2d>,
    >,
    time: Res<Time>, // We'll need to introduce the Time resource to use delta time
) {
    let (mut camera_transform, mut camera_settings, action_state) = query.single_mut();

    // Only pan if PlayerAction::Select is active
    if action_state.pressed(PlayerAction::Select) {
        camera_settings.camera_mode = CameraMode::Free; // Set to Free mode when panning

        let camera_pan_vector = action_state.axis_pair(PlayerAction::Pan).unwrap();

        // Calculate speed influenced by delta time and acceleration
        let scaled_speed = camera_settings.pan_speed.delta(time.delta());

        // Because we're moving the camera, not the object, we want to pan in the opposite direction.
        // However, UI coordinates are inverted on the y-axis, so we need to flip y a second time.
        camera_transform.translation.x -= scaled_speed * camera_pan_vector.x();
        camera_transform.translation.y += scaled_speed * camera_pan_vector.y();
    } else {
        camera_settings.pan_speed.reset_speed();
    }
}

/// Zooms the camera in and out.
fn zoom_camera(
    mut query: Query<
        (
            &mut OrthographicProjection,
            &mut CameraSettings,
            &ActionState<PlayerAction>,
        ),
        With<Camera2d>,
    >,
    time: Res<Time>,
) {
    let (mut camera_projection, mut camera_settings, action_state) = query.single_mut();
    // Here, we use the `action_value` method to extract the total net amount that the mouse wheel has travelled
    // Up and right axis movements are always positive by default
    let zoom_delta = action_state.value(PlayerAction::Zoom);

    // We want to zoom in when we use mouse wheel up
    // so we increase the scale proportionally
    // Note that the projections scale should always be positive (or our images will flip)
    let new_scale = camera_projection.scale
        * (1. - zoom_delta * camera_settings.zoom_speed.delta(time.delta()));
    // Ensure the new scale is within the min and max zoom levels
    camera_projection.scale = new_scale.clamp(camera_settings.min_zoom, camera_settings.max_zoom);
}

/// Rotates the camera around the map.
fn rotate_camera(
    mut query: Query<
        (
            &mut Transform,
            &mut CameraSettings,
            &ActionState<PlayerAction>,
        ),
        With<Camera2d>,
    >,
    time: Res<Time>,
) {
    let (mut camera_transform, mut camera_settings, action_state) = query.single_mut();

    // match PlayerAction::RotateCameraLeft and PlayerAction::RotateCameraLeft

    match (
        action_state.pressed(PlayerAction::RotateCameraLeft),
        action_state.pressed(PlayerAction::RotateCameraRight),
    ) {
        (true, false) => {
            camera_settings.camera_mode = CameraMode::Free;
            let rotation_delta = camera_settings.rotation_speed.delta(time.delta());
            camera_transform.rotate(Quat::from_rotation_z(rotation_delta));
        }
        (false, true) => {
            camera_settings.camera_mode = CameraMode::Free;
            let rotation_delta = camera_settings.rotation_speed.delta(time.delta());
            camera_transform.rotate(Quat::from_rotation_z(-rotation_delta));
        }
        _ => {
            camera_settings.rotation_speed.reset_speed();
        }
    }
}

/// Resets the camera to its default position.
fn reset_camera_positon(
    mut query: Query<
        (
            &mut Transform,
            &mut CameraSettings,
            &ActionState<PlayerAction>,
        ),
        With<Camera2d>,
    >,
) {
    let (mut camera_transform, mut camera_settings, action_state) = query.single_mut();

    if action_state.pressed(PlayerAction::ResetCameraPosition) {
        camera_settings.camera_mode = CameraMode::Free;
        camera_transform.translation = Vec3::new(0., 0., 200.);
        camera_transform.rotation = Quat::from_rotation_z(0.);
    }
}
