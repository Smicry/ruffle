//! Tests for the empty-data guard in `WgpuRenderBackend::update_texture`.

use ruffle_render::backend::RenderBackend;
use ruffle_render::bitmap::{Bitmap, BitmapFormat, PixelRegion};
use ruffle_render_wgpu::backend::WgpuRenderBackend;
use ruffle_render_wgpu::target::TextureTarget;
use ruffle_render_wgpu::wgpu;

/// Build an offscreen backend, or `None` if no wgpu adapter is available.
fn try_make_backend() -> Option<WgpuRenderBackend<TextureTarget>> {
    WgpuRenderBackend::for_offscreen(
        (16, 16),
        wgpu::Backends::all(),
        wgpu::PowerPreference::LowPower,
    )
    .ok()
}

/// Empty-data bitmap must return Ok instead of panicking on a `0..64` slice
/// over empty data.
#[test]
fn update_texture_with_empty_data_returns_ok() {
    let Some(mut backend) = try_make_backend() else {
        eprintln!("No wgpu adapter; skipping");
        return;
    };

    let handle = backend
        .register_bitmap(Bitmap::new(4, 4, BitmapFormat::Rgba, vec![0u8; 64]))
        .expect("register_bitmap");

    let empty = Bitmap::new(0, 0, BitmapFormat::Rgba, Vec::<u8>::new());
    assert!(empty.data().is_empty());

    let result = backend.update_texture(&handle, empty, PixelRegion::for_whole_size(4, 4));
    assert!(result.is_ok(), "expected Ok, got {:?}", result.err());
}

/// Non-empty path still works.
#[test]
fn update_texture_with_non_empty_data_still_works() {
    let Some(mut backend) = try_make_backend() else {
        eprintln!("No wgpu adapter; skipping");
        return;
    };

    let handle = backend
        .register_bitmap(Bitmap::new(4, 4, BitmapFormat::Rgba, vec![0u8; 64]))
        .expect("register_bitmap");

    backend
        .update_texture(
            &handle,
            Bitmap::new(4, 4, BitmapFormat::Rgba, vec![255u8; 64]),
            PixelRegion::for_whole_size(4, 4),
        )
        .expect("update_texture");
}
