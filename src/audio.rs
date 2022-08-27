use bevy::asset::{AssetServer, HandleUntyped};
use bevy::ecs::world::{Mut, World};
use bevy::prelude::Handle;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/bgm.wav")]
    pub bgm: Handle<AudioSource>,
    #[asset(path = "audio/jump.wav")]
    pub jump: Handle<AudioSource>,
    #[asset(path = "audio/death.wav")]
    pub death: Handle<AudioSource>,
    #[asset(path = "audio/explosion.wav")]
    pub explosion: Handle<AudioSource>,
    #[asset(path = "audio/collect.wav")]
    pub collect: Handle<AudioSource>,
    #[asset(path = "audio/fireball.wav")]
    pub fireball: Handle<AudioSource>,
    #[asset(path = "audio/air.wav")]
    pub air: Handle<AudioSource>,
    #[asset(path = "audio/steam.wav")]
    pub steam: Handle<AudioSource>,
    #[asset(path = "audio/pew.wav")]
    pub pew: Handle<AudioSource>,
    #[asset(path = "audio/hurt.wav")]
    pub hurt: Handle<AudioSource>,
    #[asset(path = "audio/ping.wav")]
    pub ping: Handle<AudioSource>,
    #[asset(path = "audio/blip1.wav")]
    pub blip1: Handle<AudioSource>,
    #[asset(path = "audio/blip2.wav")]
    pub blip2: Handle<AudioSource>,
}
