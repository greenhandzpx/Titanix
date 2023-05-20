#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSEL pin state"]
    pub msel: crate::Reg<msel::MSEL_SPEC>,
}
#[doc = "MSEL register accessor: an alias for `Reg<MSEL_SPEC>`"]
pub type MSEL = crate::Reg<msel::MSEL_SPEC>;
#[doc = "MSEL pin state"]
pub mod msel;
