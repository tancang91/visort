use bevy::{prelude::*, sprite::Anchor};
use bevy::time::FixedTimestep;
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
const NUMBER_BARS: u8 = 10;

const TIMESTEP_1_PER_SECOND: f64 = 15.0 / 60.0;

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
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(sorting_system)
        )
        .run();
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();

    let mut bar_collection = BarCollection {
        bars: Vec::with_capacity(NUMBER_BARS as usize),
    };

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
        let entity = commands
            .spawn_bundle(BarBunndle::new(
                x,
                y_base - (delta as f32 * (BAR_HEIGH + BAR_PADDING)),
                w.clone(),
                BAR_HEIGH,
            ))
            .id();

        bar_collection.bars.push(entity);
    }

    commands.insert_resource(bar_collection);
}

fn sorting_system(mut bar_collection: ResMut<BarCollection>
    , mut query: Query<(&Bar, &mut Sprite, &mut Transform)>
    , windows: Res<Windows>)
{
    let window = windows.get_primary().unwrap();
    let end = (NUMBER_BARS - 1) as usize;

    let bar_start = bar_collection.bars[0];
    let bar_end = bar_collection.bars[end];

    bar_collection.bars.swap(0, end);

    if let Ok((_, mut sprite, mut transform)) = query.get_mut(bar_start) {
        transform.translation.y = rank_to_y(end as u32, window.height());
        sprite.color = Color::BLUE;
    }

    if let Ok((_, mut sprite, mut transform)) = query.get_mut(bar_end) {
        transform.translation.y = rank_to_y(0, window.height());
        sprite.color = Color::DARK_GREEN;
    }
}

struct BarCollection {
    bars: Vec<Entity>,
}

#[derive(Component)]
struct Bar {
    length: f32,
}

#[derive(Bundle)]
struct BarBunndle {
    bar: Bar,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl BarBunndle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            bar: Bar {length: width},
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
        }
    }
}

fn rank_to_y(rank: u32, y_base: f32) -> f32 {
    let y_base: f32 = y_base / 2.0 - WINDOW_PADDING;
    y_base - (rank as f32 * (BAR_HEIGH + BAR_PADDING))
}

