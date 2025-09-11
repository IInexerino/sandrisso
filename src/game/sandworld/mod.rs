use std::{fmt::Display};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use bevy::asset::Handle;
use bevy::ecs::resource::Resource;
use bevy::image::Image;
use bevy::{color::Color, ecs::component::Component};

pub mod draw_image;
pub mod image_setup;
pub mod user_element_interraction;
pub mod main_interaction;

const GRID_SIZE: GridSize = GridSize::new(256, 192);
const GRID_SCALE: f32 = 5.;
const EMPTY_COLOR: Color = Color::srgba(0., 0., 0., 0.);

#[derive(Resource)]
pub struct GridImage(pub Handle<Image>);

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

#[derive(Component)]
pub struct GridParams {
    pub scale: f32,
}
#[derive(Component)]
pub struct GridCells {
    pub cells: [Elem; GRID_SIZE.count()]
}
impl GridCells {
    pub fn new_empty() -> Self {
        GridCells { cells: [ Elem::new(ElemKind::Empty, false) ; GRID_SIZE.count() ] }
    }
    pub fn get_elem_at(&self, pos: ElemPos) -> Option<Elem> {
        if pos.in_bounds() {
            Some( self.cells[(pos.y * GRID_SIZE.width + pos.x) as usize] )
        } else { None }
    }
    pub fn set_elem_at(&mut self, pos: ElemPos, elem: Elem) -> Option<()> {
        if pos.in_bounds() {
            self.cells[(pos.y * GRID_SIZE.width + pos.x) as usize] = elem; 

            Some(())
        } else { None }
    }
}

#[derive(Copy, Clone)]
pub struct Elem {
    kind: ElemKind,
    moved: bool
}
impl Elem {
    pub fn new(kind: ElemKind, moved: bool) -> Self {
        Elem { kind, moved }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ElemKind {
    Empty,
    Stone,
    Sand(SandColor),
}
impl ElemKind {
    pub fn get_base_color(&self) -> Color {
        match self {
            ElemKind::Empty => EMPTY_COLOR,
            ElemKind::Sand(sand_color) => { 
                match sand_color {
                    SandColor::Yellow => Color::srgba(0.95, 0.82, 0.20, 1.0),    // Bright golden yellow
                    SandColor::Red => Color::srgba(0.92, 0.25, 0.25, 1.0),       // Vibrant red
                    SandColor::Blue => Color::srgba(0.20, 0.45, 0.95, 1.0),      // Deeper, more saturated blue
                    SandColor::Green => Color::srgba(0.25, 0.85, 0.25, 1.0),     // Brighter, more vibrant green
                }
            },
            ElemKind::Stone => Color::srgba(0.45, 0.45, 0.45, 1.0),
        }
    }
    
    pub fn get_varied_color_from_position(&self, pos: ElemPos) -> Color {
        match self {
            ElemKind::Sand(sand_color) => {
                let mut hasher = DefaultHasher::new();
                pos.x.hash(&mut hasher);
                pos.y.hash(&mut hasher);
                let hash = hasher.finish();
                    
                let r_variation = ((hash >> 0) % 32) as f32 / 32.0;
                let g_variation = ((hash >> 8) % 32) as f32 / 32.0;
                let b_variation = ((hash >> 16) % 32) as f32 / 32.0;
                
                match sand_color {
                    SandColor::Yellow => {
                        let r = (0.95f32 + r_variation * 0.15 - 0.075).clamp(0.8, 1.0);
                        let g = (0.82f32 + g_variation * 0.20 - 0.10).clamp(0.7, 0.95);
                        let b = (0.20f32 + b_variation * 0.25 - 0.125).clamp(0.1, 0.4);
                        Color::linear_rgb(r, g, b)
                    }
                    SandColor::Red => {
                        let r = (0.92f32 + r_variation * 0.20 - 0.10).clamp(0.75, 1.0);
                        let g = (0.25f32 + g_variation * 0.25 - 0.125).clamp(0.15, 0.45);
                        let b = (0.25f32 + b_variation * 0.20 - 0.10).clamp(0.15, 0.4);
                        Color::linear_rgb(r, g, b)
                    }
                    SandColor::Blue => {
                        // Deep blue with minimal green component
                        let r = (0.20f32 + r_variation * 0.15 - 0.075).clamp(0.1, 0.3);
                        let g = (0.45f32 + g_variation * 0.20 - 0.10).clamp(0.35, 0.6);
                        let b = (0.95f32 + b_variation * 0.15 - 0.075).clamp(0.8, 1.0);
                        Color::linear_rgb(r, g, b)
                    }
                    SandColor::Green => {
                        // Vibrant green with minimal blue component
                        let r = (0.25f32 + r_variation * 0.15 - 0.075).clamp(0.15, 0.35);
                        let g = (0.85f32 + g_variation * 0.15 - 0.075).clamp(0.75, 0.95);
                        let b = (0.25f32 + b_variation * 0.15 - 0.075).clamp(0.15, 0.35);
                        Color::linear_rgb(r, g, b)
                    }
                }
            }
            _ => self.get_base_color(),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum SandColor {
    #[default]
    Yellow,
    Red,
    Blue,
    Green
}

impl Display for ElemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElemKind::Empty => write!(f, "[Empty]"),
            ElemKind::Stone => write!(f, "[Stone]"),
            ElemKind::Sand(sand_color) => {
                match sand_color {
                    SandColor::Yellow => write!(f, "[Sand(Yellow)]"),
                    SandColor::Red => write!(f, "[Sand(Red)]"),
                    SandColor::Blue => write!(f, "[Sand(Blue)]"),
                    SandColor::Green => write!(f, "[Sand(Green)]"),
                }
            },
        }
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