use bevy::ecs::system::{Local, Single};
use crate::game::sandworld::{ElemKind, ElemPos, GridCells, GRID_SIZE};


pub fn main_interaction_loop(
    mut grid_cells: Single<&mut GridCells>,
    mut dir: Local<bool>
) {

    let grid_cells = grid_cells.as_mut();

    for x in 0..GRID_SIZE.width {
        for y in (0..GRID_SIZE.height).rev() {
            let pos = ElemPos::new(x, y);
            let kind = grid_cells.get_elem_at(pos).unwrap();

            match kind {
                ElemKind::Empty | ElemKind::Stone => continue,
                ElemKind::Sand => {
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
        let sand = ElemKind::Sand;
        let permb_elems = vec![ElemKind::Empty];

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


fn unchecked_set_color_down(grid_cells: &mut GridCells, pos: ElemPos, kind: ElemKind, permb_elems: &Vec<ElemKind>) -> bool {
    let down_pos = ElemPos::new(pos.x, pos.y + 1);
    let check_kind = grid_cells.get_elem_at(down_pos).unwrap();
    if permb_elems.contains(&check_kind) {
        grid_cells.set_elem_at(pos, check_kind).unwrap();
        grid_cells.set_elem_at(down_pos, kind).unwrap();
        return true
    }
    return false
}

fn set_color_leftdown(grid_cells: &mut GridCells, pos: ElemPos, kind: ElemKind, permb_elems: &Vec<ElemKind>) -> bool {
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

fn set_color_rightdown(grid_cells: &mut GridCells, pos: ElemPos, kind: ElemKind, permb_elems: &Vec<ElemKind>) -> bool {
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