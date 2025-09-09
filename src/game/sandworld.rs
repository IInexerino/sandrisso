use std::fmt::Display;
use bevy::{asset::{Assets, Handle, RenderAssetUsages}, color::{ palettes::css, Color, ColorToPacked}, ecs::{component::Component, query::With, resource::Resource, system::{Commands, Local, Res, ResMut, Single}}, image::{Image, TextureAccessError}, input::{mouse::MouseButton, ButtonInput}, log::info, math::{Vec2, Vec3}, render::{camera::Camera, render_resource::{Extent3d, TextureDimension, TextureFormat}}, sprite::Sprite, transform::components::{GlobalTransform, Transform}, window::{PrimaryWindow, Window}};

const GRID_SCALE: f32 = 12.;
const GRID_WIDTH: u32 = 64;
const GRID_HEIGHT: u32 = 64;


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
pub struct Grid {
    pub size: GridSize,
    pub scale: f32,
}

pub struct GridSize{
    width: u32,
    height: u32,
}
impl GridSize {
    fn new(width: u32, height: u32) -> Self {
        GridSize { width, height }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct ElementPos{
    pub x: u32,
    pub y: u32
}
impl ElementPos {
    pub fn new(x: u32, y: u32) -> Self {
        ElementPos{ x, y }
    }
    pub fn get_color(&self, image: &mut Image) -> Result<Color, TextureAccessError> {
        image.get_color_at(self.x, self.y)
    }
    pub fn set_color(&self, image: &mut Image, color: Color) -> Result<(), TextureAccessError> {
        image.set_color_at(self.x, self.y, color)
    }
    pub fn in_bounds(&self, grid_size: &GridSize) -> bool {
        if self.x < grid_size.width
        || self.y < grid_size.height {
            false
        } else {
            true
        }
    }
    /*
    pub fn get_inbound_coords_within_sq_radius(&self, grid_size: &GridSize, radius: u32) -> Vec<ElementPos> {
        let adjustment = radius as i32 - 1;

        let mut neighbors = Vec::new();

        for x in self.x as i32 -adjustment..=self.x as i32+adjustment {
            for y in self.y as i32 -adjustment..=self.y as i32+adjustment{
                if x < 0 
                || y < 0
                || x >= grid_size.width as i32
                || y >= grid_size.height as i32
                || (x == self.x as i32 && y == self.y as i32) {
                    continue
                } else {
                    neighbors.push(ElementPos::new(x as u32, y as u32))
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
// 0.85882354, 0.70980394, 0.45882353
// - 0.00117646
// - 0.00019606 
impl ElementKind {
    fn from_color(color: Color) -> Option<Self> {
        if color == Color::srgba(1., 1., 1., 1.) { Some(ElementKind::Empty) }
        else if color == Color::srgba(0.85882354, 0.70980394, 0.45882353, 1.0) { Some(ElementKind::Sand) }
        else if color == Color::srgba(0.52,0.52,0.52, 1.) { Some(ElementKind::Stone) }
        else { None }
        
    }
    fn to_color(&self) -> Color {
        match self {
            ElementKind::Empty => Color::srgba(1., 1., 1., 1.),
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
    
    let grid = Grid {
        size: GridSize::new(GRID_WIDTH, GRID_HEIGHT),
        scale: GRID_SCALE,
    };

    // Create an image that we are going to draw into
    let image = Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width: grid.size.width,
            height: grid.size.height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with a black color
        &(css::WHITE.to_u8_array()),
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
        grid
    ));
    
    commands.insert_resource(GridImage(handle));
}

pub struct PrevMousePos(pub Option<ElementPos>);
impl Default for PrevMousePos {
    fn default() -> Self { PrevMousePos(None) }
}

/// If mouse button is pressed - calculates the coordinates of the cell over which the cursor is hovering
/// 
/// Inserts the [`UserGeneratedElements`] resource with the selected square and the current [`UserSelectedElement`]
pub fn user_adds_element(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    grid_q: Single<(&GlobalTransform, &Grid)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    selected_elems: Res<UserSelectedElements>,
    handle: Res<GridImage>,
    mut images: ResMut<Assets<Image>>,
    mut previous_mouse_pos: Local<PrevMousePos>,
) {
    
    if mouse_buttons.pressed(MouseButton::Left) 
    || mouse_buttons.just_pressed(MouseButton::Left) {

        if let Some(world_pos) = cursor_to_world(window, camera) {
            info!("Cursor in window: (x: {}, y: {})", world_pos.x, world_pos.y);
            let (g_transform, grid) = grid_q.into_inner();
            if let Some(current_pos) = world_to_grid(world_pos, g_transform, &grid.size, grid.scale) {
                info!("Cursor in grid: (x: {}, y: {})", current_pos.x, current_pos.y);

                let all_click_squares = if let Some(previous_m_pos) = previous_mouse_pos.0 {
                    bresenham_line(
                        previous_m_pos.x as i32,
                        previous_m_pos.y as i32, 
                        current_pos.x as i32, 
                        current_pos.y as i32, 
                    )
                } else { vec![current_pos] };

                let image = images.get_mut(&handle.0).expect("Image not found");

                for sq_pos in all_click_squares {
                    let elem_kind = ElementKind::from_color(
                        sq_pos.get_color(image).unwrap()
                    ).unwrap();

                    if elem_kind == ElementKind::Empty {
                        sq_pos.set_color(image, selected_elems.kind.to_color()).unwrap();
                    }
                }

                previous_mouse_pos.0 = Some(current_pos);
                return 
            }
        } 
    } 
    previous_mouse_pos.0 = None
}

fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<ElementPos> {
    if x0 == x1 && y0 == y1 {
        return vec![ElementPos::new(x0 as u32, y1 as u32)]
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
            points.push(ElementPos::new(x as u32, y as u32));
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
    grid_size: &GridSize,
    scale: f32,
) -> Option<ElementPos>{
    let sprite_center = sprite_transform.translation().truncate();

    let size = Vec2::new(grid_size.width as f32 * scale , grid_size.height as f32 * scale );

    let min = sprite_center - size / 2.0;

    let local_pos = world_pos - min;

    let gx = (local_pos.x / scale).floor() as isize;
    let gy = (local_pos.y / scale).floor() as isize;

    if gx >= 0 
    && gy >= 0 
    && gx < grid_size.width as isize 
    && gy < grid_size.height as isize {
        Some(ElementPos::new(gx as u32, grid_size.height - 1 - gy as u32 ))
    } else {
        None
    }
}

pub fn main_checking_loop(
    handle: Res<GridImage>,
    mut images: ResMut<Assets<Image>>,
    grid: Single<&Grid>,
) {
    let image = images.get_mut(&handle.0).expect("Image not found");

    for x in 0..grid.size.width {
        for y in (0..grid.size.height).rev() {
            let pos = ElementPos::new(x, y);
            let color = image.get_color_at(pos.x, pos.y).unwrap();
            let kind = ElementKind::from_color( color ).unwrap();

            match kind {
                ElementKind::Empty | ElementKind::Stone => continue,
                ElementKind::Sand => {
                    sand_algorithm(image, &pos, color, &grid.size);
                },
            }
        }
    }

}

fn sand_algorithm(
    image: &mut Image,
    pos: &ElementPos,
    color: Color,
    grid_size: &GridSize
) {
    if pos.y < grid_size.height - 1 {
        let permb_elem_color = ElementKind::Empty.to_color();

        if image.get_color_at(pos.x, pos.y + 1).unwrap() == permb_elem_color {
            pos.set_color(image, permb_elem_color).unwrap();
            image.set_color_at(pos.x, pos.y + 1 , color).unwrap();

        } else if pos.x > 0 && image.get_color_at(pos.x - 1, pos.y + 1).unwrap() == permb_elem_color {
            pos.set_color(image, permb_elem_color).unwrap();
            image.set_color_at(pos.x - 1, pos.y + 1 , color).unwrap();

        } else if pos.x < grid_size.width - 1 && image.get_color_at(pos.x + 1, pos.y + 1).unwrap() == permb_elem_color {
            pos.set_color(image, permb_elem_color).unwrap();
            image.set_color_at(pos.x + 1, pos.y + 1 , color).unwrap();

        }

    }
}