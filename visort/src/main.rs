use bevy::{prelude::*, sprite::Anchor};
use rand::seq::SliceRandom;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Default width
const BAR_HEIGH: f32 = 10.0;

// Window setting
const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_PADDING: f32 = 5.0;

// Bar setting
const BAR_COLOR: Color = Color::RED;
const BAR_PADDING: f32 = 4.0;
const NUMBER_BARS: u8 = 20;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();

    // Rectangle
    let x: f32 = -1.0 * (window.width() / 2.0) + WINDOW_PADDING;
    let y_base: f32 = window.height() / 2.0 - WINDOW_PADDING;
    let width_base: f32 = 20.0;

    let mut range: Vec<f32> = (0..NUMBER_BARS)
        .into_iter()
        .map(|w| width_base * (1.0 + 0.2 * w as f32))
        .collect();

    range.shuffle(&mut rng);

    for (delta, w) in range.iter().enumerate() {
        commands.spawn_bundle(BarBunndle::new(
            x,
            y_base - (delta as f32 * (BAR_HEIGH + BAR_PADDING)),
            w.clone(),
            BAR_HEIGH,
        ));
    }
}

#[derive(Component)]
struct Length(f32);

#[derive(Bundle)]
struct BarBunndle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    length: Length,
}

impl BarBunndle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                sprite: Sprite {
                    color: BAR_COLOR,
                    custom_size: Some(Vec2::new(width, height)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            length: Length(width),
        }
    }
}
