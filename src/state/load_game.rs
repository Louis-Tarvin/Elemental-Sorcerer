use bevy::asset::{AssetServer, HandleUntyped};
use bevy::ecs::world::{Mut, World};
use bevy::prelude::{Handle, Image};
use bevy::text::Font;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::LdtkAsset;

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "levels/level.ldtk")]
    pub level: Handle<LdtkAsset>,
    #[asset(path = "fonts/prstartk.ttf")]
    pub pixel_font: Handle<Font>,
    #[asset(path = "sprites/menu_background.png")]
    pub menu_background: Handle<Image>,
    #[asset(path = "sprites/button.png")]
    pub button: Handle<Image>,
    #[asset(path = "sprites/explosion.png")]
    pub explosion: Handle<Image>,
    #[asset(path = "sprites/poof.png")]
    pub poof: Handle<Image>,
    #[asset(path = "sprites/fireball.png")]
    pub fireball: Handle<Image>,
    #[asset(path = "sprites/wind.png")]
    pub wind: Handle<Image>,
    #[asset(path = "sprites/droplet.png")]
    pub droplet: Handle<Image>,
}
