use core::time::Duration;

use crate::{
    process::thread,
    register_test,
    sync::mutex::SpinNoIrqLock,
    timer::{
        current_time_ms, current_time_us,
        timeout_task::{ksleep, TimeoutTaskFuture},
    },
    utils::async_utils::block_on,
};

const THREAD_RUN_TIME_MS: usize = 5000;
const READ_WORK_TIME_US: usize = 10;

const NUM_READERS: usize = 4;
const NUM_WRITERS: usize = 2;

#[derive(Debug, Clone, Copy)]
enum Competitor {
    Lockless,
    SpinLock,
    RwLock,
    RCU,
}

static LOCK: SpinNoIrqLock<()> = SpinNoIrqLock::new(());

struct BenchObj {
    seq: usize,
    payload: usize,
}

static mut BENCH_OBJ: BenchObj = BenchObj { seq: 0, payload: 0 };

fn do_read_work() {
    let expeired_time = current_time_us() + READ_WORK_TIME_US;
    let mut now = current_time_us();
    while now < expeired_time {
        now = current_time_us();
    }
}

async fn reader_thread(id: usize, competitor: Competitor) {
    let mut loops: usize = 0;

    let expired_time = current_time_ms() + THREAD_RUN_TIME_MS;
    let mut _dummy = 0;

    while current_time_ms() < expired_time {
        match competitor {
            Competitor::Lockless => {
                _dummy += unsafe { BENCH_OBJ.payload };
                do_read_work();
            }
            Competitor::SpinLock => {
                let _lock = LOCK.lock();
                _dummy += unsafe { BENCH_OBJ.payload };
                do_read_work();
            }
            Competitor::RwLock => {
                todo!()
            }
            Competitor::RCU => {
                todo!()
            }
        }
        loops += 1;
    }

    println!("Reader {} loops: {}", id, loops);
}

async fn writer_thread(id: usize, competitor: Competitor) {
    let mut loops: usize = 0;
    let expired_time = current_time_ms() + THREAD_RUN_TIME_MS;

    while current_time_ms() < expired_time {
        match competitor {
            Competitor::Lockless => unsafe {
                BENCH_OBJ.seq = current_time_us();
                BENCH_OBJ.payload = BENCH_OBJ.seq ^ (&BENCH_OBJ as *const BenchObj as usize);
            },
            Competitor::SpinLock => {
                let _lock = LOCK.lock();
                unsafe {
                    BENCH_OBJ.seq = current_time_us();
                    BENCH_OBJ.payload = BENCH_OBJ.seq ^ (&BENCH_OBJ as *const BenchObj as usize);
                }
            }
            Competitor::RwLock => {
                todo!()
            }
            Competitor::RCU => {
                todo!()
            }
        }
        loops += 1;
    }
    println!("Writer {} loops: {}", id, loops);
}

fn run_spec_testsuit(competitor: Competitor) {
    for id in 0..NUM_READERS {
        thread::spawn_kernel_thread(reader_thread(id, competitor));
    }
    for id in 0..NUM_WRITERS {
        thread::spawn_kernel_thread(writer_thread(id, competitor));
    }

    block_on(async move {
        ksleep(Duration::from_millis(THREAD_RUN_TIME_MS as u64 + 1000)).await;
    })
}

fn test_lock_perf() {
    run_spec_testsuit(Competitor::Lockless);
    run_spec_testsuit(Competitor::SpinLock);
    // run_spec_testsuit(Competitor::RwLock);
    // run_spec_testsuit(Competitor::RCU);
}

pub fn init() {
    register_test!("lock_perf", test_lock_perf);
}
