use bevy::asset::{AssetServer, HandleUntyped};
use bevy::ecs::world::{Mut, World};
use bevy::prelude::{Handle, Image, Commands, NodeBundle, BuildChildren, TextBundle, Res, Color, Component, Query, Entity, With, DespawnRecursiveExt};
use bevy::text::{Font, TextStyle};
use bevy::ui::{Style, UiRect, Val, Size, JustifyContent, AlignItems, FlexDirection};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::LdtkAsset;

use super::load_menu::MenuAssets;

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

#[derive(Component)]
pub struct LoadingNode;

pub fn setup(mut commands: Commands, menu_assets: Res<MenuAssets>) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
                margin: UiRect::all(Val::Auto),
                padding: UiRect::all(Val::Px(20.)),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
        ..Default::default()
    })
    .insert(LoadingNode)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "Loading...",
            TextStyle {
                font: menu_assets.pixel_font.clone(),
                font_size: 40.0,
                color: Color::BLACK,
            }
        ));
    });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<LoadingNode>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
