use bevy::{asset::{Assets, RenderAssetUsages}, color::ColorToPacked, ecs::system::{Commands, ResMut}, image::Image, math::Vec3, render::render_resource::{Extent3d, TextureDimension, TextureFormat}, sprite::Sprite, transform::components::Transform};
use crate::game::sandworld::{GridCells, GridImage, GridParams, EMPTY_COLOR, GRID_SCALE, GRID_SIZE};

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
