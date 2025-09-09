use bevy::state::state::States;

pub mod menu;

#[derive(States, Default, Clone, Debug, Hash, Eq, PartialEq)]
pub enum MenuState{
    #[default]
    Disabled,
    Main,
    Quit
}