use bevy::prelude::{AssetServer, Commands, Handle, Res};
use bevy_kira_audio::AudioSource;

pub struct AudioManager {
    pub bgm_volume: f64,
    pub bgm: Handle<AudioSource>,
    pub jump: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
    pub collect: Handle<AudioSource>,
    pub fireball: Handle<AudioSource>,
    pub air: Handle<AudioSource>,
    pub steam: Handle<AudioSource>,
    pub pew: Handle<AudioSource>,
    pub hurt: Handle<AudioSource>,
    pub ping: Handle<AudioSource>,
    pub blip1: Handle<AudioSource>,
    pub blip2: Handle<AudioSource>,
}
impl AudioManager {
    pub fn new(asset_server: Res<AssetServer>) -> Self {
        Self {
            bgm_volume: 0.3,
            jump: asset_server.load("audio/jump.wav"),
            death: asset_server.load("audio/death.wav"),
            explosion: asset_server.load("audio/explosion.wav"),
            bgm: asset_server.load("audio/bgm.wav"),
            collect: asset_server.load("audio/collect.wav"),
            fireball: asset_server.load("audio/fireball.wav"),
            air: asset_server.load("audio/air.wav"),
            steam: asset_server.load("audio/steam.wav"),
            pew: asset_server.load("audio/pew.wav"),
            hurt: asset_server.load("audio/hurt.wav"),
            ping: asset_server.load("audio/ping.wav"),
            blip1: asset_server.load("audio/blip1.wav"),
            blip2: asset_server.load("audio/blip2.wav"),
        }
    }
}

pub fn init_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioManager::new(asset_server));
}
