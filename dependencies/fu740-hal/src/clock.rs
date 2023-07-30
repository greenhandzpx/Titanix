use crate::{pac::PRCI, time::Hertz};

const HFXCLK: u32 = 26_000_000;

pub trait PrciExt {
    fn setup(self) -> ClockSetup;
}

impl PrciExt for PRCI {
    fn setup(self) -> ClockSetup {
        ClockSetup {
            prci: self,
            coreclk: None,
            pclk: None,
        }
    }
}

struct PllConfig {
    r: u8,
    f: u16,
    q: u8,
    range: u8,
    bypass: bool,
}

impl PllConfig {
    fn calculate(input: u32, output: u32) -> Result<PllConfig, &'static str> {
        if input == output {
            return Ok(PllConfig {
                r: 0,
                f: 0,
                q: 0,
                range: 0,
                bypass: true,
            });
        }

        let divq: u8 = match output {
            f if f > 2_400_000_000 => {
                return Err("Requested PLL output frequency is too high");
            }
            f if f >= 1_200_000_000 => 1,
            f if f >= 600_000_000 => 2,
            f if f >= 300_000_000 => 3,
            f if f >= 150_000_000 => 4,
            f if f >= 75_000_000 => 5,
            f if f >= 37_500_000 => 6,
            _ => {
                return Err("Requested PLL output frequency is too low");
            }
        };
        let vco = (output as u64) << divq;

        let divr = (0..3)
            .min_by_key(|divr| {
                let pllin = input / (divr + 1);
                if !(7_000_000..200_000_000).contains(&pllin) {
                    i64::MAX
                } else {
                    let f1 = vco / (2 * pllin as u64);
                    let vco1 = f1 * 2 * (pllin as u64);
                    ((vco1 as i64) - (vco as i64)).abs()
                }
            })
            .ok_or("Internal error: `min_by_key()` returned `None` from non-empty iterator")?;

        let pllin = input / (divr + 1);
        let divf = (vco / (2 * pllin as u64) - 1) as u16;

        let range = match pllin {
            f if f < 7_000_000 => {
                return Err("Invalid PLL input frequency");
            }
            f if f < 11_000_000 => 1,
            f if f < 18_000_000 => 2,
            f if f < 30_000_000 => 3,
            f if f < 50_000_000 => 4,
            f if f < 80_000_000 => 5,
            f if f < 130_000_000 => 6,
            f if f < 200_000_000 => 7,
            _ => {
                return Err("Invalid PLL input frequency");
            }
        };

        Ok(PllConfig {
            r: divr as u8,
            f: divf,
            q: divq,
            range,
            bypass: false,
        })
    }

    fn output_frequency(&self, input: u32) -> u32 {
        if self.bypass {
            input
        } else {
            let vco = (input as u64) * 2 * (self.f as u64 + 1) / (self.r as u64 + 1);
            (vco >> self.q) as u32
        }
    }
}

pub struct ClockSetup {
    prci: PRCI,
    coreclk: Option<u32>,
    pclk: Option<u32>,
}

impl ClockSetup {
    pub fn coreclk<F: Into<Hertz>>(mut self, freq: F) -> Self {
        let freq = freq.into().0;
        assert!(freq < 1_600_000_000);

        self.coreclk = Some(freq);
        self
    }

    pub fn pclk<F: Into<Hertz>>(mut self, freq: F) -> Self {
        let freq = freq.into().0;
        // assert!(freq < 125_000_000);

        self.pclk = Some(freq);
        self
    }

    pub fn freeze(self) -> Clocks {
        let coreclk = self.coreclk.unwrap_or(HFXCLK);
        let pclk = self.pclk.unwrap_or(HFXCLK / 2);

        let core_pll = PllConfig::calculate(HFXCLK, coreclk).expect("Invalid PLL input parameters");
        let hfpclk_pll =
            PllConfig::calculate(HFXCLK, pclk * 2).expect("Invalid PLL input parameters");

        // Switch core clock to HFXCLK
        self.prci.core_clk_sel_reg.modify(|_, w| w.source().hfclk());

        // Apply PLL configuration
        unsafe {
            self.prci.core_pllcfg.write_with_zero(|w| {
                w.pllr().bits(core_pll.r);
                w.pllf().bits(core_pll.f);
                w.pllq().bits(core_pll.q);
                w.pllrange().bits(core_pll.range);
                w.pllbypass().bit(core_pll.bypass);
                w.pllfsebypass().set_bit()
            });
        }

        if !core_pll.bypass {
            // Wait for lock
            while self.prci.core_pllcfg.read().plllock().bit_is_clear() {}

            // Select corepll
            self.prci.corepllsel.modify(|_, w| w.source().corepll());
        }

        if coreclk != HFXCLK {
            // Select PLL as a core clock source
            self.prci
                .core_clk_sel_reg
                .modify(|_, w| w.source().pll_mux());
        }

        // Switch peripheral clock to HFXCLK
        self.prci.hfpclkpllsel.modify(|_, w| w.source().hfclk());

        // Apply PLL configuration
        unsafe {
            self.prci.hfpclk_pllcfg.write_with_zero(|w| {
                w.pllr().bits(hfpclk_pll.r);
                w.pllf().bits(hfpclk_pll.f);
                w.pllq().bits(hfpclk_pll.q);
                w.pllrange().bits(hfpclk_pll.range);
                w.pllbypass().bit(hfpclk_pll.bypass);
                w.pllfsebypass().set_bit()
            });
        }

        if !hfpclk_pll.bypass {
            // Wait for lock
            while self.prci.hfpclk_pllcfg.read().plllock().bit_is_clear() {}
        }

        // Enable clock
        self.prci
            .hfpclk_plloutdiv
            .modify(|r, w| unsafe { w.bits(r.bits() | 1u32 << 31) });

        if pclk != HFXCLK / 2 {
            // Select PLL as a peripheral clock source
            self.prci.hfpclkpllsel.modify(|_, w| w.source().hfpclkpll());
        }

        // Set divider to 0 (divide by 2)
        unsafe {
            self.prci.hfpclk_div_reg.write_with_zero(|w| w.bits(0));
        }

        Clocks {
            coreclk: core_pll.output_frequency(HFXCLK),
            pclk: hfpclk_pll.output_frequency(HFXCLK) / 2,
        }
    }
}

pub struct Clocks {
    coreclk: u32,
    pclk: u32,
}

impl Clocks {
    pub fn coreclk(&self) -> Hertz {
        Hertz(self.coreclk)
    }

    pub fn pclk(&self) -> Hertz {
        Hertz(self.pclk)
    }
}
