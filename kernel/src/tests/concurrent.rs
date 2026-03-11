use core::time::Duration;

use alloc::{boxed::Box, string::ToString};

use crate::{
    process::thread,
    processor::local_hart,
    register_test,
    sync::{mutex::SpinNoIrqLock, rcu::RcuBox},
    timer::{current_time_ms, current_time_us, timeout_task::ksleep},
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

static mut BENCH_OBJ_RCU: Option<RcuBox<BenchObj>> = None;

fn do_read_work(obj: &BenchObj) -> usize {
    let expeired_time = current_time_us() + READ_WORK_TIME_US;
    let mut now = current_time_us();
    let val = obj.payload;
    while now < expeired_time {
        now = current_time_us();
    }
    val
}

async fn reader_thread(id: usize, competitor: Competitor) {
    let mut loops: usize = 0;

    let expired_time = current_time_ms() + THREAD_RUN_TIME_MS;
    let mut dummy = 0;

    println!("reader start..");
    while current_time_ms() < expired_time {
        match competitor {
            Competitor::Lockless => {
                // dummy += unsafe { BENCH_OBJ.payload };
                dummy += unsafe { do_read_work(&BENCH_OBJ) };
            }
            Competitor::SpinLock => {
                let _lock = LOCK.lock();
                dummy += unsafe { do_read_work(&BENCH_OBJ) };
            }
            Competitor::RwLock => {
                todo!()
            }
            Competitor::RCU => {
                // println!("1 {}", local_hart().hart_id());
                log::info!("reader do rcu read...");
                let guard = unsafe { BENCH_OBJ_RCU.as_ref().unwrap() }.rcu_read();
                log::info!("reader got the guard");
                dummy += do_read_work(guard.as_ref());
                log::info!("reader do rcu read done");
            }
        }
        loops += 1;
        thread::yield_now().await;
    }

    println!("Reader {} loops: {}, val {}", id, loops, dummy);
}

async fn writer_thread(id: usize, competitor: Competitor) {
    let mut loops: usize = 0;
    let expired_time = current_time_ms() + THREAD_RUN_TIME_MS;

    println!("writer start..");

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
                log::info!("writer do rcu write...");
                let seq = current_time_us();
                let payload = unsafe { seq ^ (&BENCH_OBJ as *const BenchObj as usize) };
                let new_obj = RcuBox::new(BenchObj { seq, payload });
                unsafe {
                    BENCH_OBJ_RCU.as_ref().unwrap().rcu_write(new_obj);
                }
                log::info!("writer do rcu write done");
            }
        }
        loops += 1;
        thread::yield_now().await;
    }
    println!("Writer {} loops: {}", id, loops);
}

fn run_spec_testsuit(competitor: Competitor) {
    println!("Running testsuit, competitor: {:?}...", competitor);
    for id in 0..NUM_READERS {
        thread::spawn_kernel_thread(reader_thread(id, competitor), "reader".to_string());
    }
    for id in 0..NUM_WRITERS {
        thread::spawn_kernel_thread(writer_thread(id, competitor), "writer".to_string());
    }

    block_on(async move {
        ksleep(Duration::from_millis(THREAD_RUN_TIME_MS as u64 + 1000)).await;
    });

    println!("Running testsuit, competitor: {:?} Done", competitor);
}

fn test_lock_perf() {
    unsafe {
        BENCH_OBJ_RCU = Some(RcuBox::new(BenchObj { seq: 0, payload: 0 }));
    }
    run_spec_testsuit(Competitor::Lockless);
    run_spec_testsuit(Competitor::SpinLock);
    run_spec_testsuit(Competitor::RCU);
    // run_spec_testsuit(Competitor::RwLock);
}

pub fn init() {
    register_test!("lock_perf", test_lock_perf);
}
