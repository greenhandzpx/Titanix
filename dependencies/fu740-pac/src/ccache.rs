#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Information about the Cache Configuration"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - The index of the largest way which has been enabled. May only be increased."]
    pub wayenable: crate::Reg<wayenable::WAYENABLE_SPEC>,
    _reserved2: [u8; 0x1ff4],
    #[doc = "0x2000 - The L2 performance event0 control register."]
    pub l2perfevent0: crate::Reg<l2perfevent0::L2PERFEVENT0_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x2008 - The L2 performance event1 control register."]
    pub l2perfevent1: crate::Reg<l2perfevent1::L2PERFEVENT1_SPEC>,
    _reserved4: [u8; 0x07f4],
    #[doc = "0x2800 - The L2 Client Filterregister."]
    pub l2clientfilter: crate::Reg<l2clientfilter::L2CLIENTFILTER_SPEC>,
    _reserved5: [u8; 0x07fc],
    #[doc = "0x3000 - The L2 performance monitor counter0 register."]
    pub l2pmcounter0: crate::Reg<l2pmcounter0::L2PMCOUNTER0_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x3008 - The L2 performance monitor counter1 register."]
    pub l2pmcounter1: crate::Reg<l2pmcounter1::L2PMCOUNTER1_SPEC>,
    _reserved7: [u8; 0x01ec],
    #[doc = "0x31f8 - The L2 performance monitor counter63 register."]
    pub l2pmcounter63: crate::Reg<l2pmcounter63::L2PMCOUNTER63_SPEC>,
}
#[doc = "config register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Information about the Cache Configuration"]
pub mod config;
#[doc = "wayenable register accessor: an alias for `Reg<WAYENABLE_SPEC>`"]
pub type WAYENABLE = crate::Reg<wayenable::WAYENABLE_SPEC>;
#[doc = "The index of the largest way which has been enabled. May only be increased."]
pub mod wayenable;
#[doc = "l2perfevent0 register accessor: an alias for `Reg<L2PERFEVENT0_SPEC>`"]
pub type L2PERFEVENT0 = crate::Reg<l2perfevent0::L2PERFEVENT0_SPEC>;
#[doc = "The L2 performance event0 control register."]
pub mod l2perfevent0;
#[doc = "l2perfevent1 register accessor: an alias for `Reg<L2PERFEVENT1_SPEC>`"]
pub type L2PERFEVENT1 = crate::Reg<l2perfevent1::L2PERFEVENT1_SPEC>;
#[doc = "The L2 performance event1 control register."]
pub mod l2perfevent1;
#[doc = "l2clientfilter register accessor: an alias for `Reg<L2CLIENTFILTER_SPEC>`"]
pub type L2CLIENTFILTER = crate::Reg<l2clientfilter::L2CLIENTFILTER_SPEC>;
#[doc = "The L2 Client Filterregister."]
pub mod l2clientfilter;
#[doc = "l2pmcounter0 register accessor: an alias for `Reg<L2PMCOUNTER0_SPEC>`"]
pub type L2PMCOUNTER0 = crate::Reg<l2pmcounter0::L2PMCOUNTER0_SPEC>;
#[doc = "The L2 performance monitor counter0 register."]
pub mod l2pmcounter0;
#[doc = "l2pmcounter1 register accessor: an alias for `Reg<L2PMCOUNTER1_SPEC>`"]
pub type L2PMCOUNTER1 = crate::Reg<l2pmcounter1::L2PMCOUNTER1_SPEC>;
#[doc = "The L2 performance monitor counter1 register."]
pub mod l2pmcounter1;
#[doc = "l2pmcounter63 register accessor: an alias for `Reg<L2PMCOUNTER63_SPEC>`"]
pub type L2PMCOUNTER63 = crate::Reg<l2pmcounter63::L2PMCOUNTER63_SPEC>;
#[doc = "The L2 performance monitor counter63 register."]
pub mod l2pmcounter63;
