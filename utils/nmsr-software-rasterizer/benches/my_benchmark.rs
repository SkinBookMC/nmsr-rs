use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};

use glam::Vec3;
pub use nmsr_rasterizer_test::{camera::CameraRotation, model::{RenderEntry, Size}, shader::ShaderState};
use nmsr_rasterizer_test::shader;


fn bench(c: &mut Criterion) {
    let mut camera = nmsr_rasterizer_test::camera::Camera::new_orbital(
        Vec3::new(0.0, 16.5 + 2.5, 0.0),
        45.0 + 3.5 + 4.0,
        CameraRotation {
            yaw: 20f32,
            pitch: 10f32,
            roll: 0f32,
        },
        nmsr_rasterizer_test::camera::ProjectionParameters::Perspective { fov: 45f32 },
        Some(Size {
            width: 512,
            height: 869
        }),
    );
    
    let mut texture = image::open("NickAc.png").unwrap().into_rgba8();
    
    ears_rs::utils::process_erase_regions(&mut texture).expect("Failed to process erase regions");
    ears_rs::utils::strip_alpha(&mut texture);
    
    let state = ShaderState {
        transform: camera.get_view_projection_matrix(),
        texture,
        sun: shader::SunInformation {
            direction: glam::Vec3::new(0.0, -1.0, 1.0),
            intensity: 2.0,
            ambient: 0.621,
        },
    };
    
    
    let mut entry = RenderEntry::new((512, 869).into());
    let mut group = c.benchmark_group("nmsr-rs");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("render_entry", |b| b.iter(|| entry.draw(&state)));
    group.finish();
    entry.dump();
}

criterion_group!(benches, bench);
criterion_main!(benches);