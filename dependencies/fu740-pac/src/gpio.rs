#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin value"]
    pub input_val: crate::Reg<input_val::INPUT_VAL_SPEC>,
    #[doc = "0x04 - Pin input enable"]
    pub input_en: crate::Reg<input_en::INPUT_EN_SPEC>,
    #[doc = "0x08 - Pin output enable"]
    pub output_en: crate::Reg<output_en::OUTPUT_EN_SPEC>,
    #[doc = "0x0c - Output value"]
    pub output_val: crate::Reg<output_val::OUTPUT_VAL_SPEC>,
    #[doc = "0x10 - Internal pull-up enable"]
    pub pue: crate::Reg<pue::PUE_SPEC>,
    #[doc = "0x14 - Pin drive strength"]
    pub ds: crate::Reg<ds::DS_SPEC>,
    #[doc = "0x18 - Rise interrupt enable"]
    pub rise_ie: crate::Reg<rise_ie::RISE_IE_SPEC>,
    #[doc = "0x1c - Rise interrupt pending"]
    pub rise_ip: crate::Reg<rise_ip::RISE_IP_SPEC>,
    #[doc = "0x20 - Fall interrupt enable"]
    pub fall_ie: crate::Reg<fall_ie::FALL_IE_SPEC>,
    #[doc = "0x24 - Fall interrupt pending"]
    pub fall_ip: crate::Reg<fall_ip::FALL_IP_SPEC>,
    #[doc = "0x28 - High interrupt enable"]
    pub high_ie: crate::Reg<high_ie::HIGH_IE_SPEC>,
    #[doc = "0x2c - High interrupt pending"]
    pub high_ip: crate::Reg<high_ip::HIGH_IP_SPEC>,
    #[doc = "0x30 - Low interrupt enable"]
    pub low_ie: crate::Reg<low_ie::LOW_IE_SPEC>,
    #[doc = "0x34 - Low interrupt pending"]
    pub low_ip: crate::Reg<low_ip::LOW_IP_SPEC>,
    #[doc = "0x38 - I/O function enable"]
    pub iof_en: crate::Reg<iof_en::IOF_EN_SPEC>,
    #[doc = "0x3c - I/O function select"]
    pub iof_sel: crate::Reg<iof_sel::IOF_SEL_SPEC>,
    #[doc = "0x40 - Output XOR (invert)"]
    pub out_xor: crate::Reg<out_xor::OUT_XOR_SPEC>,
}
#[doc = "input_val register accessor: an alias for `Reg<INPUT_VAL_SPEC>`"]
pub type INPUT_VAL = crate::Reg<input_val::INPUT_VAL_SPEC>;
#[doc = "Pin value"]
pub mod input_val;
#[doc = "input_en register accessor: an alias for `Reg<INPUT_EN_SPEC>`"]
pub type INPUT_EN = crate::Reg<input_en::INPUT_EN_SPEC>;
#[doc = "Pin input enable"]
pub mod input_en;
#[doc = "output_en register accessor: an alias for `Reg<OUTPUT_EN_SPEC>`"]
pub type OUTPUT_EN = crate::Reg<output_en::OUTPUT_EN_SPEC>;
#[doc = "Pin output enable"]
pub mod output_en;
#[doc = "output_val register accessor: an alias for `Reg<OUTPUT_VAL_SPEC>`"]
pub type OUTPUT_VAL = crate::Reg<output_val::OUTPUT_VAL_SPEC>;
#[doc = "Output value"]
pub mod output_val;
#[doc = "pue register accessor: an alias for `Reg<PUE_SPEC>`"]
pub type PUE = crate::Reg<pue::PUE_SPEC>;
#[doc = "Internal pull-up enable"]
pub mod pue;
#[doc = "ds register accessor: an alias for `Reg<DS_SPEC>`"]
pub type DS = crate::Reg<ds::DS_SPEC>;
#[doc = "Pin drive strength"]
pub mod ds;
#[doc = "rise_ie register accessor: an alias for `Reg<RISE_IE_SPEC>`"]
pub type RISE_IE = crate::Reg<rise_ie::RISE_IE_SPEC>;
#[doc = "Rise interrupt enable"]
pub mod rise_ie;
#[doc = "rise_ip register accessor: an alias for `Reg<RISE_IP_SPEC>`"]
pub type RISE_IP = crate::Reg<rise_ip::RISE_IP_SPEC>;
#[doc = "Rise interrupt pending"]
pub mod rise_ip;
#[doc = "fall_ie register accessor: an alias for `Reg<FALL_IE_SPEC>`"]
pub type FALL_IE = crate::Reg<fall_ie::FALL_IE_SPEC>;
#[doc = "Fall interrupt enable"]
pub mod fall_ie;
#[doc = "fall_ip register accessor: an alias for `Reg<FALL_IP_SPEC>`"]
pub type FALL_IP = crate::Reg<fall_ip::FALL_IP_SPEC>;
#[doc = "Fall interrupt pending"]
pub mod fall_ip;
#[doc = "high_ie register accessor: an alias for `Reg<HIGH_IE_SPEC>`"]
pub type HIGH_IE = crate::Reg<high_ie::HIGH_IE_SPEC>;
#[doc = "High interrupt enable"]
pub mod high_ie;
#[doc = "high_ip register accessor: an alias for `Reg<HIGH_IP_SPEC>`"]
pub type HIGH_IP = crate::Reg<high_ip::HIGH_IP_SPEC>;
#[doc = "High interrupt pending"]
pub mod high_ip;
#[doc = "low_ie register accessor: an alias for `Reg<LOW_IE_SPEC>`"]
pub type LOW_IE = crate::Reg<low_ie::LOW_IE_SPEC>;
#[doc = "Low interrupt enable"]
pub mod low_ie;
#[doc = "low_ip register accessor: an alias for `Reg<LOW_IP_SPEC>`"]
pub type LOW_IP = crate::Reg<low_ip::LOW_IP_SPEC>;
#[doc = "Low interrupt pending"]
pub mod low_ip;
#[doc = "iof_en register accessor: an alias for `Reg<IOF_EN_SPEC>`"]
pub type IOF_EN = crate::Reg<iof_en::IOF_EN_SPEC>;
#[doc = "I/O function enable"]
pub mod iof_en;
#[doc = "iof_sel register accessor: an alias for `Reg<IOF_SEL_SPEC>`"]
pub type IOF_SEL = crate::Reg<iof_sel::IOF_SEL_SPEC>;
#[doc = "I/O function select"]
pub mod iof_sel;
#[doc = "out_xor register accessor: an alias for `Reg<OUT_XOR_SPEC>`"]
pub type OUT_XOR = crate::Reg<out_xor::OUT_XOR_SPEC>;
#[doc = "Output XOR (invert)"]
pub mod out_xor;
