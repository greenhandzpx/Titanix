#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Serial clock divisor"]
    pub sckdiv: crate::Reg<sckdiv::SCKDIV_SPEC>,
    #[doc = "0x04 - Serial clock mode"]
    pub sckmode: crate::Reg<sckmode::SCKMODE_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Chip select ID"]
    pub csid: crate::Reg<csid::CSID_SPEC>,
    #[doc = "0x14 - Chip select default"]
    pub csdef: crate::Reg<csdef::CSDEF_SPEC>,
    #[doc = "0x18 - Chip select mode"]
    pub csmode: crate::Reg<csmode::CSMODE_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x28 - Delay control 0"]
    pub delay0: crate::Reg<delay0::DELAY0_SPEC>,
    #[doc = "0x2c - Delay control 1"]
    pub delay1: crate::Reg<delay1::DELAY1_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x38 - SPI extra sampling delay to increase the SPI frequency"]
    pub extradel: crate::Reg<extradel::EXTRADEL_SPEC>,
    #[doc = "0x3c - Number of delay stages from slave to the SPI controller"]
    pub sampledel: crate::Reg<sampledel::SAMPLEDEL_SPEC>,
    #[doc = "0x40 - Frame format"]
    pub fmt: crate::Reg<fmt::FMT_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x48 - Tx FIFO Data"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x4c - Rx FIFO data"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x50 - Tx FIFO watermark"]
    pub txmark: crate::Reg<txmark::TXMARK_SPEC>,
    #[doc = "0x54 - Rx FIFO watermark"]
    pub rxmark: crate::Reg<rxmark::RXMARK_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x60 - SPI flash interface control"]
    pub fctrl: crate::Reg<fctrl::FCTRL_SPEC>,
    #[doc = "0x64 - SPI flash instruction format"]
    pub ffmt: crate::Reg<ffmt::FFMT_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x70 - SPI interrupt enable"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x74 - SPI interrupt pending"]
    pub ip: crate::Reg<ip::IP_SPEC>,
}
#[doc = "sckdiv register accessor: an alias for `Reg<SCKDIV_SPEC>`"]
pub type SCKDIV = crate::Reg<sckdiv::SCKDIV_SPEC>;
#[doc = "Serial clock divisor"]
pub mod sckdiv;
#[doc = "sckmode register accessor: an alias for `Reg<SCKMODE_SPEC>`"]
pub type SCKMODE = crate::Reg<sckmode::SCKMODE_SPEC>;
#[doc = "Serial clock mode"]
pub mod sckmode;
#[doc = "csid register accessor: an alias for `Reg<CSID_SPEC>`"]
pub type CSID = crate::Reg<csid::CSID_SPEC>;
#[doc = "Chip select ID"]
pub mod csid;
#[doc = "csdef register accessor: an alias for `Reg<CSDEF_SPEC>`"]
pub type CSDEF = crate::Reg<csdef::CSDEF_SPEC>;
#[doc = "Chip select default"]
pub mod csdef;
#[doc = "csmode register accessor: an alias for `Reg<CSMODE_SPEC>`"]
pub type CSMODE = crate::Reg<csmode::CSMODE_SPEC>;
#[doc = "Chip select mode"]
pub mod csmode;
#[doc = "delay0 register accessor: an alias for `Reg<DELAY0_SPEC>`"]
pub type DELAY0 = crate::Reg<delay0::DELAY0_SPEC>;
#[doc = "Delay control 0"]
pub mod delay0;
#[doc = "delay1 register accessor: an alias for `Reg<DELAY1_SPEC>`"]
pub type DELAY1 = crate::Reg<delay1::DELAY1_SPEC>;
#[doc = "Delay control 1"]
pub mod delay1;
#[doc = "extradel register accessor: an alias for `Reg<EXTRADEL_SPEC>`"]
pub type EXTRADEL = crate::Reg<extradel::EXTRADEL_SPEC>;
#[doc = "SPI extra sampling delay to increase the SPI frequency"]
pub mod extradel;
#[doc = "sampledel register accessor: an alias for `Reg<SAMPLEDEL_SPEC>`"]
pub type SAMPLEDEL = crate::Reg<sampledel::SAMPLEDEL_SPEC>;
#[doc = "Number of delay stages from slave to the SPI controller"]
pub mod sampledel;
#[doc = "fmt register accessor: an alias for `Reg<FMT_SPEC>`"]
pub type FMT = crate::Reg<fmt::FMT_SPEC>;
#[doc = "Frame format"]
pub mod fmt;
#[doc = "txdata register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Tx FIFO Data"]
pub mod txdata;
#[doc = "rxdata register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Rx FIFO data"]
pub mod rxdata;
#[doc = "txmark register accessor: an alias for `Reg<TXMARK_SPEC>`"]
pub type TXMARK = crate::Reg<txmark::TXMARK_SPEC>;
#[doc = "Tx FIFO watermark"]
pub mod txmark;
#[doc = "rxmark register accessor: an alias for `Reg<RXMARK_SPEC>`"]
pub type RXMARK = crate::Reg<rxmark::RXMARK_SPEC>;
#[doc = "Rx FIFO watermark"]
pub mod rxmark;
#[doc = "fctrl register accessor: an alias for `Reg<FCTRL_SPEC>`"]
pub type FCTRL = crate::Reg<fctrl::FCTRL_SPEC>;
#[doc = "SPI flash interface control"]
pub mod fctrl;
#[doc = "ffmt register accessor: an alias for `Reg<FFMT_SPEC>`"]
pub type FFMT = crate::Reg<ffmt::FFMT_SPEC>;
#[doc = "SPI flash instruction format"]
pub mod ffmt;
#[doc = "ie register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "SPI interrupt enable"]
pub mod ie;
#[doc = "ip register accessor: an alias for `Reg<IP_SPEC>`"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "SPI interrupt pending"]
pub mod ip;
