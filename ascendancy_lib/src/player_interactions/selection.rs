use bevy::prelude::*;
use bevy_mod_picking::{
    prelude::{Down, ListenerInput, On, Pointer},
    selection::PickSelection,
};

use crate::solar_system::attributes::SystemAttributes;

/// The currently selected entity.
#[derive(Resource)]
pub struct Selection {
    /// The currently selected entity.
    pub selected: Vec<SystemAttributes>,
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

impl Selection {
    /// Creates a new `Selection` resource.
    pub fn new() -> Self {
        Self {
            selected: Vec::new(),
        }
    }

    /// Sets the selected entity.
    pub fn set(&mut self, entity: SystemAttributes) {
        self.selected.push(entity);
    }

    /// Removes the selected entity.
    pub fn remove(&mut self, entity: SystemAttributes) {
        self.selected.retain(|x| *x != entity);
    }

    /// Clears the selected entity.
    pub fn clear(&mut self) {
        self.selected.clear();
    }

    /// Returns the selected entity.
    pub fn get_all(&self) -> Vec<SystemAttributes> {
        self.selected.clone()
    }

    /// Returns the selected entity.
    pub fn count(&self) -> usize {
        self.selected.len()
    }

    /// Get selection by index
    pub fn get(&self, index: usize) -> Option<SystemAttributes> {
        if index < self.selected.len() {
            Some(self.selected[index].clone())
        } else {
            None
        }
    }
}

/// Listens for the `PointerDown` event and prints the entity that was clicked.
pub fn listen_for_clicked_event(
    mut selection: ResMut<Selection>,
    mut query: Query<(&PickSelection, &SystemAttributes), Changed<PickSelection>>,
) {
    for (pick_selection, system_attributes) in query.iter_mut() {
        On::<Pointer<Down>>::send_event::<UpdateSelectedItemEvent>();
        if pick_selection.is_selected {
            selection.set(system_attributes.clone());
        } else {
            selection.remove(system_attributes.clone());
        }
    }
}

/// An event that is triggered when the user clicks on an entity.
#[derive(Event, Debug)]
pub struct UpdateSelectedItemEvent(pub Entity);

impl From<ListenerInput<Pointer<Down>>> for UpdateSelectedItemEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        let update_selected_items = UpdateSelectedItemEvent(event.target);
        println!("UpdateSelectedItems: Entity: {:?}", update_selected_items.0);
        update_selected_items
    }
}

impl UpdateSelectedItemEvent {
    /// Prints the current state of `UpdateSelectedItems` for debugging purposes.
    pub fn debug(&self) {
        println!("UpdateSelectedItems: {:?}", self);
    }
}
