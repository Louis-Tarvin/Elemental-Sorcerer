use bevy::asset::{AssetServer, HandleUntyped};
use bevy::ecs::world::{Mut, World};
use bevy::prelude::{Handle, Image};
use bevy::text::Font;
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection)]
pub struct MenuAssets {
    #[asset(path = "fonts/prstartk.ttf")]
    pub pixel_font: Handle<Font>,
    #[asset(path = "tilesets/background.png")]
    pub background: Handle<Image>,
    #[asset(path = "sprites/button.png")]
    pub button: Handle<Image>,
}
