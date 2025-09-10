use bevy::{asset::Assets, ecs::system::{Res, ResMut, Single}, image::Image};

use crate::game::sandworld::{ElemPos, GridCells, GridImage, GRID_SIZE};

pub fn draw_image(
    grid_cells: Single<&GridCells>,
    handle: Res<GridImage>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = images.get_mut(&handle.0).expect("Image not found");

    for x in 0..GRID_SIZE.width {
        for y in 0..GRID_SIZE.height {
            let elem_pos = ElemPos::new(x, y);
            let elem_color = grid_cells
                .get_elem_at(elem_pos)
                .unwrap()
                .get_varied_color_from_position(elem_pos);
            image.set_color_at(x, y, elem_color).unwrap();
        }
    }
}