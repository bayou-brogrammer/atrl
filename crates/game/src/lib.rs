#![allow(clippy::module_inception)]

mod player {
    mod player_plugin;
    pub use player_plugin::*;
}

mod systems {
    mod apply_damage;
    pub use apply_damage::*;
    mod input;
    pub use input::*;
    mod move_actors;
    pub use move_actors::*;
    mod player_input;
    pub use player_input::*;
    mod perform_healing;
    pub use perform_healing::*;
    mod spawn_player;
    pub use spawn_player::*;
}

mod spawner {
    mod spawner_plugin;
    pub use spawner_plugin::*;
}

mod game_plugin;

pub mod prelude {
    mod import {
        pub use bevy::{
            prelude::*,
            utils::{HashMap, HashSet},
        };

        pub use banana_bevy_utils::prelude::*;

        pub use bevy_ecs_tilemap::prelude::*;
        pub use bevy_tileset::prelude::*;

        pub use leafwing_input_manager::{action_state::ActionState, prelude::*};

        pub use iyes_loopless::prelude::*;

        pub use atrl_camera::prelude::*;
        pub use atrl_common::prelude::AssetLoadState::*;
        pub use atrl_common::prelude::ConstructState::*;
        pub use atrl_common::prelude::TurnState::*;
        pub use atrl_common::prelude::UiState::*;
        pub use atrl_common::prelude::*;
        pub use atrl_map::prelude::*;
        pub use atrl_raws::prelude::*;
        pub use atrl_ui::prelude::*;
    }
    pub(crate) use import::*;

    pub use crate::player::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    pub use crate::game_plugin::*;
}