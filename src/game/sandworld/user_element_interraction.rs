use bevy::{ecs::{query::With, resource::Resource, system::{Local, Res, ResMut, Single}}, input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput}, math::Vec2, render::camera::Camera, transform::components::GlobalTransform, window::{PrimaryWindow, Window}};
use crate::game::sandworld::{Elem, ElemKind, ElemPos, GridCells, GridParams, GRID_SIZE};

#[derive(Resource)]
pub struct UserSelectedElements{
    pub kind: ElemKind,
    pub radius: u32
}
impl UserSelectedElements{ 
    pub fn single(kind: ElemKind) -> Self { 
        UserSelectedElements { kind , radius: 1 }
    }
}

pub fn user_selects_element(
    keys: Res<ButtonInput<KeyCode>>,
    mut element_selection: ResMut<UserSelectedElements>,
) {
    let toggled_elem_kind = if keys.just_pressed(KeyCode::KeyM) {
        match element_selection.kind {
            ElemKind::Empty => Some(ElemKind::Sand),
            ElemKind::Sand => Some(ElemKind::Stone),
            ElemKind::Stone => Some(ElemKind::Empty),
        }
    } else { None };

    let toggled_radius = if keys.just_pressed(KeyCode::KeyN) {
        match element_selection.radius {
            _ => Some(1),
        }
    } else { None };

    if let Some(kind) = toggled_elem_kind {
        element_selection.kind = kind
    }
    if let Some(radius) = toggled_radius {
        element_selection.radius = radius
    }
}

pub struct PrevMousePos(pub Option<ElemPos>);
impl Default for PrevMousePos {
    fn default() -> Self { PrevMousePos(None) }
}

/// If mouse button is pressed - calculates the coordinates of the cell over which the cursor is hovering
/// 
/// Inserts the [`UserGeneratedElements`] resource with the selected square and the current [`UserSelectedElement`]
pub fn user_adds_element(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    grid_q: Single<(&GlobalTransform, &GridParams, &mut GridCells)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    selected_elems: Res<UserSelectedElements>,
    mut previous_mouse_pos: Local<PrevMousePos>,
) {
    if mouse_buttons.pressed(MouseButton::Left) 
    || mouse_buttons.just_pressed(MouseButton::Left) {

        if let Some(world_pos) = cursor_to_world(window, camera) {
            let (g_transform, grid_params, mut grid_cells) = grid_q.into_inner();
            if let Some(current_pos) = world_to_grid(world_pos, g_transform, grid_params.scale) {

                let all_click_squares = if let Some(previous_m_pos) = previous_mouse_pos.0 {
                    bresenham_line(
                        previous_m_pos.x as i32,
                        previous_m_pos.y as i32, 
                        current_pos.x as i32, 
                        current_pos.y as i32, 
                    )
                } else { vec![current_pos] };

                for sq_pos in all_click_squares {

                    if grid_cells.get_elem_at(sq_pos).unwrap().kind == ElemKind::Empty 
                    || selected_elems.kind == ElemKind::Empty {
                        grid_cells.set_elem_at(sq_pos, Elem::new(selected_elems.kind, false,)).unwrap();
                    }
                }

                previous_mouse_pos.0 = Some(current_pos);
                return 
            }
        } 
    } 
    previous_mouse_pos.0 = None
}

fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<ElemPos> {
    if x0 == x1 && y0 == y1 {
        return vec![ElemPos::new(x0 as u32, y1 as u32)]
    }

    let mut points = Vec::new();

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        if !(x == x0 && y == y0) { 
            points.push(ElemPos::new(x as u32, y as u32));
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    
    points
}

/// window cursor position to world cursor position
fn cursor_to_world(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>
) -> Option<Vec2> {
    let (camera, cam_transform) = camera.into_inner();

    if let Some(screen_pos) = window.cursor_position() {
        camera.viewport_to_world_2d(cam_transform, screen_pos)
            .ok()
    } else {
        None
    }
}

/// world cursor coordinates to grid coordinates
fn world_to_grid(
    world_pos: Vec2,
    sprite_transform: &GlobalTransform,
    scale: f32,
) -> Option<ElemPos>{
    let sprite_center = sprite_transform.translation().truncate();

    let size = Vec2::new(GRID_SIZE.width as f32 * scale , GRID_SIZE.height as f32 * scale );

    let min = sprite_center - size / 2.0;

    let local_pos = world_pos - min;

    let gx = (local_pos.x / scale).floor() as isize;
    let gy = (local_pos.y / scale).floor() as isize;

    if gx >= 0 
    && gy >= 0 
    && gx < GRID_SIZE.width as isize 
    && gy < GRID_SIZE.height as isize {
        Some(ElemPos::new(gx as u32, GRID_SIZE.height - 1 - gy as u32 ))
    } else {
        None
    }
}
