pub mod ability_menu;
pub mod game;
pub mod load_game;
pub mod load_menu;
pub mod main_menu;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum State {
    #[default]
    LoadMenu,
    MainMenu,
    LoadGame,
    InGame,
    AbilityMenu,
}
