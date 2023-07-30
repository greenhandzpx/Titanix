use alloc::sync::Arc;
use fu740_hal::{
    clock::PrciExt,
    time::{Hertz, U32Ext},
};
use fu740_pac::{Peripherals, PRCI};

use crate::{driver::Mutex, println};

const INPUT_CLK_RATE: u64 = 26_000_000;

pub trait CalcRate {
    fn rate(self) -> u64;
}

impl CalcRate for fu740_pac::prci::core_pllcfg::R {
    fn rate(self) -> u64 {
        let divr = self.pllr().bits();
        let divf = self.pllf().bits();
        let divq = self.pllq().bits();
        let bypass = self.pllbypass().bit();
        println!(
            "core_pllcfg: r={}, f={}, q={}, bypass={}",
            divr, divf, divq, bypass
        );
        if bypass {
            INPUT_CLK_RATE
        } else {
            (INPUT_CLK_RATE * 2 * ((divf as u64) + 1) / ((divr as u64) + 1)) >> divq
        }
    }
}

impl CalcRate for fu740_pac::prci::hfpclk_pllcfg::R {
    fn rate(self) -> u64 {
        let divr = self.pllr().bits();
        let divf = self.pllf().bits();
        let divq = self.pllq().bits();
        let bypass = self.pllbypass().bit();
        println!(
            "hfpclk_pllcfg: r={}, f={}, q={}, bypass={}",
            divr, divf, divq, bypass
        );
        if bypass {
            INPUT_CLK_RATE
        } else {
            (INPUT_CLK_RATE * 2 * ((divf as u64) + 1) / ((divr as u64) + 1)) >> (divq + 1)
        }
    }
}

pub fn init_prci() {
    let prci = unsafe { Peripherals::steal().PRCI };
    let coreclk_rate = prci.core_pllcfg.read().rate();
    let pclk_rate = prci.hfpclk_pllcfg.read().rate();
    println!("coreclk rate = {}, pclk rate = {}", coreclk_rate, pclk_rate);
    let new_coreclk_rate = (coreclk_rate as f64 * 1.05) as u32;
    println!("new coreclk rate = {}", new_coreclk_rate);
    let mut setup = prci.setup();
    setup = setup.pclk((pclk_rate as u32).hz());
    setup = setup.coreclk(new_coreclk_rate.hz());
    let clocks = setup.freeze();
    println!(
        "coreclk rate = {}, pclk rate = {}",
        clocks.coreclk().0,
        clocks.pclk().0
    );
}
