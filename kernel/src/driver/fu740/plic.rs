use alloc::sync::Arc;
use fu740_pac::{Peripherals, PLIC};

use crate::{driver::Mutex, println};

static PLIC_INSTANCE: Mutex<Option<PLIC>> = Mutex::new(None);

pub fn init_plic() {
    *PLIC_INSTANCE.lock() = Some(unsafe { Peripherals::steal().PLIC });
}
