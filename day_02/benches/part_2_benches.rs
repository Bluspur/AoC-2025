use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main};
use solution::part_2::*;
use solution::*;

// Per the test data.
fn setup_test_database() -> ShopDatabase {
    ShopDatabase::new(&[
        Range::new(11, 22),
        Range::new(95, 115),
        Range::new(998, 1012),
        Range::new(1188511880, 1188511890),
        Range::new(222220, 222224),
        Range::new(1698522, 1698528),
        Range::new(446443, 446449),
        Range::new(38593856, 38593862),
        Range::new(565653, 565659),
        Range::new(824824821, 824824827),
        Range::new(2121212118, 2121212124),
    ])
}

fn load_file() -> ShopDatabase {
    let raw = std::fs::read_to_string("D:/code/AoC-2025/day_02/inputs/actual.txt").unwrap();
    ShopDatabase::from_str(&raw).unwrap()
}

fn benchmark_invalid_multi_test(c: &mut Criterion) {
    let db = setup_test_database();
    c.bench_function("multi-threaded | test data", |b| {
        b.iter(|| sum_all_invalid_in_db_multi(&db))
    });
}

fn benchmark_invalid_multi_actual(c: &mut Criterion) {
    // let db = setup_test_database();
    let db = load_file();
    c.bench_function("multi-threaded | actual data", |b| {
        b.iter(|| sum_all_invalid_in_db_multi(&db))
    });
}

fn benchmark_invalid_single_test(c: &mut Criterion) {
    let db = setup_test_database();
    c.bench_function("single-threaded | test data", |b| {
        b.iter(|| sum_all_invalid_in_db_single(&db))
    });
}

fn benchmark_invalid_single_actual(c: &mut Criterion) {
    let db = load_file();
    c.bench_function("single-threaded | actual data", |b| {
        b.iter(|| sum_all_invalid_in_db_single(&db))
    });
}

criterion_group!(
    benches,
    benchmark_invalid_multi_actual,
    benchmark_invalid_single_actual,
    benchmark_invalid_multi_test,
    benchmark_invalid_single_test
);
criterion_main!(benches);
