use bevy::{core_pipeline::core_2d::Camera2d, ecs::{ query::With, system::{Res, ResMut, Single}}, input::{keyboard::KeyCode, ButtonInput}, log::error, render::camera::Projection, ui::UiScale, window::{MonitorSelection, Window, WindowMode}};

pub fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    mut query_camera: Single<&mut Projection, With<Camera2d>>, 
    mut uiscale: ResMut<UiScale>
) {
    if keys.just_pressed(KeyCode::F11) {
        match query_camera.as_mut() {
            Projection::Orthographic(ortho) => {
                match window.mode {
                    bevy::window::WindowMode::Windowed => {

                        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
                        ortho.scale = ortho.scale / 2.;
                        uiscale.0 = uiscale.0 * 2.;

                    }
                    bevy::window::WindowMode::BorderlessFullscreen(_) => {

                        window.mode = WindowMode::Windowed;
                        ortho.scale = ortho.scale * 2.;
                        uiscale.0 = uiscale.0 / 2.;
                    },
                    _ => {
                        error!("Window is in invalid mode")
                    }
                };
                
            }
            _ => {
                error!("Scrolling Error: Projection is not Orthograpic as should be by Default");
            }
        }
    }
}