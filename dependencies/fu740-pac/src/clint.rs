#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSIP Register for hart 0"]
    pub msip_0: crate::Reg<msip_0::MSIP_0_SPEC>,
    #[doc = "0x04 - MSIP Register for hart 1"]
    pub msip_1: crate::Reg<msip_1::MSIP_1_SPEC>,
    #[doc = "0x08 - MSIP Register for hart 2"]
    pub msip_2: crate::Reg<msip_2::MSIP_2_SPEC>,
    #[doc = "0x0c - MSIP Register for hart 3"]
    pub msip_3: crate::Reg<msip_3::MSIP_3_SPEC>,
    #[doc = "0x10 - MSIP Register for hart 4"]
    pub msip_4: crate::Reg<msip_4::MSIP_4_SPEC>,
    _reserved5: [u8; 0x3fec],
    #[doc = "0x4000..0x4008 - MTIMECMP Register for hart 0"]
    pub mtimecmp_0: crate::Reg<mtimecmp_0::MTIMECMP_0_SPEC>,
    #[doc = "0x4008..0x4010 - MTIMECMP Register for hart 1"]
    pub mtimecmp_1: crate::Reg<mtimecmp_1::MTIMECMP_1_SPEC>,
    #[doc = "0x4010..0x4018 - MTIMECMP Register for hart 2"]
    pub mtimecmp_2: crate::Reg<mtimecmp_2::MTIMECMP_2_SPEC>,
    #[doc = "0x4018..0x4020 - MTIMECMP Register for hart 3"]
    pub mtimecmp_3: crate::Reg<mtimecmp_3::MTIMECMP_3_SPEC>,
    #[doc = "0x4020..0x4028 - MTIMECMP Register for hart 4"]
    pub mtimecmp_4: crate::Reg<mtimecmp_4::MTIMECMP_4_SPEC>,
    _reserved10: [u8; 0x7fd0],
    #[doc = "0xbff8..0xc000 - MTIME Register"]
    pub mtime: crate::Reg<mtime::MTIME_SPEC>,
}
#[doc = "msip_0 register accessor: an alias for `Reg<MSIP_0_SPEC>`"]
pub type MSIP_0 = crate::Reg<msip_0::MSIP_0_SPEC>;
#[doc = "MSIP Register for hart 0"]
pub mod msip_0;
#[doc = "msip_1 register accessor: an alias for `Reg<MSIP_1_SPEC>`"]
pub type MSIP_1 = crate::Reg<msip_1::MSIP_1_SPEC>;
#[doc = "MSIP Register for hart 1"]
pub mod msip_1;
#[doc = "msip_2 register accessor: an alias for `Reg<MSIP_2_SPEC>`"]
pub type MSIP_2 = crate::Reg<msip_2::MSIP_2_SPEC>;
#[doc = "MSIP Register for hart 2"]
pub mod msip_2;
#[doc = "msip_3 register accessor: an alias for `Reg<MSIP_3_SPEC>`"]
pub type MSIP_3 = crate::Reg<msip_3::MSIP_3_SPEC>;
#[doc = "MSIP Register for hart 3"]
pub mod msip_3;
#[doc = "msip_4 register accessor: an alias for `Reg<MSIP_4_SPEC>`"]
pub type MSIP_4 = crate::Reg<msip_4::MSIP_4_SPEC>;
#[doc = "MSIP Register for hart 4"]
pub mod msip_4;
#[doc = "mtimecmp_0 register accessor: an alias for `Reg<MTIMECMP_0_SPEC>`"]
pub type MTIMECMP_0 = crate::Reg<mtimecmp_0::MTIMECMP_0_SPEC>;
#[doc = "MTIMECMP Register for hart 0"]
pub mod mtimecmp_0;
#[doc = "mtimecmp_1 register accessor: an alias for `Reg<MTIMECMP_1_SPEC>`"]
pub type MTIMECMP_1 = crate::Reg<mtimecmp_1::MTIMECMP_1_SPEC>;
#[doc = "MTIMECMP Register for hart 1"]
pub mod mtimecmp_1;
#[doc = "mtimecmp_2 register accessor: an alias for `Reg<MTIMECMP_2_SPEC>`"]
pub type MTIMECMP_2 = crate::Reg<mtimecmp_2::MTIMECMP_2_SPEC>;
#[doc = "MTIMECMP Register for hart 2"]
pub mod mtimecmp_2;
#[doc = "mtimecmp_3 register accessor: an alias for `Reg<MTIMECMP_3_SPEC>`"]
pub type MTIMECMP_3 = crate::Reg<mtimecmp_3::MTIMECMP_3_SPEC>;
#[doc = "MTIMECMP Register for hart 3"]
pub mod mtimecmp_3;
#[doc = "mtimecmp_4 register accessor: an alias for `Reg<MTIMECMP_4_SPEC>`"]
pub type MTIMECMP_4 = crate::Reg<mtimecmp_4::MTIMECMP_4_SPEC>;
#[doc = "MTIMECMP Register for hart 4"]
pub mod mtimecmp_4;
#[doc = "mtime register accessor: an alias for `Reg<MTIME_SPEC>`"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "MTIME Register"]
pub mod mtime;
