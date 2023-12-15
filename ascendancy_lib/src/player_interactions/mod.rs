use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

use leafwing_input_manager::{
    prelude::{ActionState, DualAxis, InputManagerPlugin, InputMap},
    user_input::UserInput,
    Actionlike,
};

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
            Self::TogglePause => UserInput::Single(InputKind::Keyboard(KeyCode::Space)),
            Self::Deselect => UserInput::Single(InputKind::Mouse(MouseButton::Right)),
            Self::SelectStructure => UserInput::Single(InputKind::Keyboard(KeyCode::Key1)),
            Self::CenterCameraOnSelection => UserInput::Single(InputKind::Keyboard(KeyCode::L)),
            Self::DragCamera => UserInput::Single(InputKind::Mouse(MouseButton::Middle)),
            Self::Pan => UserInput::Single(InputKind::DualAxis(DualAxis::mouse_motion())),
            // Plus and Equals are swapped. See:
            Self::ZoomIn => UserInput::Single(InputKind::MouseWheel(MouseWheelDirection::Up)),
            Self::ZoomOut => UserInput::Single(InputKind::MouseWheel(MouseWheelDirection::Down)),

            Self::Zoom => UserInput::Single(InputKind::SingleAxis(SingleAxis::mouse_wheel_y())),

            Self::RotateCameraLeft => KeyCode::Q.into(),
            Self::RotateCameraRight => KeyCode::E.into(),
            Self::ResetCameraPosition => KeyCode::R.into(),
        }
    }

    /// The default key bindings
    fn default_input_map() -> InputMap<PlayerAction> {
        let mut input_map = InputMap::default();

        for variant in PlayerAction::variants() {
            input_map.insert(variant.kbm_binding(), variant);
        }
        input_map
    }
}
