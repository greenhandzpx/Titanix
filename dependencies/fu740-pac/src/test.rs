#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Test finisher register"]
    pub finisher: crate::Reg<finisher::FINISHER_SPEC>,
}
#[doc = "finisher register accessor: an alias for `Reg<FINISHER_SPEC>`"]
pub type FINISHER = crate::Reg<finisher::FINISHER_SPEC>;
#[doc = "Test finisher register"]
pub mod finisher;
