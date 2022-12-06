use jemallocator::Jemalloc;
use libc::{signal, SIGINT};
use native_ipc_rust::Writer;
use rand::{seq::SliceRandom, thread_rng};
use std::time::Instant;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod config;

static mut RUNNING: bool = true;
static mut MESSAGES_SENT: usize = 0;

fn on_interrupt() {
    unsafe { RUNNING = false }
}

fn random_message(size: usize) -> Vec<u8> {
    let mut vec = [b'a', b'b']
        .iter()
        .cycle()
        .take(size)
        .cloned()
        .collect::<Vec<_>>();
    vec.shuffle(&mut thread_rng());
    vec
}

fn main() {
    let mut writer = Writer::<{ config::QUEUE_SIZE }>::new(config::STORAGE_PREFIX).unwrap();

    unsafe { signal(SIGINT, on_interrupt as usize) };

    let started_at = Instant::now();

    while unsafe { RUNNING && MESSAGES_SENT < config::MESSAGES_COUNT } {
        let message = random_message(config::MESSAGE_SIZE);
        writer.ipc_push(&message).unwrap();
        unsafe { MESSAGES_SENT += 1 }
        // println!(
        //     "Send message {:?} no {}",
        //     std::str::from_utf8(&message).unwrap(),
        //     unsafe { MESSAGES_SENT + 1 }
        // );
        std::thread::sleep(std::time::Duration::from_millis(config::PUSH_DELAY));
    }

    let diff = (Instant::now() - started_at).as_secs_f64();
    let total_bytes = config::MESSAGES_COUNT * config::MESSAGE_SIZE;
    println!(
        "Time taken: {:.10} (total bytes: {}, throughput: {}b/s)",
        diff,
        total_bytes,
        total_bytes as f64 / diff
    );

    while unsafe { RUNNING } {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
