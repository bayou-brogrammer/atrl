use crate::prelude::*;
pub type CurrentGameState = CurrentState<GameState>;
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AssetLoadState {
    #[default]
    Load,
    LoadFailure,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ConstructState {
    #[default]
    Construct,
    Setup,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UiState {
    #[default]
    MainMenu,
}

#[derive(Default, Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum GameState {
    #[default]
    Initializing,
    AssetLoad(AssetLoadState),
    Ui(UiState),
    Construct(ConstructState),
    InGame,
    Dead,
    Quit,
}

/// Flow:
/// 1. Initialize
/// |-> 2. SplashSetup
/// |-> 3. AssetLoadState
///      |-> AssetLoadFailure
///          |-> Quit
/// |
/// |-> 4. ConstructState
/// |-> 5. UiState(MaiMenu)
/// |-> 6. InGame
///      |-> MainMenu
///      |-> Quit
///          |-> None :)
impl StateNext for GameState {
    fn next(&self) -> Option<Self> {
        match self {
            Self::Initializing => Some(Self::AssetLoad(AssetLoadState::Load)),
            Self::InGame => Some(Self::Ui(UiState::MainMenu)),
            Self::Quit => None,

            // Assets
            Self::AssetLoad(asset_state) => match asset_state {
                AssetLoadState::Load => Some(Self::Ui(UiState::MainMenu)),
                AssetLoadState::LoadFailure => Some(Self::Quit),
            },

            // UI
            Self::Ui(ui_state) => match ui_state {
                UiState::MainMenu => Some(Self::Construct(ConstructState::Construct)),
            },

            // Construct
            Self::Construct(construct_state) => match construct_state {
                ConstructState::Construct => Some(Self::Construct(ConstructState::Setup)),
                ConstructState::Setup => Some(Self::InGame),
            },

            Self::Dead => Some(Self::Ui(UiState::MainMenu)),
        }
    }
}
