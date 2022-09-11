pub mod ability_menu;
pub mod load_game;
pub mod load_menu;
pub mod main_menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    LoadMenu,
    MainMenu,
    LoadGame,
    InGame,
    AbilityMenu,
}
