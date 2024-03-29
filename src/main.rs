#![warn(clippy::nursery, clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)] // Bevy has a lot of arguments, so we shush clippy
#![allow(unused_imports)] // TODO: REMOVE ME

pub(crate) mod prelude;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::prelude::*;

#[cfg(feature = "debug")]
pub(crate) mod debug {
    mod systems {
        mod show_ui;
        mod window_title;
        pub use show_ui::*;
        pub use window_title::*;
    }
    pub use systems::*;

    mod debug_plugin;
    pub use debug_plugin::*;
}

mod log;

fn main() {
    let mut app = App::new();

    // Default Plugins
    default_plugins(&mut app).insert_resource(ClearColor(Color::BLACK));

    // anything we don't need in release versions
    #[cfg(feature = "debug")]
    app.add_plugin(debug::DebugPlugin);

    // game related
    app.add_plugin(GamePlugin {
        state_running: GameState::InGame,
        state_main_menu: GameState::Ui(MainMenu),
        state_asset_load: GameState::AssetLoad(Load),
        state_construct: GameState::Construct(Construct),
        state_construct_setup: GameState::Construct(Setup),
        state_asset_load_failure: GameState::AssetLoad(LoadFailure),
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
                // present_mode: PresentMode::AutoVsync,
                ..Default::default()
            },
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(atlr_log_plugin())
        .build();

    app.add_plugins(defaults);

    #[cfg(feature = "release")]
    {
        defaults.add_before::<bevy::asset::AssetPlugin, _>(bevy_embedded_assets::EmbeddedAssetPlugin);
    }

    app
}
