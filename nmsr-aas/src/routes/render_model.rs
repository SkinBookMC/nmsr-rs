use deadpool::managed::Object;
use image::{ImageFormat, RgbaImage};
use nmsr_rendering::{
    errors::NMSRRenderingError,
    high_level::{
        parts::provider::PlayerPartProviderContext,
        pipeline::{
            pools::SceneContextPoolManager,
            scene::Scene,
        },
        player_model::PlayerModel,
    },
};
use tracing::instrument;

use crate::{
    error::Result,
    model::{
        request::{RenderRequest, RenderRequestFeatures},
        resolver::{ResolvedRenderEntryTextureType, ResolvedRenderRequest},
    },
};

use super::{render::create_png_from_bytes, NMSRState};

pub(crate) async fn internal_render_model(
    request: RenderRequest,
    state: &NMSRState,
    resolved: ResolvedRenderRequest,
) -> Result<Vec<u8>> {
    let scene_context = state.create_scene_context().await?;

    let size = request.get_size();

    let mode = &request.mode;
    let camera = request.get_camera();
    let arm_rotation = mode.get_arm_rotation();
    let lighting = mode.get_lighting(!request.features.contains(RenderRequestFeatures::Shading));
    let parts = mode.get_body_parts();

    let final_model = request.model.unwrap_or(resolved.model);

    let has_layers = request.features.contains(RenderRequestFeatures::BodyLayers);
    let has_cape = request.features.contains(RenderRequestFeatures::Cape)
        && resolved
            .textures
            .contains_key(&ResolvedRenderEntryTextureType::Cape);

    let part_context = PlayerPartProviderContext {
        model: PlayerModel::from(final_model),
        has_layers, // TODO - Hat layers
        has_cape,
        arm_rotation,
    };

    let mut scene = Scene::new(
        &state.graphics_context,
        scene_context,
        camera,
        lighting,
        size,
        &part_context,
        parts,
    );

    load_textures(resolved, &state, &request, &mut scene)?;

    scene.render(&state.graphics_context)?;

    let render = scene.copy_output_texture(&state.graphics_context).await?;

    let render_bytes = create_png_from_bytes((size.width, size.height), &render)?;

    Ok(render_bytes)
}

#[instrument(skip_all)]
fn load_textures(
    resolved: ResolvedRenderRequest,
    state: &NMSRState,
    request: &RenderRequest,
    scene: &mut Scene<Object<SceneContextPoolManager>>,
) -> Result<()> {
    for (texture_type, texture_bytes) in resolved.textures {
        let mut image_buffer = load_image(&texture_bytes)?;

        if texture_type == ResolvedRenderEntryTextureType::Skin {
            image_buffer = state.process_skin(image_buffer, request.features)?;
        }

        scene.set_texture(&state.graphics_context, texture_type.into(), &image_buffer);
    }

    Ok(())
}

fn load_image(texture: &[u8]) -> Result<RgbaImage> {
    let img = image::load_from_memory_with_format(&texture, ImageFormat::Png)
        .map_err(|_| NMSRRenderingError::ImageFromRawError)?;
    Ok(img.into_rgba8())
}