use std::cmp::min;
use std::time::Instant;
use log::info;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use rocksdb::{DB, DBWithThreadMode, Options, SingleThreaded, WriteBatchWithTransaction};

pub fn rand_string(len:usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), len)
}

pub fn rand_batch(size:usize, key_length:usize, value_length:usize) -> WriteBatchWithTransaction<false> {
    let mut batch = WriteBatchWithTransaction::<false>::default();
    for i in 0..size {
        batch.put(rand_string(key_length).as_str(), rand_string(value_length).as_str());
    }
    batch
}

pub fn rand_db(preload_key_count:usize, kv_size:usize) -> (String, DBWithThreadMode<SingleThreaded>) {
    let log_target_name = rand_string(32);
    let log_target = log_target_name.as_str();
    info!(target:log_target, "preload_key_count={preload_key_count}");
    info!(target:log_target, "kv_size={kv_size}");
    let file_name = rand_string(32);
    let path = format!("/tmp/{file_name}");
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts,path.as_str()).unwrap();

    // Grow DB to target size in small batches.
    let mut remaining = preload_key_count;
    while remaining > 0 {
        let cur_batch_size = min(remaining, 1000);
        remaining-=cur_batch_size;

        let timer1 = Instant::now();
        let preloading_batch = rand_batch(cur_batch_size, kv_size, kv_size);
        let batch_creation_time = timer1.elapsed();
        info!(target:log_target, "batch_creation_time={batch_creation_time:?}");

        let timer2 = Instant::now();
        db.write(preloading_batch);
        let preload_time = timer2.elapsed();
        info!(target:log_target, "preload_time={preload_time:?}");
    }

    (path, db)
}