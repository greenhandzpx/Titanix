#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crystal Oscillator Configuration and Status register"]
    pub hfxosccfg: crate::Reg<hfxosccfg::HFXOSCCFG_SPEC>,
    #[doc = "0x04 - PLL Configuration and Status register"]
    pub core_pllcfg: crate::Reg<core_pllcfg::CORE_PLLCFG_SPEC>,
    #[doc = "0x08 - PLL Final Divide Configuration register"]
    pub core_plloutdiv: crate::Reg<core_plloutdiv::CORE_PLLOUTDIV_SPEC>,
    #[doc = "0x0c - PLL Configuration and Status register"]
    pub ddr_pllcfg: crate::Reg<ddr_pllcfg::DDR_PLLCFG_SPEC>,
    #[doc = "0x10 - PLL Final Divide Configuration register"]
    pub ddr_plloutdiv: crate::Reg<ddr_plloutdiv::DDR_PLLOUTDIV_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - PLL Configuration and Status register"]
    pub gemgxl_pllcfg: crate::Reg<gemgxl_pllcfg::GEMGXL_PLLCFG_SPEC>,
    #[doc = "0x20 - PLL Final Divide Configuration register"]
    pub gemgxl_plloutdiv: crate::Reg<gemgxl_plloutdiv::GEMGXL_PLLOUTDIV_SPEC>,
    #[doc = "0x24 - Core clock source register"]
    pub core_clk_sel_reg: crate::Reg<core_clk_sel_reg::CORE_CLK_SEL_REG_SPEC>,
    #[doc = "0x28 - Software controlled resets"]
    pub devices_reset_n: crate::Reg<devices_reset_n::DEVICES_RESET_N_SPEC>,
    #[doc = "0x2c - Current selection of each clock mux"]
    pub clk_mux_status: crate::Reg<clk_mux_status::CLK_MUX_STATUS_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x38 - PLL Configuration and Status register"]
    pub dvfs_core_pllcfg: crate::Reg<dvfs_core_pllcfg::DVFS_CORE_PLLCFG_SPEC>,
    #[doc = "0x3c - PLL Final Divide Configuration register"]
    pub dvfs_core_plloutdiv: crate::Reg<dvfs_core_plloutdiv::DVFS_CORE_PLLOUTDIV_SPEC>,
    #[doc = "0x40 - Clock select register"]
    pub corepllsel: crate::Reg<corepllsel::COREPLLSEL_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x50 - PLL Configuration and Status register"]
    pub hfpclk_pllcfg: crate::Reg<hfpclk_pllcfg::HFPCLK_PLLCFG_SPEC>,
    #[doc = "0x54 - PLL Final Divide Configuration register"]
    pub hfpclk_plloutdiv: crate::Reg<hfpclk_plloutdiv::HFPCLK_PLLOUTDIV_SPEC>,
    #[doc = "0x58 - Periphery clock source register"]
    pub hfpclkpllsel: crate::Reg<hfpclkpllsel::HFPCLKPLLSEL_SPEC>,
    #[doc = "0x5c - HFPCLK PLL divider register"]
    pub hfpclk_div_reg: crate::Reg<hfpclk_div_reg::HFPCLK_DIV_REG_SPEC>,
    _reserved17: [u8; 0x80],
    #[doc = "0xe0 - PLL presence register"]
    pub prci_plls: crate::Reg<prci_plls::PRCI_PLLS_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0xf0 - "]
    pub procmoncfg: crate::Reg<procmoncfg::PROCMONCFG_SPEC>,
}
#[doc = "hfxosccfg register accessor: an alias for `Reg<HFXOSCCFG_SPEC>`"]
pub type HFXOSCCFG = crate::Reg<hfxosccfg::HFXOSCCFG_SPEC>;
#[doc = "Crystal Oscillator Configuration and Status register"]
pub mod hfxosccfg;
#[doc = "core_pllcfg register accessor: an alias for `Reg<CORE_PLLCFG_SPEC>`"]
pub type CORE_PLLCFG = crate::Reg<core_pllcfg::CORE_PLLCFG_SPEC>;
#[doc = "PLL Configuration and Status register"]
pub mod core_pllcfg;
#[doc = "core_plloutdiv register accessor: an alias for `Reg<CORE_PLLOUTDIV_SPEC>`"]
pub type CORE_PLLOUTDIV = crate::Reg<core_plloutdiv::CORE_PLLOUTDIV_SPEC>;
#[doc = "PLL Final Divide Configuration register"]
pub mod core_plloutdiv;
#[doc = "ddr_pllcfg register accessor: an alias for `Reg<DDR_PLLCFG_SPEC>`"]
pub type DDR_PLLCFG = crate::Reg<ddr_pllcfg::DDR_PLLCFG_SPEC>;
#[doc = "PLL Configuration and Status register"]
pub mod ddr_pllcfg;
#[doc = "ddr_plloutdiv register accessor: an alias for `Reg<DDR_PLLOUTDIV_SPEC>`"]
pub type DDR_PLLOUTDIV = crate::Reg<ddr_plloutdiv::DDR_PLLOUTDIV_SPEC>;
#[doc = "PLL Final Divide Configuration register"]
pub mod ddr_plloutdiv;
#[doc = "gemgxl_pllcfg register accessor: an alias for `Reg<GEMGXL_PLLCFG_SPEC>`"]
pub type GEMGXL_PLLCFG = crate::Reg<gemgxl_pllcfg::GEMGXL_PLLCFG_SPEC>;
#[doc = "PLL Configuration and Status register"]
pub mod gemgxl_pllcfg;
#[doc = "gemgxl_plloutdiv register accessor: an alias for `Reg<GEMGXL_PLLOUTDIV_SPEC>`"]
pub type GEMGXL_PLLOUTDIV = crate::Reg<gemgxl_plloutdiv::GEMGXL_PLLOUTDIV_SPEC>;
#[doc = "PLL Final Divide Configuration register"]
pub mod gemgxl_plloutdiv;
#[doc = "core_clk_sel_reg register accessor: an alias for `Reg<CORE_CLK_SEL_REG_SPEC>`"]
pub type CORE_CLK_SEL_REG = crate::Reg<core_clk_sel_reg::CORE_CLK_SEL_REG_SPEC>;
#[doc = "Core clock source register"]
pub mod core_clk_sel_reg;
#[doc = "devices_reset_n register accessor: an alias for `Reg<DEVICES_RESET_N_SPEC>`"]
pub type DEVICES_RESET_N = crate::Reg<devices_reset_n::DEVICES_RESET_N_SPEC>;
#[doc = "Software controlled resets"]
pub mod devices_reset_n;
#[doc = "clk_mux_status register accessor: an alias for `Reg<CLK_MUX_STATUS_SPEC>`"]
pub type CLK_MUX_STATUS = crate::Reg<clk_mux_status::CLK_MUX_STATUS_SPEC>;
#[doc = "Current selection of each clock mux"]
pub mod clk_mux_status;
#[doc = "dvfs_core_pllcfg register accessor: an alias for `Reg<DVFS_CORE_PLLCFG_SPEC>`"]
pub type DVFS_CORE_PLLCFG = crate::Reg<dvfs_core_pllcfg::DVFS_CORE_PLLCFG_SPEC>;
#[doc = "PLL Configuration and Status register"]
pub mod dvfs_core_pllcfg;
#[doc = "dvfs_core_plloutdiv register accessor: an alias for `Reg<DVFS_CORE_PLLOUTDIV_SPEC>`"]
pub type DVFS_CORE_PLLOUTDIV = crate::Reg<dvfs_core_plloutdiv::DVFS_CORE_PLLOUTDIV_SPEC>;
#[doc = "PLL Final Divide Configuration register"]
pub mod dvfs_core_plloutdiv;
#[doc = "corepllsel register accessor: an alias for `Reg<COREPLLSEL_SPEC>`"]
pub type COREPLLSEL = crate::Reg<corepllsel::COREPLLSEL_SPEC>;
#[doc = "Clock select register"]
pub mod corepllsel;
#[doc = "hfpclk_pllcfg register accessor: an alias for `Reg<HFPCLK_PLLCFG_SPEC>`"]
pub type HFPCLK_PLLCFG = crate::Reg<hfpclk_pllcfg::HFPCLK_PLLCFG_SPEC>;
#[doc = "PLL Configuration and Status register"]
pub mod hfpclk_pllcfg;
#[doc = "hfpclk_plloutdiv register accessor: an alias for `Reg<HFPCLK_PLLOUTDIV_SPEC>`"]
pub type HFPCLK_PLLOUTDIV = crate::Reg<hfpclk_plloutdiv::HFPCLK_PLLOUTDIV_SPEC>;
#[doc = "PLL Final Divide Configuration register"]
pub mod hfpclk_plloutdiv;
#[doc = "hfpclkpllsel register accessor: an alias for `Reg<HFPCLKPLLSEL_SPEC>`"]
pub type HFPCLKPLLSEL = crate::Reg<hfpclkpllsel::HFPCLKPLLSEL_SPEC>;
#[doc = "Periphery clock source register"]
pub mod hfpclkpllsel;
#[doc = "hfpclk_div_reg register accessor: an alias for `Reg<HFPCLK_DIV_REG_SPEC>`"]
pub type HFPCLK_DIV_REG = crate::Reg<hfpclk_div_reg::HFPCLK_DIV_REG_SPEC>;
#[doc = "HFPCLK PLL divider register"]
pub mod hfpclk_div_reg;
#[doc = "prci_plls register accessor: an alias for `Reg<PRCI_PLLS_SPEC>`"]
pub type PRCI_PLLS = crate::Reg<prci_plls::PRCI_PLLS_SPEC>;
#[doc = "PLL presence register"]
pub mod prci_plls;
#[doc = "procmoncfg register accessor: an alias for `Reg<PROCMONCFG_SPEC>`"]
pub type PROCMONCFG = crate::Reg<procmoncfg::PROCMONCFG_SPEC>;
#[doc = ""]
pub mod procmoncfg;
