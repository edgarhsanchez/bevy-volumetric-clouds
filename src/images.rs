use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use crate::config::CloudsConfig;

pub const IMAGE_SIZE: u32 = 1920;

pub(crate) fn cloud_output_size(config: &CloudsConfig) -> UVec2 {
    UVec2::new(
        config.render_resolution.x.ceil().max(1.0) as u32,
        config.render_resolution.y.ceil().max(1.0) as u32,
    )
}

pub(crate) fn build_cloud_output_image(size: UVec2) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: size.x.max(1),
            height: size.y.max(1),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; 4 * 4 * 2],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    image
}

pub(crate) fn upsert_cloud_output_image(
    images: &mut Assets<Image>,
    handle: &mut Handle<Image>,
    size: UVec2,
) {
    let image = build_cloud_output_image(size);
    if let Some(existing) = images.get_mut(&*handle) {
        *existing = image;
    } else {
        *handle = images.add(image);
    }
}

pub fn build_images(
    mut images: ResMut<Assets<Image>>,
    output_size: UVec2,
) -> (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>) {
    let cloud_render_image = build_cloud_output_image(output_size);

    let mut cloud_atlas_image = Image::new_fill(
        Extent3d {
            width: IMAGE_SIZE,
            height: IMAGE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; 4 * 4 * 2],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    cloud_atlas_image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let mut cloud_worley_image = Image::new_fill(
        Extent3d {
            width: 32,
            height: 32,
            depth_or_array_layers: 32,
        },
        TextureDimension::D3,
        &[0; 4 * 4 * 2],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    cloud_worley_image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let sky_image = build_cloud_output_image(output_size);

    (
        images.add(cloud_render_image),
        images.add(cloud_atlas_image),
        images.add(cloud_worley_image),
        images.add(sky_image),
    )
}
