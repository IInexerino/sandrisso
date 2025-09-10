use bevy::{app::{FixedUpdate, Plugin, Startup, Update}, core_pipeline::core_2d::Camera2d, ecs::{entity::Entity, query::With, schedule::{IntoScheduleConfigs, SystemSet}, system::{Commands, Res, ResMut, Single}}, input::{keyboard::KeyCode, ButtonInput}, log::info, render::camera::{OrthographicProjection, Projection}, state::{condition::in_state, state::{NextState, OnEnter, OnExit}}, ui::UiScale};
use crate::{game::sandworld::{ image_setup::empty_grid_image_setup, main_interaction::main_interaction_loop, draw_image::draw_image, user_element_interraction::{user_adds_element, user_selects_element, UserSelectedElements}, ElemKind, GridParams}, utils::helper_utils::toggle_resolution, AppState};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {

        app
        .insert_resource(UserSelectedElements::single(ElemKind::Sand))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, toggle_resolution)


        .add_systems(OnEnter(AppState::InGame),
            empty_grid_image_setup
        )
            

        .configure_sets(FixedUpdate, 
            (
                ElementSystem::MainInteractionLoop,
                ElementSystem::DrawOnImage,
                ElementSystem::UserElementGeneration,
            )
                .run_if(in_state(AppState::InGame))
                .chain(),
        )
        .add_systems(FixedUpdate, 
            (
                main_interaction_loop.in_set(ElementSystem::MainInteractionLoop),
                draw_image.in_set(ElementSystem::DrawOnImage),
                (
                    user_selects_element, 
                    user_adds_element
                )
                    .in_set(ElementSystem::UserElementGeneration),
                back_to_main_menu.run_if(in_state(AppState::InGame))
            )
        )

        
        .add_systems(OnExit(AppState::InGame),
            despawn_grid
        );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum ElementSystem {
    UserElementGeneration,
    MainInteractionLoop,
    DrawOnImage
}

fn spawn_camera(
    mut commands: Commands,
    mut uiscale: ResMut<UiScale>
) {
    info!("Running the spawn_camera system");
    commands.spawn((
        Camera2d,
        Projection::Orthographic(
            OrthographicProjection {
                scale: 2.0,
                ..OrthographicProjection::default_2d()
            }
        )
    ));
    uiscale.0 = uiscale.0 / 2.;
}

fn back_to_main_menu(keys: Res<ButtonInput<KeyCode>>, mut app_s: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_s.set(AppState::MainMenu)
    }
}

fn despawn_grid( mut commands: Commands, grid_e: Single<Entity, With<GridParams>> ) {
    commands.entity(grid_e.into_inner()).despawn();
}