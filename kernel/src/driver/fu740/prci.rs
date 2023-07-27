use alloc::sync::Arc;
use fu740_pac::{Peripherals, PRCI};

use crate::driver::Mutex;

static PRCI_INSTANCE: Mutex<Option<PRCI>> = Mutex::new(None);

const INPUT_CLK_RATE: u64 = 26_000_000;

pub fn init_prci() {
    *PRCI_INSTANCE.lock() = Some(unsafe { Peripherals::steal().PRCI });
}

pub fn coreclk_rate() -> u64 {
    let prci = PRCI_INSTANCE.lock();
    let prci = prci.as_ref().unwrap();
    let reader = prci.core_pllcfg.read();
    let divr = reader.pllr().bits();
    let divf = reader.pllf().bits();
    let divq = reader.pllq().bits();
    let bypass = reader.pllbypass().bit();

    if bypass {
        INPUT_CLK_RATE
    } else {
        (INPUT_CLK_RATE * 2 * ((divf as u64) + 1) / ((divr as u64) + 1)) >> divq
    }
}

pub fn pclk_rate() -> u64 {
    let prci = PRCI_INSTANCE.lock();
    let prci = prci.as_ref().unwrap();
    let reader = prci.hfpclk_pllcfg.read();
    let divr = reader.pllr().bits();
    let divf = reader.pllf().bits();
    let divq = reader.pllq().bits();
    let bypass = reader.pllbypass().bit();

    if bypass {
        INPUT_CLK_RATE
    } else {
        (INPUT_CLK_RATE * 2 * ((divf as u64) + 1) / ((divr as u64) + 1)) >> divq
    }
}
