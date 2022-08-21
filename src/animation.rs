use bevy::{
    prelude::{Changed, Component, Query, Res},
    sprite::TextureAtlasSprite,
    time::{Time, Timer},
};

pub enum AnimaionState {
    Idle,
    Walking,
    JumpUp,
    JumpDown,
}

#[derive(Component)]
pub struct Animated {
    pub state: AnimaionState,
    timer: Timer,
    start: usize,
    end: usize,
    play_once: bool,
}
impl Animated {
    pub fn new(seconds_per_frame: f32, play_once: bool) -> Self {
        Self {
            state: AnimaionState::Idle,
            timer: Timer::from_seconds(seconds_per_frame, true),
            start: 20,
            end: 24,
            play_once,
        }
    }
}

pub fn system(time: Res<Time>, mut query: Query<(&mut TextureAtlasSprite, &mut Animated)>) {
    let delta = time.delta();
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(delta);
        if animation.timer.finished() {
            if sprite.index < animation.start {
                sprite.index = animation.start;
            }
            sprite.index = ((sprite.index - animation.start + 1)
                % (animation.end - animation.start))
                + animation.start;
            if animation.play_once && sprite.index + 1 == animation.end {
                animation.start = sprite.index;
            }
        }
    }
}

pub fn state_update_system(mut query: Query<&mut Animated, Changed<Animated>>) {
    for mut animation in query.iter_mut() {
        match animation.state {
            AnimaionState::Idle => {
                animation.start = 40;
                animation.end = 44;
            }
            AnimaionState::Walking => {
                animation.start = 8;
                animation.end = 14;
            }
            AnimaionState::JumpUp => {
                animation.start = 56;
                animation.end = 59;
            }
            AnimaionState::JumpDown => {
                animation.start = 48;
                animation.end = 51;
            }
        }
    }
}
