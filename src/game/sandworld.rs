use std::fmt::Display;
use bevy::{asset::{Assets, Handle, RenderAssetUsages}, color::{ Color, ColorToPacked}, ecs::{component::Component, query::With, resource::Resource, system::{Commands, Local, Res, ResMut, Single}}, image::Image, input::{mouse::MouseButton, ButtonInput}, log::info, math::{Vec2, Vec3}, render::{camera::Camera, render_resource::{Extent3d, TextureDimension, TextureFormat}}, sprite::Sprite, transform::components::{GlobalTransform, Transform}, window::{PrimaryWindow, Window}};

const GRID_SCALE: f32 = 5.;
const GRID_SIZE: GridSize = GridSize::new(256, 192);
const EMPTY_COLOR: Color = Color::srgba(0., 0., 0., 0.);


#[derive(Resource)]
pub struct GridImage(pub Handle<Image>);

#[derive(Resource)]
pub struct UserSelectedElements{
    pub kind: ElementKind,
    pub radius: u32
}
impl UserSelectedElements{ 
    pub fn single(kind: ElementKind) -> Self { 
        UserSelectedElements { kind , radius: 1 }
    }
}

#[derive(Component)]
pub struct GridParams {
    pub scale: f32,
}
#[derive(Component)]
pub struct GridCells {
    pub cells: [ElementKind; GRID_SIZE.count()]
}
impl GridCells {
    pub fn new_empty() -> Self {
        GridCells { cells: [ ElementKind::Empty ; GRID_SIZE.count() ] }
    }
    pub fn get_elem_at(&self, pos: ElemPos) -> Option<ElementKind> {
        if pos.in_bounds() {
            Some( self.cells[(pos.y * GRID_SIZE.width + pos.x) as usize] )
        } else { None }
    }
    pub fn set_elem_at(&mut self, pos: ElemPos, kind: ElementKind) -> Option<()> {
        if pos.in_bounds() {
            self.cells[(pos.y * GRID_SIZE.width + pos.x) as usize] = kind; 

            Some(())
        } else { None }
    }
}

pub struct GridSize{
    width: u32,
    height: u32,
}
impl GridSize {
    const fn new(width: u32, height: u32) -> Self {
        GridSize { width, height }
    }
    const fn count(&self) -> usize {
        (self.width * self.height) as usize
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct ElemPos{
    pub x: u32,
    pub y: u32
}
impl ElemPos {
    pub fn new(x: u32, y: u32) -> Self {
        ElemPos{ x, y }
    }
    pub fn in_bounds(&self) -> bool {
        if self.y < GRID_SIZE.height 
        && self.x < GRID_SIZE.width { true } else { false }
    }
    pub fn in_border_bottom(&self) -> bool {
        if self.y < GRID_SIZE.height - 1 { true }
        else { false }
    }
    pub fn in_border_left(&self) -> bool {
        if self.x > 0 { true }
        else { false }
    }
    pub fn in_border_right(&self) -> bool {
        if self.x < GRID_SIZE.width - 1 { true }
        else { false }
    }
    /*
    pub fn get_inbound_coords_within_sq_radius(&self, radius: u32) -> Vec<ElemPos> {
        let adjustment = radius as i32 - 1;

        let mut neighbors = Vec::new();

        for x in self.x as i32 -adjustment..=self.x as i32+adjustment {
            for y in self.y as i32 -adjustment..=self.y as i32+adjustment{
                if x < 0 
                || y < 0
                || x >= GRID_SIZE.width as i32
                || y >= GRID_SIZE.height as i32
                || (x == self.x as i32 && y == self.y as i32) {
                    continue
                } else {
                    neighbors.push(ElemPos::new(x as u32, y as u32))
                }
            }
        }

        neighbors
    }
     */
}

#[derive(Clone, Copy, PartialEq)]
pub enum ElementKind {
    Empty,
    Stone,
    Sand,
}
impl ElementKind {
    fn to_color(&self) -> Color {
        match self {
            ElementKind::Empty => EMPTY_COLOR,
            ElementKind::Sand => Color::srgba(0.86, 0.71, 0.46, 1.0),
            ElementKind::Stone => Color::srgba(0.52,0.52,0.52, 1.),
        }
    }
}
impl Display for ElementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementKind::Empty => write!(f, "[Empty]"),
            ElementKind::Stone => write!(f, "[Stone]"),
            ElementKind::Sand => write!(f, "[Sand]"),
        }

        
    }
}

