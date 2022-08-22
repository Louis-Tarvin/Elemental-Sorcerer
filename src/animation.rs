use bevy::{
    prelude::{Changed, Component, Query, Res},
    sprite::TextureAtlasSprite,
    time::{Time, Timer},
};

#[derive(Component, Eq, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    JumpUp,
    JumpDown,
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
}

#[derive(Component)]
pub struct Animated {
    timer: Timer,
    start: usize,
    end: usize,
    play_once: bool,
}
impl Animated {
    pub fn new(seconds_per_frame: f32, start: usize, end: usize, play_once: bool) -> Self {
        Self {
            timer: Timer::from_seconds(seconds_per_frame, true),
            start,
            end,
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

pub fn state_update_system(
    mut query: Query<(&mut Animated, &AnimationState), Changed<AnimationState>>,
) {
    for (mut animation, state) in query.iter_mut() {
        match state {
            AnimationState::Idle => {
                animation.start = 40;
                animation.end = 44;
            }
            AnimationState::Walking => {
                animation.start = 8;
                animation.end = 14;
            }
            AnimationState::JumpUp => {
                animation.start = 56;
                animation.end = 59;
            }
            AnimationState::JumpDown => {
                animation.start = 48;
                animation.end = 51;
            }
        }
    }
}
