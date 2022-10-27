use std::time::{Duration, Instant};
use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main};
use criterion::measurement::Measurement;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use rocksdb::{DB, Options, ReadOptions, WriteBatchWithTransaction};
use bench_rocksdb::{rand_batch, rand_db, rand_string};

pub fn bench_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("seek");
    for kv_size in [1,8,64,512] {
        for load in [1000000,1000000000,] {
            let bench_id = format!("db_size_{load}____kv_size_{kv_size}");
            let (path, db) = rand_db(load, kv_size);
            group.bench_function(bench_id.as_str(), move |b| {
                b.iter(|| {
                    let mut iter = db.raw_iterator();
                    iter.seek(rand_string(kv_size).as_str());
                });
            });
            DB::destroy(&Options::default(), path);
        }
    }

    group.finish();
}


criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_group);

criterion_main!(benches);
