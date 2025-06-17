use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use onion_generator::{generate_onion_address, generate_with_prefix};
use std::time::Duration;

fn bench_single_generation(c: &mut Criterion) {
    c.bench_function("generate_single_address", |b| {
        b.iter(|| {
            black_box(generate_onion_address().unwrap());
        })
    });
}

fn bench_prefix_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("prefix_generation");
    
    // Test with different prefix lengths
    for prefix_len in [1, 2, 3].iter() {
        let prefix = "a".repeat(*prefix_len);
        let prefixes = vec![prefix.clone()];
        
        group.bench_with_input(
            BenchmarkId::new("prefix_length", prefix_len),
            &prefixes,
            |b, prefixes| {
                b.iter(|| {
                    black_box(generate_with_prefix(prefixes).unwrap());
                })
            },
        );
    }
    
    group.finish();
}

fn bench_common_prefixes(c: &mut Criterion) {
    let mut group = c.benchmark_group("common_prefixes");
    group.measurement_time(Duration::from_secs(30)); // Longer measurement time
    
    // Test with commonly searched prefixes
    let test_cases = vec![
        ("single_char", vec!["a".to_string()]),
        ("two_chars", vec!["ab".to_string()]),
        ("three_chars", vec!["abc".to_string()]),
        ("multiple_prefixes", vec!["a".to_string(), "b".to_string(), "c".to_string()]),
    ];
    
    for (name, prefixes) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("prefix_type", name),
            &prefixes,
            |b, prefixes| {
                b.iter(|| {
                    black_box(generate_with_prefix(prefixes).unwrap());
                })
            },
        );
    }
    
    group.finish();
}

fn bench_crypto_operations(c: &mut Criterion) {
    use onion_generator::crypto::*;
    
    let mut group = c.benchmark_group("crypto_operations");
    
    group.bench_function("keypair_generation", |b| {
        b.iter(|| {
            black_box(generate_keypair());
        })
    });
    
    let secret_key = [0u8; 32];
    group.bench_function("expand_secret_key", |b| {
        b.iter(|| {
            black_box(expand_secret_key(&secret_key).unwrap());
        })
    });
    
    let public_key = [0u8; 32];
    group.bench_function("calculate_checksum", |b| {
        b.iter(|| {
            black_box(calculate_checksum(&public_key).unwrap());
        })
    });
    
    let data = b"hello world test data for encoding";
    group.bench_function("base32_encode", |b| {
        b.iter(|| {
            black_box(base32_encode(data));
        })
    });
    
    group.bench_function("base64_encode", |b| {
        b.iter(|| {
            black_box(base64_encode(data));
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_single_generation,
    bench_prefix_generation,
    bench_common_prefixes,
    bench_crypto_operations
);
criterion_main!(benches);
