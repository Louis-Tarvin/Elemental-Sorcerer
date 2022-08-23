pub mod ability_menu;
pub mod main_menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    MainMenu,
    Loading,
    InGame,
    AbilityMenu,
}
