#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)] // Bevy can have complex queries, so we shush clippy
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy

pub(crate) mod prelude;
use crate::prelude::*;

#[cfg(feature = "debug")]
mod debug;
#[cfg(feature = "debug")]
use debug::*;

fn main() {
    let mut app = App::new();

    // Default Plugins
    default_plugins(&mut app).insert_resource(ClearColor(Color::BLACK));

    // anything we don't need in release versions
    #[cfg(feature = "debug")]
    app.add_plugin(DebugPlugin);

    // game related
    app.add_plugin(GamePlugin {
        state_running: GameState::InGame,
        state_main_menu: GameState::Ui(MainMenu),
        state_construct: GameState::Construct(MapGen),
    });

    app.run();
}

fn default_plugins(app: &mut App) -> &mut App {
    let defaults = DefaultPlugins
        .set(WindowPlugin {
            window: WindowDescriptor {
                title: APP_NAME.to_string(),
                width: (GRID_WIDTH * MIN_SPRITE_WIDTH) as f32 * INITIAL_WINDOW_SCALE,
                height: (GRID_HEIGHT * MIN_SPRITE_HEIGHT) as f32 * INITIAL_WINDOW_SCALE,
                resize_constraints: WindowResizeConstraints {
                    min_width: (GRID_WIDTH * MIN_SPRITE_WIDTH) as f32,
                    min_height: (GRID_HEIGHT * MIN_SPRITE_HEIGHT) as f32,
                    ..Default::default()
                },
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            },
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .build()
        .disable::<bevy::log::LogPlugin>();

    app.add_plugins(defaults);

    #[cfg(not(feature = "debug"))]
    app.add_plugin(bevy::log::LogPlugin { level: bevy::log::Level::WARN, ..Default::default() });

    app
}
