use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use leafwing_input_manager::Actionlike;
use self::selection::{listen_for_clicked_event, UpdateSelectedItems};

pub(crate) mod camera;
/// solar system selection module
pub mod selection;

/// All of the code needed for users to interact with the simulation.
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_event::<UpdateSelectedItems>()
            .init_resource::<ActionState<PlayerAction>>()
            .insert_resource(PlayerAction::default_input_map())
            .insert_resource(selection::Selection::new())
            .add_plugins(camera::CameraPlugin)
            .add_systems(Update, listen_for_clicked_event);
    }
}

/// Public system sets for player interaction, used for system ordering and config
#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) enum InteractionSystem {
    /// Moves the camera
    MoveCamera,
    /// Cursor position is set
    _ComputeCursorPos,
}

/// Actions that the player can take to modify the game world or their view of it.
///
/// This should only store actions that need a dedicated keybinding.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    /// Selects a tile or group of tiles.
    Select,
    /// Pause or unpause the game.
    TogglePause,
    /// When the clipboard is empty, selects a tile or group of tiles.
    Deselect,
    /// Selects a structure from a wheel menu.
    SelectStructure,
    /// Snaps the camera to the selected object
    CenterCameraOnSelection,
    /// Drag the camera with the cursor
    DragCamera,
    /// Move the camera from side to side
    Pan,
    /// Zoom in or out
    Zoom,
    /// Reveal less of the map by moving the camera closer
    ZoomIn,
    /// Reveal more of the map by pulling the camera away
    ZoomOut,
    /// Rotates the camera counterclockwise
    RotateCameraLeft,
    /// Rotates the camera clockwise
    RotateCameraRight,
    ///Reset the camera position back to 0,0,0 and 0 rotation
    ResetCameraPosition,
}

impl PlayerAction {
    /// The default keybindings for mouse and keyboard.
    fn kbm_binding(&self) -> UserInput {
        match self {
            Self::Select => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
            Self::TogglePause => UserInput::Single(InputKind::PhysicalKey(KeyCode::Space)),
            Self::Deselect => UserInput::Single(InputKind::Mouse(MouseButton::Right)),
            Self::SelectStructure => UserInput::Single(InputKind::PhysicalKey(KeyCode::Digit1)),
            Self::CenterCameraOnSelection => UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyL)),
            Self::DragCamera => UserInput::Single(InputKind::Mouse(MouseButton::Middle)),
            Self::Pan => UserInput::Single(InputKind::DualAxis(DualAxis::mouse_motion())),
            // Plus and Equals are swapped. See:
            Self::ZoomIn => UserInput::Single(InputKind::MouseWheel(MouseWheelDirection::Up)),
            Self::ZoomOut => UserInput::Single(InputKind::MouseWheel(MouseWheelDirection::Down)),

            Self::Zoom => UserInput::Single(InputKind::SingleAxis(SingleAxis::mouse_wheel_y())),

            Self::RotateCameraLeft => KeyCode::KeyQ.into(),
            Self::RotateCameraRight => KeyCode::KeyE.into(),
            Self::ResetCameraPosition => KeyCode::KeyR.into(),
        }
    }

    /// The default key bindings
    fn default_input_map() -> InputMap<PlayerAction> {
        let mut input_map = InputMap::default();
    
        input_map.insert(Self::Select, Self::Select.kbm_binding());
        input_map.insert(Self::TogglePause, Self::TogglePause.kbm_binding());
        input_map.insert(Self::Deselect, Self::Deselect.kbm_binding());
        input_map.insert(Self::SelectStructure, Self::SelectStructure.kbm_binding());
        input_map.insert(Self::CenterCameraOnSelection, Self::CenterCameraOnSelection.kbm_binding());
        input_map.insert(Self::DragCamera, Self::DragCamera.kbm_binding());
        input_map.insert(Self::Pan, Self::Pan.kbm_binding());
        input_map.insert(Self::Zoom, Self::Zoom.kbm_binding());
        input_map.insert(Self::ZoomIn, Self::ZoomIn.kbm_binding());
        input_map.insert(Self::ZoomOut, Self::ZoomOut.kbm_binding());
        input_map.insert(Self::RotateCameraLeft, Self::RotateCameraLeft.kbm_binding());
        input_map.insert(Self::RotateCameraRight, Self::RotateCameraRight.kbm_binding());
        input_map.insert(Self::ResetCameraPosition, Self::ResetCameraPosition.kbm_binding());
        // Return the input_map
        input_map
    }
}