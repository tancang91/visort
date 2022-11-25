mod utils;

use bevy::time::FixedTimestep;
use bevy::{prelude::*, sprite::Anchor};
use bevy_egui::{egui, egui::TextureFilter, EguiContext, EguiPlugin, EguiSettings};

use rand::seq::SliceRandom;

use visort_core::{BubbleSorter, InsertionSorter, Sorter};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Window setting
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_PADDING: f32 = 5.0;

// Bar setting
const BAR_HEIGH: f32 = 4.0;
const BAR_BASE_WIDTH: f32 = 20.0;
const BAR_COLOR: Color = Color::RED;
const BAR_PADDING: f32 = 2.0;
const NUMBER_BARS: u8 = 50;

const TIMESTEP_1_PER_SECOND: f64 = 1.0 / 120.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "I am a window!".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(render_system)
                .with_system(sorting_system),
        )
        .add_system(ui_system)
        .run();
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();

    let mut bar_collection = BarCollection {
        bars: Vec::with_capacity(NUMBER_BARS as usize),
        snapshot: None,
        index: -1,
        algorithm: SortAlgorithm::BubbleSort,
    };

    // Rectangle
    let x: f32 = -1.0 * (window.width() / 2.0) + WINDOW_PADDING;
    let y_base: f32 = window.height() / 2.0 - WINDOW_PADDING;

    let mut range: Vec<f32> = (0..NUMBER_BARS)
        .into_iter()
        .map(|w| BAR_BASE_WIDTH * (2.0 + 0.2 * w as f32))
        .collect();

    range.shuffle(&mut rng);

    for (delta, w) in range.iter().enumerate() {
        let entity = commands
            .spawn_bundle(BarBunndle::new(
                x,
                rank_to_y(delta as u32, y_base),
                w.clone(),
                BAR_HEIGH,
            ))
            .id();

        bar_collection.bars.push(entity);
    }
    bar_collection.index = 0;

    commands.insert_resource(bar_collection);
}

fn is_sorted<I>(data: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord,
{
    let mut it = data.into_iter();
    match it.next() {
        None => true,
        Some(first) => it
            .scan(first, |state, next| {
                let cmp = *state <= next;
                *state = next;
                Some(cmp)
            })
            .all(|b| b),
    }
}

fn sorting_system(mut bar_collection: ResMut<BarCollection>, bars: Query<&Bar>) {
    let ranges: Vec<_> = bar_collection
        .bars
        .iter()
        .map(|entity| bars.get(entity.clone()).unwrap().length as i32)
        .collect();

    match bar_collection.snapshot {
        None => bar_collection.snapshot = Some(InsertionSorter.sort(&ranges)),
        _ => {}
    }
}

fn render_system(
    mut bar_collection: ResMut<BarCollection>,
    mut query: Query<(&Bar, &mut Sprite, &mut Transform)>,
    mut egui_ctx: ResMut<EguiContext>,
    windows: Res<Windows>,
) {
    let height = windows.get_primary().unwrap().height();

    match bar_collection.snapshot {
        Some(ref s) => {
            let index = bar_collection.index as usize;
            if index < s.len() {
                let snapshot = s.get(index).unwrap();

                for (rank, &index) in snapshot.iter().enumerate() {
                    let bar = bar_collection.bars[index as usize];
                    if let Ok((_, mut sprite, mut transform)) = query.get_mut(bar) {
                        transform.translation.y = rank_to_y(rank as u32, height);
                        sprite.color = Color::BLUE;
                    }
                }
                bar_collection.index += 1;
            }
        }
        _ => {}
    };
}

fn ui_system(mut bar_collection: ResMut<BarCollection>, mut egui_ctx: ResMut<EguiContext>) {
    egui::SidePanel::right("config_panel")
        //.default_width(100.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.allocate_space(egui::Vec2::new(100.0, 20.0));
            ui.vertical(|ui| {
                ui.label("Select your algorithm");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", bar_collection.algorithm))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut bar_collection.algorithm,
                            SortAlgorithm::BubbleSort,
                            "BubbleSort",
                        );
                        ui.selectable_value(
                            &mut bar_collection.algorithm,
                            SortAlgorithm::InsertionSort,
                            "InsertionSort",
                        );
                    });
            });
            ui.allocate_space(egui::Vec2::new(100.0, 2.0));
            if ui.button("Run").clicked() {}
            ui.end_row();
        });
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum SortAlgorithm {
    InsertionSort,
    BubbleSort,
}

#[derive(Resource)]
struct BarCollection {
    bars: Vec<Entity>,
    snapshot: Option<Vec<Vec<u32>>>,
    index: i32,
    algorithm: SortAlgorithm,
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
            bar: Bar { length: width },
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
