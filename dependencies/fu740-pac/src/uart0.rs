#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit data register"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x04 - Receive data register"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x08 - Transmit control register"]
    pub txctrl: crate::Reg<txctrl::TXCTRL_SPEC>,
    #[doc = "0x0c - Receive control register"]
    pub rxctrl: crate::Reg<rxctrl::RXCTRL_SPEC>,
    #[doc = "0x10 - UART interrupt enable"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x14 - UART interrupt pending"]
    pub ip: crate::Reg<ip::IP_SPEC>,
    #[doc = "0x18 - Baud rate divisor"]
    pub div: crate::Reg<div::DIV_SPEC>,
}
#[doc = "txdata register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit data register"]
pub mod txdata;
#[doc = "rxdata register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive data register"]
pub mod rxdata;
#[doc = "txctrl register accessor: an alias for `Reg<TXCTRL_SPEC>`"]
pub type TXCTRL = crate::Reg<txctrl::TXCTRL_SPEC>;
#[doc = "Transmit control register"]
pub mod txctrl;
#[doc = "rxctrl register accessor: an alias for `Reg<RXCTRL_SPEC>`"]
pub type RXCTRL = crate::Reg<rxctrl::RXCTRL_SPEC>;
#[doc = "Receive control register"]
pub mod rxctrl;
#[doc = "ie register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "UART interrupt enable"]
pub mod ie;
#[doc = "ip register accessor: an alias for `Reg<IP_SPEC>`"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "UART interrupt pending"]
pub mod ip;
#[doc = "div register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Baud rate divisor"]
pub mod div;
