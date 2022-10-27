use std::time::Duration;
use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main};
use criterion::measurement::Measurement;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use rocksdb::{DB, Options, WriteBatchWithTransaction};
use bench_rocksdb;
use bench_rocksdb::{rand_batch, rand_db, rand_string};

pub fn bench_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_write");

    /// The goal is to benchmark write latency on different db size, but writes can change db size.
    /// The idea here is to reuse the DB, limit sample size, and assume the DB size are not changed
    /// a lot by the samples.

    // Set an impractical value so `sample_size` can take control.
    group.measurement_time(Duration::from_millis(1));
    group.warm_up_time(Duration::from_millis(1));
    group.sample_size(100);
    for load in [1000000,1000000000] {
        for batch_size in [1,1000,] {
            let bench_id = format!("db_size_{load}____write_batch_size_{batch_size}");
            let (path, db) = rand_db(load, 64);
            group.bench_function(bench_id.as_str(), move |b| {
                b.iter(|| {
                    let batch = rand_batch(batch_size, 32, 64);
                    db.write(batch);
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
