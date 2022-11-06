use bevy::{prelude::*, sprite::Anchor};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Rectangle
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_translation(
                       Vec3::new(-200.0, 200.0, 0.0)
        ),
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(50.0, 20.0)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        ..default()
    });
}
