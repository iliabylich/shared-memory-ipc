mod config;

use jemallocator::Jemalloc;
use native_ipc_rust::Reader;
use std::time::Instant;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let mut reader = Reader::<{ config::QUEUE_SIZE }>::new(config::STORAGE_PREFIX).unwrap();

    let started_at = Instant::now();

    for _ in 0..config::MESSAGES_COUNT {
        if let Some(message) = reader.ipc_pop().unwrap() {
            assert_eq!(message.len(), config::MESSAGE_SIZE);
            // println!(
            //     "message {} {:?}",
            //     i + 1,
            //     String::from_utf8(message).unwrap()
            // );
        } else {
            panic!("No messages");
        }
    }

    let diff = (Instant::now() - started_at).as_secs_f64();
    let total_bytes = config::MESSAGES_COUNT * config::MESSAGE_SIZE;
    println!(
        "Time taken: {:.10} (total bytes: {}, throughput: {}b/s)",
        diff,
        total_bytes,
        total_bytes as f64 / diff
    );

    println!("Done");
}
