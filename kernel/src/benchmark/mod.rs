use core::sync::atomic::AtomicBool;

use crate::timer::current_time_duration;

const ITERS: usize = 2000;
const BUF_LEN: usize = 4096;

static EVENT_START: AtomicBool = AtomicBool::new(false);
static EVENT_FINISH: AtomicBool = AtomicBool::new(false);

static mut EVENT: Event = Event {
    src_ptr: 0,
    dst_ptr: 0,
    len: 0,
};

struct Event {
    src_ptr: usize,
    dst_ptr: usize,
    len: usize,
}

pub fn smp_cp_test(hart_id: usize) {
    println!("start to run smp_cp_test..., hart {}", hart_id);
    let iters = ITERS;

    if hart_id == 1 {
        leader(iters, true);
    } else {
        follower(iters, true);
    }

    if hart_id == 1 {
        leader(iters, false);
    } else {
        follower(iters, false);
    }
    println!("smp_cp_test finished");
}

fn leader(iters: usize, is_smp: bool) {
    let start_ts = current_time_duration();

    let mut src_buf = [0; BUF_LEN];
    let mut dst_buf = [0; BUF_LEN];

    let mut glo_sum = 0;

    for i in 0..iters {
        log::info!("round {}", i);
        src_buf.fill(i);
        match is_smp {
            true => {
                unsafe {
                    EVENT.dst_ptr = dst_buf.as_ptr().add(BUF_LEN / 2) as usize;
                    EVENT.src_ptr = src_buf.as_ptr().add(BUF_LEN / 2) as usize;
                    EVENT.len = BUF_LEN / 2;
                }
                EVENT_START.store(true, core::sync::atomic::Ordering::Release);
                dst_buf[..BUF_LEN / 2].copy_from_slice(&src_buf[..BUF_LEN / 2]);
                while !EVENT_FINISH.load(core::sync::atomic::Ordering::Acquire) {}
                EVENT_FINISH.store(false, core::sync::atomic::Ordering::Release);
            }
            false => {
                dst_buf.copy_from_slice(&src_buf);
            }
        }
        let mut sum = 0;
        for i in dst_buf {
            sum += dst_buf[i];
        }
        glo_sum += sum;
    }

    let end_ts = current_time_duration();
    println!("[benchmark][smp-cp] sum {}", glo_sum);

    println!(
        "[benchmark][smp-cp] is smp {}, time consumed {:?}",
        is_smp,
        end_ts - start_ts
    );
}

fn follower(iters: usize, is_smp: bool) {
    log::info!("enter follower");
    if !is_smp {
        return;
    }
    for _ in 0..iters {
        while !EVENT_START.load(core::sync::atomic::Ordering::Acquire) {}
        EVENT_START.store(false, core::sync::atomic::Ordering::Release);

        unsafe {
            log::info!("[follower] start copy.. len {}", EVENT.len);
            let src_buf = core::slice::from_raw_parts(EVENT.src_ptr as *const u8, EVENT.len);
            let dst_buf = core::slice::from_raw_parts_mut(EVENT.dst_ptr as *mut u8, EVENT.len);
            dst_buf.copy_from_slice(src_buf);
        };

        EVENT_FINISH.store(true, core::sync::atomic::Ordering::Release);
    }
}