/// Creates an black image of a certain size at the center of the world, upscaled by the scaling factor 
pub fn empty_grid_image_setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>
) {
    let grid = GridParams { scale: GRID_SCALE };

    // Create an image that we are going to draw into
    let image = Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width: GRID_SIZE.width,
            height: GRID_SIZE.height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with a transparent black color
        &(EMPTY_COLOR.to_srgba().to_u8_array()),
        // Use the same encoding as the color we set
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let handle = images.add(image);
    let transform = Transform::from_xyz(0., 0., 0.)
            .with_scale(Vec3::splat(grid.scale));

    commands.spawn((
        Sprite::from_image(handle.clone()),
        transform,
        grid,
        GridCells::new_empty(),
    ));
    
    commands.insert_resource(GridImage(handle));
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

                    if grid_cells.get_elem_at(sq_pos).unwrap() == ElementKind::Empty 
                    || selected_elems.kind == ElementKind::Empty {
                        grid_cells.set_elem_at(sq_pos, selected_elems.kind);
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

pub fn main_checking_loop(
    mut grid_cells: Single<&mut GridCells>,
    mut dir: Local<bool>
) {

    let grid_cells = grid_cells.as_mut();

    for x in 0..GRID_SIZE.width {
        for y in (0..GRID_SIZE.height).rev() {
            let pos = ElemPos::new(x, y);
            let kind = grid_cells.get_elem_at(pos).unwrap();

            match kind {
                ElementKind::Empty | ElementKind::Stone => continue,
                ElementKind::Sand => {
                    sand_algorithm(grid_cells, pos, *dir);
                },
            }
        }
    }
    if *dir { *dir = false } else { *dir = true }
}

fn sand_algorithm(
    grid_cells: &mut GridCells,
    pos: ElemPos,
    dir: bool
) {
    if pos.in_border_bottom() {
        let sand = ElementKind::Sand;
        let permb_elems = vec![ElementKind::Empty];

        if unchecked_set_color_down(grid_cells, pos, sand, &permb_elems) {}
        else if dir {
            if set_color_leftdown(grid_cells, pos, sand, &permb_elems) {}
            else if set_color_rightdown(grid_cells, pos, sand, &permb_elems) {}
        } else {
            if set_color_rightdown(grid_cells, pos, sand, &permb_elems) {}
            else if set_color_leftdown(grid_cells, pos, sand, &permb_elems) {}
        }
    }
}


fn unchecked_set_color_down(grid_cells: &mut GridCells, pos: ElemPos, kind: ElementKind, permb_elems: &Vec<ElementKind>) -> bool {
    let down_pos = ElemPos::new(pos.x, pos.y + 1);
    let check_kind = grid_cells.get_elem_at(down_pos).unwrap();
    if permb_elems.contains(&check_kind) {
        grid_cells.set_elem_at(pos, check_kind).unwrap();
        grid_cells.set_elem_at(down_pos, kind).unwrap();
        return true
    }
    return false
}

fn set_color_leftdown(grid_cells: &mut GridCells, pos: ElemPos, kind: ElementKind, permb_elems: &Vec<ElementKind>) -> bool {
    if pos.in_border_left() {
        let leftdown_pos = ElemPos::new(pos.x - 1, pos.y + 1);
        let check_kind = grid_cells.get_elem_at(leftdown_pos).unwrap();
        if permb_elems.contains(&check_kind) {
            grid_cells.set_elem_at(pos, check_kind).unwrap();
            grid_cells.set_elem_at(leftdown_pos, kind).unwrap();
            return true
        }
    }
    return false
}

fn set_color_rightdown(grid_cells: &mut GridCells, pos: ElemPos, kind: ElementKind, permb_elems: &Vec<ElementKind>) -> bool {
    if pos.in_border_right() {
        let rightdown_pos = ElemPos::new(pos.x + 1, pos.y + 1);
        let check_kind = grid_cells.get_elem_at(rightdown_pos).unwrap();
        if permb_elems.contains(&check_kind) {
            grid_cells.set_elem_at(pos, check_kind).unwrap();
            grid_cells.set_elem_at(rightdown_pos, kind).unwrap();
            return true
        }
    }
    return false
}

pub fn draw_image(
    grid_cells: Single<&GridCells>,
    handle: Res<GridImage>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = images.get_mut(&handle.0).expect("Image not found");

    for x in 0..GRID_SIZE.width {
        for y in 0..GRID_SIZE.height {
            let elem_color = grid_cells
                .get_elem_at(ElemPos::new(x, y))
                .unwrap()
                .to_color();
            image.set_color_at(x, y, elem_color).unwrap();
        }
    }
}