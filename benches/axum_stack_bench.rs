use criterion::{criterion_group, criterion_main, Criterion};
use axum_stack::cors;
use axum_stack::request_id;

fn bench_cors_permissive(c: &mut Criterion) {
    c.bench_function("cors_permissive", |b| {
        b.iter(|| {
            let layer = cors::cors_permissive();
            let _ = std::hint::black_box(layer);
        });
    });
}

fn bench_cors_restrictive_single_origin(c: &mut Criterion) {
    c.bench_function("cors_restrictive_single_origin", |b| {
        b.iter(|| {
            let layer = cors::cors_restrictive(&["https://app.example.com"], true);
            let _ = std::hint::black_box(layer);
        });
    });
}

fn bench_cors_restrictive_multiple_origins(c: &mut Criterion) {
    c.bench_function("cors_restrictive_multiple_origins", |b| {
        b.iter(|| {
            let layer = cors::cors_restrictive(
                &["https://app.example.com", "https://admin.example.com"],
                false,
            );
            let _ = std::hint::black_box(layer);
        });
    });
}

fn bench_cors_api(c: &mut Criterion) {
    c.bench_function("cors_api", |b| {
        b.iter(|| {
            let layer = cors::cors_api(&["https://api.example.com"]);
            let _ = std::hint::black_box(layer);
        });
    });
}

fn bench_request_id_layer(c: &mut Criterion) {
    c.bench_function("request_id_layer", |b| {
        b.iter(|| {
            let (set_id, propagate_id) = request_id::request_id_layer();
            let _ = std::hint::black_box(set_id);
            let _ = std::hint::black_box(propagate_id);
        });
    });
}

criterion_group!(
    benches,
    bench_cors_permissive,
    bench_cors_restrictive_single_origin,
    bench_cors_restrictive_multiple_origins,
    bench_cors_api,
    bench_request_id_layer,
);
criterion_main!(benches);
