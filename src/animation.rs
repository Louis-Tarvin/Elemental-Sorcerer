use bevy::{
    prelude::{Component, Query, Res},
    sprite::TextureAtlasSprite,
    time::{Time, Timer},
};

#[derive(Component)]
pub struct Animated {
    timer: Timer,
    pub start: usize,
    pub end: usize,
    pub play_once: bool,
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
