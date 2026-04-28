#![cfg(not(target_family = "wasm"))]
//! Tests for the empty-data guard in `WgpuRenderBackend::update_texture`.

use ruffle_render::backend::RenderBackend;
use ruffle_render::bitmap::{Bitmap, BitmapFormat, PixelRegion};
use ruffle_render_wgpu::backend::WgpuRenderBackend;
use ruffle_render_wgpu::target::TextureTarget;
use ruffle_render_wgpu::wgpu;

const TEXTURE_SIZE: u32 = 4;
const RGBA_BYTES: usize = (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize;

/// Build an offscreen backend; panics if no wgpu adapter is available.
fn make_backend() -> WgpuRenderBackend<TextureTarget> {
    WgpuRenderBackend::for_offscreen(
        (16, 16),
        wgpu::Backends::all(),
        wgpu::PowerPreference::default(),
    )
    .expect("wgpu adapter required for update_texture tests")
}

/// Register a 4x4 RGBA bitmap filled with zeros and return its handle.
fn register_blank_bitmap(
    backend: &mut WgpuRenderBackend<TextureTarget>,
) -> ruffle_render::bitmap::BitmapHandle {
    backend
        .register_bitmap(Bitmap::new(
            TEXTURE_SIZE,
            TEXTURE_SIZE,
            BitmapFormat::Rgba,
            vec![0u8; RGBA_BYTES],
        ))
        .expect("register_bitmap")
}

/// Empty-data bitmap must return Ok instead of panicking on a `0..64` slice
/// over empty data.
#[test]
fn update_texture_with_empty_data_returns_ok() {
    let mut backend = make_backend();
    let handle = register_blank_bitmap(&mut backend);

    let empty = Bitmap::new(0, 0, BitmapFormat::Rgba, Vec::<u8>::new());
    assert!(
        empty.data().is_empty(),
        "precondition: bitmap data is empty"
    );

    backend
        .update_texture(
            &handle,
            empty,
            PixelRegion::for_whole_size(TEXTURE_SIZE, TEXTURE_SIZE),
        )
        .expect("update_texture with empty bitmap must return Ok");
}

/// Complementary branch: non-empty data must still reach the normal write path
/// and succeed, guarding against the guard being widened to short-circuit all
/// updates.
#[test]
fn update_texture_with_non_empty_data_still_works() {
    let mut backend = make_backend();
    let handle = register_blank_bitmap(&mut backend);

    backend
        .update_texture(
            &handle,
            Bitmap::new(
                TEXTURE_SIZE,
                TEXTURE_SIZE,
                BitmapFormat::Rgba,
                vec![255u8; RGBA_BYTES],
            ),
            PixelRegion::for_whole_size(TEXTURE_SIZE, TEXTURE_SIZE),
        )
        .expect("update_texture");
}
