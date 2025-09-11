use bevy::ecs::system::{Local, Single};
use crate::game::sandworld::{Elem, ElemKind, ElemPos, GridCells, GRID_SIZE};

pub fn main_interaction_loop(
    mut grid_cells: Single<&mut GridCells>,
    mut dir: Local<bool>
) {

    let grid_cells = grid_cells.as_mut();

    for y in (0..GRID_SIZE.height).rev() {

        let x_range = 
            if y % 2 == 0 
            && *dir { (0..GRID_SIZE.width).collect::<Vec<u32>>() }
            else if y % 2 != 0
            && *dir { (0..GRID_SIZE.width).rev().collect::<Vec<u32>>() }
            else if y % 2 == 0 { (0..GRID_SIZE.width).rev().collect::<Vec<u32>>() }
            else { (0..GRID_SIZE.width).collect::<Vec<u32>>() };

        for x in x_range {
            let pos = ElemPos::new(x, y);
            let elem = grid_cells.get_elem_at(pos).unwrap();

            if !elem.moved {
                match elem.kind {
                    ElemKind::Empty | ElemKind::Stone => continue,
                    ElemKind::Sand(_) => {
                        sand_algorithm(grid_cells, pos, *dir, elem.kind);
                    },
                }
            } else {
                grid_cells.set_elem_at(pos, Elem::new( elem.kind, false));
            }
        }
    }
    if *dir { *dir = false } else { *dir = true }
}

fn sand_algorithm(
    grid_cells: &mut GridCells,
    pos: ElemPos,
    dir: bool,
    sand: ElemKind
) {
    if pos.in_border_bottom() {
        let permb_elems = vec![ElemKind::Empty];

        if unchecked_set_color_down(grid_cells, pos, sand, &permb_elems) { return }
        else if dir {
            if set_color_leftdown(grid_cells, pos, sand, &permb_elems) { return }
            else if set_color_rightdown(grid_cells, pos, sand, &permb_elems) { return }
        } else {
            if set_color_rightdown(grid_cells, pos, sand, &permb_elems) { return }
            else if set_color_leftdown(grid_cells, pos, sand, &permb_elems) { return }
        }
    }
}

fn unchecked_set_color_down(grid_cells: &mut GridCells, pos: ElemPos, kind: ElemKind, permb_elems: &Vec<ElemKind>) -> bool {
    let down_pos = ElemPos::new(pos.x, pos.y + 1);
    let check_kind = grid_cells.get_elem_at(down_pos).unwrap().kind;
    if permb_elems.contains(&check_kind) {
        grid_cells.set_elem_at(pos, Elem::new(check_kind, false)).unwrap();
        grid_cells.set_elem_at(down_pos, Elem::new(kind, false)).unwrap();
        return true
    }
    return false
}

fn set_color_leftdown(grid_cells: &mut GridCells, pos: ElemPos, kind: ElemKind, permb_elems: &Vec<ElemKind>) -> bool {
    if pos.in_border_left() {
        let leftdown_pos = ElemPos::new(pos.x - 1, pos.y + 1);
        let check_kind = grid_cells.get_elem_at(leftdown_pos).unwrap().kind;
        if permb_elems.contains(&check_kind) {
            grid_cells.set_elem_at(pos, Elem::new(check_kind, false)).unwrap();
            grid_cells.set_elem_at(leftdown_pos, Elem::new(kind, true)).unwrap();
            return true
        }
    }
    return false
}

fn set_color_rightdown(grid_cells: &mut GridCells, pos: ElemPos, kind: ElemKind, permb_elems: &Vec<ElemKind>) -> bool {
    if pos.in_border_right() {
        let rightdown_pos = ElemPos::new(pos.x + 1, pos.y + 1);
        let check_kind = grid_cells.get_elem_at(rightdown_pos).unwrap().kind;
        if permb_elems.contains(&check_kind) {
            grid_cells.set_elem_at(pos, Elem::new(check_kind, false)).unwrap();
            grid_cells.set_elem_at(rightdown_pos, Elem::new(kind, true)).unwrap();
            return true
        }
    }
    return false
}