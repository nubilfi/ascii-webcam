// File: benches/ascii_conversion.rs

use ascii_webcam::ascii::process_frame;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opencv::imgcodecs;

fn benchmark_process_frame(c: &mut Criterion) {
    // Load a sample image
    let img = imgcodecs::imread("assets/pexels-cat.jpg", imgcodecs::IMREAD_COLOR)
        .expect("Failed to load sample image");

    c.bench_function("process_frame 640x480", |b| {
        b.iter(|| process_frame(black_box(&img), black_box(640), black_box(480)))
    });

    c.bench_function("process_frame 1280x720", |b| {
        b.iter(|| process_frame(black_box(&img), black_box(1280), black_box(720)))
    });
}

criterion_group!(benches, benchmark_process_frame);
criterion_main!(benches);
