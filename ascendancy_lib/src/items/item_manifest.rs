//! Defines write-only data for each variety of item.

use bevy::{
    asset::Asset,
    reflect::{Reflect, TypePath},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

use crate::asset_management::manifest::{loader::IsRawManifest, Id, Manifest};

use super::item_tags::{ItemKind, ItemTag};

/// The marker type for [`Id<Item>`](super::Id).
#[derive(Reflect, Clone, Copy, PartialEq, Eq)]
pub struct Item;
/// Stores the read-only definitions for all items.
pub type ItemManifest = Manifest<Item, ItemData>;

impl ItemManifest {
    /// Does the provided `item_id` meet the requirements of the given `tag`?
    pub fn has_tag(&self, item_id: Id<Item>, tag: ItemTag) -> bool {
        let data = self.get(item_id);

        match tag {
            ItemTag::Fluid => data.fluid,
            ItemTag::Category => todo!(),
        }
    }

    /// Returns the complete list of tags that the given item belongs to.
    pub fn tags(&self, item_id: Id<Item>) -> Vec<ItemTag> {
        let data = self.get(item_id);

        let tags = Vec::new();

        tags
    }

    /// Returns the complete list of [`ItemKind`] that this item belongs to.
    pub fn kinds(&self, item_id: Id<Item>) -> Vec<ItemKind> {
        let mut kinds = Vec::new();
        kinds.push(ItemKind::Single(item_id));

        for tag in self.tags(item_id) {
            kinds.push(ItemKind::Tag(tag));
        }

        kinds
    }

    /// Returns the complete list of [`ItemKind`] that match the given `tag`.
    pub fn kinds_with_tag(&self, tag: ItemTag) -> Vec<ItemKind> {
        let mut kinds = Vec::new();

        for item_id in self.variants() {
            if self.has_tag(item_id, tag) {
                kinds.push(ItemKind::Single(item_id));
            }
        }

        kinds.push(ItemKind::Tag(tag));

        kinds
    }

    /// Returns the human-readable name associated with the provided `item_kind`.
    ///
    /// # Panics
    /// This function panics when the given ID does not exist in the manifest.
    /// We assume that all IDs are valid and the manifests are complete.
    pub fn name_of_kind(&self, item_kind: ItemKind) -> &str {
        match item_kind {
            ItemKind::Single(id) => self.name(id),
            ItemKind::Tag(tag) => tag.name(),
        }
    }
}

/// The data associated with each item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemData {
    /// the mass of the item
    pub volume_per_unit: u32,
    /// Is this item a fluid?
    pub fluid: bool,
    /// The category of the item.
    pub category: String,
}

/// The unprocessed [`ItemData`] as seen in the manifest file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawItemData {
    /// the mass of the item
    pub volume_per_unit: u32,
    /// Is this item a fluid?
    pub fluid: bool,
    /// The category of the item.
    pub category: String,
}

impl From<RawItemData> for ItemData {
    fn from(raw: RawItemData) -> Self {
        Self {
            volume_per_unit: raw.volume_per_unit,
            fluid: raw.fluid,
            category: raw.category,
        }
    }
}

/// The [`ItemManifest`] as seen in the manifest file.
#[derive(Asset, Debug, Clone, Serialize, Deserialize, TypePath, PartialEq)]
pub struct RawItemManifest {
    /// The data for each item.
    pub items: HashMap<String, RawItemData>,
}

impl IsRawManifest for RawItemManifest {
    const EXTENSION: &'static str = "item_manifest.json";

    type Marker = Item;
    type Data = ItemData;

    fn process(&self) -> Manifest<Self::Marker, Self::Data> {
        let mut manifest = Manifest::new();

        for (raw_id, raw_data) in self.items.clone() {
            let data = ItemData::from(raw_data);

            manifest.insert(raw_id, data)
        }

        manifest
    }
}
