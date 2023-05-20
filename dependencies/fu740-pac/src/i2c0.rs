#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Prescale register lo-byte"]
    pub prescale_low: crate::Reg<prescale_low::PRESCALE_LOW_SPEC>,
    #[doc = "0x04 - Clock Prescale register hi-byte"]
    pub prescale_high: crate::Reg<prescale_high::PRESCALE_HIGH_SPEC>,
    #[doc = "0x08 - Control register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x0c - Transmit and receive data byte register"]
    pub transmit__receive: crate::Reg<transmit__receive::TRANSMIT__RECEIVE_SPEC>,
    #[doc = "0x10 - Command write and status read register"]
    pub command__status: crate::Reg<command__status::COMMAND__STATUS_SPEC>,
}
#[doc = "prescale_low register accessor: an alias for `Reg<PRESCALE_LOW_SPEC>`"]
pub type PRESCALE_LOW = crate::Reg<prescale_low::PRESCALE_LOW_SPEC>;
#[doc = "Clock Prescale register lo-byte"]
pub mod prescale_low;
#[doc = "prescale_high register accessor: an alias for `Reg<PRESCALE_HIGH_SPEC>`"]
pub type PRESCALE_HIGH = crate::Reg<prescale_high::PRESCALE_HIGH_SPEC>;
#[doc = "Clock Prescale register hi-byte"]
pub mod prescale_high;
#[doc = "control register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register"]
pub mod control;
#[doc = "transmit__receive register accessor: an alias for `Reg<TRANSMIT__RECEIVE_SPEC>`"]
pub type TRANSMIT__RECEIVE = crate::Reg<transmit__receive::TRANSMIT__RECEIVE_SPEC>;
#[doc = "Transmit and receive data byte register"]
pub mod transmit__receive;
#[doc = "command__status register accessor: an alias for `Reg<COMMAND__STATUS_SPEC>`"]
pub type COMMAND__STATUS = crate::Reg<command__status::COMMAND__STATUS_SPEC>;
#[doc = "Command write and status read register"]
pub mod command__status;
