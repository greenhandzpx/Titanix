#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM configuration register"]
    pub pwmcfg: crate::Reg<pwmcfg::PWMCFG_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - PWM count register"]
    pub pwmcount: crate::Reg<pwmcount::PWMCOUNT_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Scaled PWM count register"]
    pub pwms: crate::Reg<pwms::PWMS_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x20 - PWM 0 compare register"]
    pub pwmcmp0: crate::Reg<pwmcmp0::PWMCMP0_SPEC>,
    #[doc = "0x24 - PWM 1 compare register"]
    pub pwmcmp1: crate::Reg<pwmcmp1::PWMCMP1_SPEC>,
    #[doc = "0x28 - PWM 2 compare register"]
    pub pwmcmp2: crate::Reg<pwmcmp2::PWMCMP2_SPEC>,
    #[doc = "0x2c - PWM 3 compare register"]
    pub pwmcmp3: crate::Reg<pwmcmp3::PWMCMP3_SPEC>,
}
#[doc = "pwmcfg register accessor: an alias for `Reg<PWMCFG_SPEC>`"]
pub type PWMCFG = crate::Reg<pwmcfg::PWMCFG_SPEC>;
#[doc = "PWM configuration register"]
pub mod pwmcfg;
#[doc = "pwmcount register accessor: an alias for `Reg<PWMCOUNT_SPEC>`"]
pub type PWMCOUNT = crate::Reg<pwmcount::PWMCOUNT_SPEC>;
#[doc = "PWM count register"]
pub mod pwmcount;
#[doc = "pwms register accessor: an alias for `Reg<PWMS_SPEC>`"]
pub type PWMS = crate::Reg<pwms::PWMS_SPEC>;
#[doc = "Scaled PWM count register"]
pub mod pwms;
#[doc = "pwmcmp0 register accessor: an alias for `Reg<PWMCMP0_SPEC>`"]
pub type PWMCMP0 = crate::Reg<pwmcmp0::PWMCMP0_SPEC>;
#[doc = "PWM 0 compare register"]
pub mod pwmcmp0;
#[doc = "pwmcmp1 register accessor: an alias for `Reg<PWMCMP1_SPEC>`"]
pub type PWMCMP1 = crate::Reg<pwmcmp1::PWMCMP1_SPEC>;
#[doc = "PWM 1 compare register"]
pub mod pwmcmp1;
#[doc = "pwmcmp2 register accessor: an alias for `Reg<PWMCMP2_SPEC>`"]
pub type PWMCMP2 = crate::Reg<pwmcmp2::PWMCMP2_SPEC>;
#[doc = "PWM 2 compare register"]
pub mod pwmcmp2;
#[doc = "pwmcmp3 register accessor: an alias for `Reg<PWMCMP3_SPEC>`"]
pub type PWMCMP3 = crate::Reg<pwmcmp3::PWMCMP3_SPEC>;
#[doc = "PWM 3 compare register"]
pub mod pwmcmp3;
