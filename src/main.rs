use bevy::{app::{App, PluginGroup}, log::LogPlugin, render::texture::ImagePlugin, state::{app::AppExtStates, state::States}, window::{Window, WindowPlugin, WindowResolution}, DefaultPlugins};
use crate::{game::game::GamePlugin, menu::menu::MenuPlugin};

mod game;
mod menu;
mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(
                WindowPlugin{
                    primary_window: 
                        Some(Window{
                            title: "Sandrissi".into(),
                            resolution: WindowResolution::new(960., 540. ),
                            ..Default::default()
                        }), 
                        ..Default::default()
                }
            )
            .set( ImagePlugin::default_nearest() )
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                level: bevy::log::Level::DEBUG,
                custom_layer: |_| None,
            }),
        MenuPlugin,
        GamePlugin,
    ))

    
    .insert_state(AppState::MainMenu);

    app.run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    MainMenu,
    InGame,
}