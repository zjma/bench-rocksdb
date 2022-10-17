use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main};
use criterion::measurement::Measurement;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use rocksdb::{DB, Options, WriteBatchWithTransaction};

fn rand_string(len:usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), len)
}

fn rand_batch(size:usize) -> WriteBatchWithTransaction<false> {
    let mut batch = WriteBatchWithTransaction::<false>::default();
    for i in 0..size {
        batch.put(rand_string(32).as_str(), rand_string(32).as_str());
    }
    batch
}

pub fn bench_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("rocksdb_batch_write_performance");

    for load in [1,100,10000,1000000] {
        for batch_size in [0,1,100,10000,1000000] {
            let bench_id = format!("db_size_{load}____write_batch_size_{batch_size}");
            group.bench_function(bench_id.as_str(), move |b| {
                b.iter(|| {
                    // Create DB.
                    let file_name = rand_string(32);
                    let path = format!("/tmp/{file_name}");
                    let db = DB::open_default(path.as_str()).unwrap();

                    // Grow DB to target size.
                    let preloading_batch = rand_batch(load);
                    db.write(preloading_batch);

                    // The actual batch write.
                    if batch_size >= 1 {
                        let subject_batch = rand_batch(batch_size);
                        db.write(subject_batch);
                    }

                    // Destroy DB.
                    let _ = DB::destroy(&Options::default(), path);
                });
            });
        }
    }

    group.finish();
}


criterion_group!(
    name = benches;
    //config = Criterion::default().sample_size(10);
    config = Criterion::default();
    targets = bench_group);

criterion_main!(benches);
