#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - PRIORITY Register for interrupt id 1"]
    pub priority_1: crate::Reg<priority_1::PRIORITY_1_SPEC>,
    #[doc = "0x08 - PRIORITY Register for interrupt id 2"]
    pub priority_2: crate::Reg<priority_2::PRIORITY_2_SPEC>,
    #[doc = "0x0c - PRIORITY Register for interrupt id 3"]
    pub priority_3: crate::Reg<priority_3::PRIORITY_3_SPEC>,
    #[doc = "0x10 - PRIORITY Register for interrupt id 4"]
    pub priority_4: crate::Reg<priority_4::PRIORITY_4_SPEC>,
    #[doc = "0x14 - PRIORITY Register for interrupt id 5"]
    pub priority_5: crate::Reg<priority_5::PRIORITY_5_SPEC>,
    #[doc = "0x18 - PRIORITY Register for interrupt id 6"]
    pub priority_6: crate::Reg<priority_6::PRIORITY_6_SPEC>,
    #[doc = "0x1c - PRIORITY Register for interrupt id 7"]
    pub priority_7: crate::Reg<priority_7::PRIORITY_7_SPEC>,
    #[doc = "0x20 - PRIORITY Register for interrupt id 8"]
    pub priority_8: crate::Reg<priority_8::PRIORITY_8_SPEC>,
    #[doc = "0x24 - PRIORITY Register for interrupt id 9"]
    pub priority_9: crate::Reg<priority_9::PRIORITY_9_SPEC>,
    #[doc = "0x28 - PRIORITY Register for interrupt id 10"]
    pub priority_10: crate::Reg<priority_10::PRIORITY_10_SPEC>,
    #[doc = "0x2c - PRIORITY Register for interrupt id 11"]
    pub priority_11: crate::Reg<priority_11::PRIORITY_11_SPEC>,
    #[doc = "0x30 - PRIORITY Register for interrupt id 12"]
    pub priority_12: crate::Reg<priority_12::PRIORITY_12_SPEC>,
    #[doc = "0x34 - PRIORITY Register for interrupt id 13"]
    pub priority_13: crate::Reg<priority_13::PRIORITY_13_SPEC>,
    #[doc = "0x38 - PRIORITY Register for interrupt id 14"]
    pub priority_14: crate::Reg<priority_14::PRIORITY_14_SPEC>,
    #[doc = "0x3c - PRIORITY Register for interrupt id 15"]
    pub priority_15: crate::Reg<priority_15::PRIORITY_15_SPEC>,
    #[doc = "0x40 - PRIORITY Register for interrupt id 16"]
    pub priority_16: crate::Reg<priority_16::PRIORITY_16_SPEC>,
    #[doc = "0x44 - PRIORITY Register for interrupt id 17"]
    pub priority_17: crate::Reg<priority_17::PRIORITY_17_SPEC>,
    #[doc = "0x48 - PRIORITY Register for interrupt id 18"]
    pub priority_18: crate::Reg<priority_18::PRIORITY_18_SPEC>,
    #[doc = "0x4c - PRIORITY Register for interrupt id 19"]
    pub priority_19: crate::Reg<priority_19::PRIORITY_19_SPEC>,
    #[doc = "0x50 - PRIORITY Register for interrupt id 20"]
    pub priority_20: crate::Reg<priority_20::PRIORITY_20_SPEC>,
    #[doc = "0x54 - PRIORITY Register for interrupt id 21"]
    pub priority_21: crate::Reg<priority_21::PRIORITY_21_SPEC>,
    #[doc = "0x58 - PRIORITY Register for interrupt id 22"]
    pub priority_22: crate::Reg<priority_22::PRIORITY_22_SPEC>,
    #[doc = "0x5c - PRIORITY Register for interrupt id 23"]
    pub priority_23: crate::Reg<priority_23::PRIORITY_23_SPEC>,
    #[doc = "0x60 - PRIORITY Register for interrupt id 24"]
    pub priority_24: crate::Reg<priority_24::PRIORITY_24_SPEC>,
    #[doc = "0x64 - PRIORITY Register for interrupt id 25"]
    pub priority_25: crate::Reg<priority_25::PRIORITY_25_SPEC>,
    #[doc = "0x68 - PRIORITY Register for interrupt id 26"]
    pub priority_26: crate::Reg<priority_26::PRIORITY_26_SPEC>,
    #[doc = "0x6c - PRIORITY Register for interrupt id 27"]
    pub priority_27: crate::Reg<priority_27::PRIORITY_27_SPEC>,
    #[doc = "0x70 - PRIORITY Register for interrupt id 28"]
    pub priority_28: crate::Reg<priority_28::PRIORITY_28_SPEC>,
    #[doc = "0x74 - PRIORITY Register for interrupt id 29"]
    pub priority_29: crate::Reg<priority_29::PRIORITY_29_SPEC>,
    #[doc = "0x78 - PRIORITY Register for interrupt id 30"]
    pub priority_30: crate::Reg<priority_30::PRIORITY_30_SPEC>,
    #[doc = "0x7c - PRIORITY Register for interrupt id 31"]
    pub priority_31: crate::Reg<priority_31::PRIORITY_31_SPEC>,
    #[doc = "0x80 - PRIORITY Register for interrupt id 32"]
    pub priority_32: crate::Reg<priority_32::PRIORITY_32_SPEC>,
    #[doc = "0x84 - PRIORITY Register for interrupt id 33"]
    pub priority_33: crate::Reg<priority_33::PRIORITY_33_SPEC>,
    #[doc = "0x88 - PRIORITY Register for interrupt id 34"]
    pub priority_34: crate::Reg<priority_34::PRIORITY_34_SPEC>,
    #[doc = "0x8c - PRIORITY Register for interrupt id 35"]
    pub priority_35: crate::Reg<priority_35::PRIORITY_35_SPEC>,
    #[doc = "0x90 - PRIORITY Register for interrupt id 36"]
    pub priority_36: crate::Reg<priority_36::PRIORITY_36_SPEC>,
    #[doc = "0x94 - PRIORITY Register for interrupt id 37"]
    pub priority_37: crate::Reg<priority_37::PRIORITY_37_SPEC>,
    #[doc = "0x98 - PRIORITY Register for interrupt id 38"]
    pub priority_38: crate::Reg<priority_38::PRIORITY_38_SPEC>,
    #[doc = "0x9c - PRIORITY Register for interrupt id 39"]
    pub priority_39: crate::Reg<priority_39::PRIORITY_39_SPEC>,
    #[doc = "0xa0 - PRIORITY Register for interrupt id 40"]
    pub priority_40: crate::Reg<priority_40::PRIORITY_40_SPEC>,
    #[doc = "0xa4 - PRIORITY Register for interrupt id 41"]
    pub priority_41: crate::Reg<priority_41::PRIORITY_41_SPEC>,
    #[doc = "0xa8 - PRIORITY Register for interrupt id 42"]
    pub priority_42: crate::Reg<priority_42::PRIORITY_42_SPEC>,
    #[doc = "0xac - PRIORITY Register for interrupt id 43"]
    pub priority_43: crate::Reg<priority_43::PRIORITY_43_SPEC>,
    #[doc = "0xb0 - PRIORITY Register for interrupt id 44"]
    pub priority_44: crate::Reg<priority_44::PRIORITY_44_SPEC>,
    #[doc = "0xb4 - PRIORITY Register for interrupt id 45"]
    pub priority_45: crate::Reg<priority_45::PRIORITY_45_SPEC>,
    #[doc = "0xb8 - PRIORITY Register for interrupt id 46"]
    pub priority_46: crate::Reg<priority_46::PRIORITY_46_SPEC>,
    #[doc = "0xbc - PRIORITY Register for interrupt id 47"]
    pub priority_47: crate::Reg<priority_47::PRIORITY_47_SPEC>,
    #[doc = "0xc0 - PRIORITY Register for interrupt id 48"]
    pub priority_48: crate::Reg<priority_48::PRIORITY_48_SPEC>,
    #[doc = "0xc4 - PRIORITY Register for interrupt id 49"]
    pub priority_49: crate::Reg<priority_49::PRIORITY_49_SPEC>,
    #[doc = "0xc8 - PRIORITY Register for interrupt id 50"]
    pub priority_50: crate::Reg<priority_50::PRIORITY_50_SPEC>,
    #[doc = "0xcc - PRIORITY Register for interrupt id 51"]
    pub priority_51: crate::Reg<priority_51::PRIORITY_51_SPEC>,
    #[doc = "0xd0 - PRIORITY Register for interrupt id 52"]
    pub priority_52: crate::Reg<priority_52::PRIORITY_52_SPEC>,
    #[doc = "0xd4 - PRIORITY Register for interrupt id 53"]
    pub priority_53: crate::Reg<priority_53::PRIORITY_53_SPEC>,
    #[doc = "0xd8 - PRIORITY Register for interrupt id 54"]
    pub priority_54: crate::Reg<priority_54::PRIORITY_54_SPEC>,
    #[doc = "0xdc - PRIORITY Register for interrupt id 55"]
    pub priority_55: crate::Reg<priority_55::PRIORITY_55_SPEC>,
    #[doc = "0xe0 - PRIORITY Register for interrupt id 56"]
    pub priority_56: crate::Reg<priority_56::PRIORITY_56_SPEC>,
    #[doc = "0xe4 - PRIORITY Register for interrupt id 57"]
    pub priority_57: crate::Reg<priority_57::PRIORITY_57_SPEC>,
    #[doc = "0xe8 - PRIORITY Register for interrupt id 58"]
    pub priority_58: crate::Reg<priority_58::PRIORITY_58_SPEC>,
    #[doc = "0xec - PRIORITY Register for interrupt id 59"]
    pub priority_59: crate::Reg<priority_59::PRIORITY_59_SPEC>,
    #[doc = "0xf0 - PRIORITY Register for interrupt id 60"]
    pub priority_60: crate::Reg<priority_60::PRIORITY_60_SPEC>,
    #[doc = "0xf4 - PRIORITY Register for interrupt id 61"]
    pub priority_61: crate::Reg<priority_61::PRIORITY_61_SPEC>,
    #[doc = "0xf8 - PRIORITY Register for interrupt id 62"]
    pub priority_62: crate::Reg<priority_62::PRIORITY_62_SPEC>,
    #[doc = "0xfc - PRIORITY Register for interrupt id 63"]
    pub priority_63: crate::Reg<priority_63::PRIORITY_63_SPEC>,
    #[doc = "0x100 - PRIORITY Register for interrupt id 64"]
    pub priority_64: crate::Reg<priority_64::PRIORITY_64_SPEC>,
    #[doc = "0x104 - PRIORITY Register for interrupt id 65"]
    pub priority_65: crate::Reg<priority_65::PRIORITY_65_SPEC>,
    #[doc = "0x108 - PRIORITY Register for interrupt id 66"]
    pub priority_66: crate::Reg<priority_66::PRIORITY_66_SPEC>,
    #[doc = "0x10c - PRIORITY Register for interrupt id 67"]
    pub priority_67: crate::Reg<priority_67::PRIORITY_67_SPEC>,
    #[doc = "0x110 - PRIORITY Register for interrupt id 68"]
    pub priority_68: crate::Reg<priority_68::PRIORITY_68_SPEC>,
    #[doc = "0x114 - PRIORITY Register for interrupt id 69"]
    pub priority_69: crate::Reg<priority_69::PRIORITY_69_SPEC>,
    _reserved69: [u8; 0x0ee8],
    #[doc = "0x1000 - PENDING Register for interrupt ids 31 to 0"]
    pub pending_0: crate::Reg<pending_0::PENDING_0_SPEC>,
    #[doc = "0x1004 - PENDING Register for interrupt ids 63 to 32"]
    pub pending_1: crate::Reg<pending_1::PENDING_1_SPEC>,
    #[doc = "0x1008 - PENDING Register for interrupt ids 69 to 64"]
    pub pending_2: crate::Reg<pending_2::PENDING_2_SPEC>,
    _reserved72: [u8; 0x0ff4],
    #[doc = "0x2000 - ENABLE Register for interrupt ids 31 to 0 for hart 0"]
    pub enable_0_0: crate::Reg<enable_0_0::ENABLE_0_0_SPEC>,
    #[doc = "0x2004 - ENABLE Register for interrupt ids 63 to 32 for hart 0"]
    pub enable_1_0: crate::Reg<enable_1_0::ENABLE_1_0_SPEC>,
    #[doc = "0x2008 - ENABLE Register for interrupt ids 69 to 64 for hart 0"]
    pub enable_2_0: crate::Reg<enable_2_0::ENABLE_2_0_SPEC>,
    _reserved75: [u8; 0x74],
    #[doc = "0x2080 - ENABLE Register for interrupt ids 31 to 0 for hart 1"]
    pub enable_0_1: crate::Reg<enable_0_1::ENABLE_0_1_SPEC>,
    #[doc = "0x2084 - ENABLE Register for interrupt ids 63 to 32 for hart 1"]
    pub enable_1_1: crate::Reg<enable_1_1::ENABLE_1_1_SPEC>,
    #[doc = "0x2088 - ENABLE Register for interrupt ids 69 to 64 for hart 1"]
    pub enable_2_1: crate::Reg<enable_2_1::ENABLE_2_1_SPEC>,
    _reserved78: [u8; 0x74],
    #[doc = "0x2100 - ENABLE Register for interrupt ids 31 to 0 for hart 2"]
    pub enable_0_2: crate::Reg<enable_0_2::ENABLE_0_2_SPEC>,
    #[doc = "0x2104 - ENABLE Register for interrupt ids 63 to 32 for hart 2"]
    pub enable_1_2: crate::Reg<enable_1_2::ENABLE_1_2_SPEC>,
    #[doc = "0x2108 - ENABLE Register for interrupt ids 69 to 64 for hart 2"]
    pub enable_2_2: crate::Reg<enable_2_2::ENABLE_2_2_SPEC>,
    _reserved81: [u8; 0x74],
    #[doc = "0x2180 - ENABLE Register for interrupt ids 31 to 0 for hart 3"]
    pub enable_0_3: crate::Reg<enable_0_3::ENABLE_0_3_SPEC>,
    #[doc = "0x2184 - ENABLE Register for interrupt ids 63 to 32 for hart 3"]
    pub enable_1_3: crate::Reg<enable_1_3::ENABLE_1_3_SPEC>,
    #[doc = "0x2188 - ENABLE Register for interrupt ids 69 to 64 for hart 3"]
    pub enable_2_3: crate::Reg<enable_2_3::ENABLE_2_3_SPEC>,
    _reserved84: [u8; 0x74],
    #[doc = "0x2200 - ENABLE Register for interrupt ids 31 to 0 for hart 4"]
    pub enable_0_4: crate::Reg<enable_0_4::ENABLE_0_4_SPEC>,
    #[doc = "0x2204 - ENABLE Register for interrupt ids 63 to 32 for hart 4"]
    pub enable_1_4: crate::Reg<enable_1_4::ENABLE_1_4_SPEC>,
    #[doc = "0x2208 - ENABLE Register for interrupt ids 69 to 64 for hart 4"]
    pub enable_2_4: crate::Reg<enable_2_4::ENABLE_2_4_SPEC>,
    _reserved87: [u8; 0x001f_ddf4],
    #[doc = "0x200000 - PRIORITY THRESHOLD Register for hart 0"]
    pub threshold_0: crate::Reg<threshold_0::THRESHOLD_0_SPEC>,
    #[doc = "0x200004 - CLAIM and COMPLETE Register for hart 0"]
    pub claimplete_0: crate::Reg<claimplete_0::CLAIMPLETE_0_SPEC>,
    _reserved89: [u8; 0x0ff8],
    #[doc = "0x201000 - PRIORITY THRESHOLD Register for hart 1"]
    pub threshold_1: crate::Reg<threshold_1::THRESHOLD_1_SPEC>,
    #[doc = "0x201004 - CLAIM and COMPLETE Register for hart 1"]
    pub claimplete_1: crate::Reg<claimplete_1::CLAIMPLETE_1_SPEC>,
    _reserved91: [u8; 0x0ff8],
    #[doc = "0x202000 - PRIORITY THRESHOLD Register for hart 2"]
    pub threshold_2: crate::Reg<threshold_2::THRESHOLD_2_SPEC>,
    #[doc = "0x202004 - CLAIM and COMPLETE Register for hart 2"]
    pub claimplete_2: crate::Reg<claimplete_2::CLAIMPLETE_2_SPEC>,
    _reserved93: [u8; 0x0ff8],
    #[doc = "0x203000 - PRIORITY THRESHOLD Register for hart 3"]
    pub threshold_3: crate::Reg<threshold_3::THRESHOLD_3_SPEC>,
    #[doc = "0x203004 - CLAIM and COMPLETE Register for hart 3"]
    pub claimplete_3: crate::Reg<claimplete_3::CLAIMPLETE_3_SPEC>,
    _reserved95: [u8; 0x0ff8],
    #[doc = "0x204000 - PRIORITY THRESHOLD Register for hart 4"]
    pub threshold_4: crate::Reg<threshold_4::THRESHOLD_4_SPEC>,
    #[doc = "0x204004 - CLAIM and COMPLETE Register for hart 4"]
    pub claimplete_4: crate::Reg<claimplete_4::CLAIMPLETE_4_SPEC>,
}
#[doc = "priority_1 register accessor: an alias for `Reg<PRIORITY_1_SPEC>`"]
pub type PRIORITY_1 = crate::Reg<priority_1::PRIORITY_1_SPEC>;
#[doc = "PRIORITY Register for interrupt id 1"]
pub mod priority_1;
#[doc = "priority_2 register accessor: an alias for `Reg<PRIORITY_2_SPEC>`"]
pub type PRIORITY_2 = crate::Reg<priority_2::PRIORITY_2_SPEC>;
#[doc = "PRIORITY Register for interrupt id 2"]
pub mod priority_2;
#[doc = "priority_3 register accessor: an alias for `Reg<PRIORITY_3_SPEC>`"]
pub type PRIORITY_3 = crate::Reg<priority_3::PRIORITY_3_SPEC>;
#[doc = "PRIORITY Register for interrupt id 3"]
pub mod priority_3;
#[doc = "priority_4 register accessor: an alias for `Reg<PRIORITY_4_SPEC>`"]
pub type PRIORITY_4 = crate::Reg<priority_4::PRIORITY_4_SPEC>;
#[doc = "PRIORITY Register for interrupt id 4"]
pub mod priority_4;
#[doc = "priority_5 register accessor: an alias for `Reg<PRIORITY_5_SPEC>`"]
pub type PRIORITY_5 = crate::Reg<priority_5::PRIORITY_5_SPEC>;
#[doc = "PRIORITY Register for interrupt id 5"]
pub mod priority_5;
#[doc = "priority_6 register accessor: an alias for `Reg<PRIORITY_6_SPEC>`"]
pub type PRIORITY_6 = crate::Reg<priority_6::PRIORITY_6_SPEC>;
#[doc = "PRIORITY Register for interrupt id 6"]
pub mod priority_6;
#[doc = "priority_7 register accessor: an alias for `Reg<PRIORITY_7_SPEC>`"]
pub type PRIORITY_7 = crate::Reg<priority_7::PRIORITY_7_SPEC>;
#[doc = "PRIORITY Register for interrupt id 7"]
pub mod priority_7;
#[doc = "priority_8 register accessor: an alias for `Reg<PRIORITY_8_SPEC>`"]
pub type PRIORITY_8 = crate::Reg<priority_8::PRIORITY_8_SPEC>;
#[doc = "PRIORITY Register for interrupt id 8"]
pub mod priority_8;
#[doc = "priority_9 register accessor: an alias for `Reg<PRIORITY_9_SPEC>`"]
pub type PRIORITY_9 = crate::Reg<priority_9::PRIORITY_9_SPEC>;
#[doc = "PRIORITY Register for interrupt id 9"]
pub mod priority_9;
#[doc = "priority_10 register accessor: an alias for `Reg<PRIORITY_10_SPEC>`"]
pub type PRIORITY_10 = crate::Reg<priority_10::PRIORITY_10_SPEC>;
#[doc = "PRIORITY Register for interrupt id 10"]
pub mod priority_10;
#[doc = "priority_11 register accessor: an alias for `Reg<PRIORITY_11_SPEC>`"]
pub type PRIORITY_11 = crate::Reg<priority_11::PRIORITY_11_SPEC>;
#[doc = "PRIORITY Register for interrupt id 11"]
pub mod priority_11;
#[doc = "priority_12 register accessor: an alias for `Reg<PRIORITY_12_SPEC>`"]
pub type PRIORITY_12 = crate::Reg<priority_12::PRIORITY_12_SPEC>;
#[doc = "PRIORITY Register for interrupt id 12"]
pub mod priority_12;
#[doc = "priority_13 register accessor: an alias for `Reg<PRIORITY_13_SPEC>`"]
pub type PRIORITY_13 = crate::Reg<priority_13::PRIORITY_13_SPEC>;
#[doc = "PRIORITY Register for interrupt id 13"]
pub mod priority_13;
#[doc = "priority_14 register accessor: an alias for `Reg<PRIORITY_14_SPEC>`"]
pub type PRIORITY_14 = crate::Reg<priority_14::PRIORITY_14_SPEC>;
#[doc = "PRIORITY Register for interrupt id 14"]
pub mod priority_14;
#[doc = "priority_15 register accessor: an alias for `Reg<PRIORITY_15_SPEC>`"]
pub type PRIORITY_15 = crate::Reg<priority_15::PRIORITY_15_SPEC>;
#[doc = "PRIORITY Register for interrupt id 15"]
pub mod priority_15;
#[doc = "priority_16 register accessor: an alias for `Reg<PRIORITY_16_SPEC>`"]
pub type PRIORITY_16 = crate::Reg<priority_16::PRIORITY_16_SPEC>;
#[doc = "PRIORITY Register for interrupt id 16"]
pub mod priority_16;
#[doc = "priority_17 register accessor: an alias for `Reg<PRIORITY_17_SPEC>`"]
pub type PRIORITY_17 = crate::Reg<priority_17::PRIORITY_17_SPEC>;
#[doc = "PRIORITY Register for interrupt id 17"]
pub mod priority_17;
#[doc = "priority_18 register accessor: an alias for `Reg<PRIORITY_18_SPEC>`"]
pub type PRIORITY_18 = crate::Reg<priority_18::PRIORITY_18_SPEC>;
#[doc = "PRIORITY Register for interrupt id 18"]
pub mod priority_18;
#[doc = "priority_19 register accessor: an alias for `Reg<PRIORITY_19_SPEC>`"]
pub type PRIORITY_19 = crate::Reg<priority_19::PRIORITY_19_SPEC>;
#[doc = "PRIORITY Register for interrupt id 19"]
pub mod priority_19;
#[doc = "priority_20 register accessor: an alias for `Reg<PRIORITY_20_SPEC>`"]
pub type PRIORITY_20 = crate::Reg<priority_20::PRIORITY_20_SPEC>;
#[doc = "PRIORITY Register for interrupt id 20"]
pub mod priority_20;
#[doc = "priority_21 register accessor: an alias for `Reg<PRIORITY_21_SPEC>`"]
pub type PRIORITY_21 = crate::Reg<priority_21::PRIORITY_21_SPEC>;
#[doc = "PRIORITY Register for interrupt id 21"]
pub mod priority_21;
#[doc = "priority_22 register accessor: an alias for `Reg<PRIORITY_22_SPEC>`"]
pub type PRIORITY_22 = crate::Reg<priority_22::PRIORITY_22_SPEC>;
#[doc = "PRIORITY Register for interrupt id 22"]
pub mod priority_22;
#[doc = "priority_23 register accessor: an alias for `Reg<PRIORITY_23_SPEC>`"]
pub type PRIORITY_23 = crate::Reg<priority_23::PRIORITY_23_SPEC>;
#[doc = "PRIORITY Register for interrupt id 23"]
pub mod priority_23;
#[doc = "priority_24 register accessor: an alias for `Reg<PRIORITY_24_SPEC>`"]
pub type PRIORITY_24 = crate::Reg<priority_24::PRIORITY_24_SPEC>;
#[doc = "PRIORITY Register for interrupt id 24"]
pub mod priority_24;
#[doc = "priority_25 register accessor: an alias for `Reg<PRIORITY_25_SPEC>`"]
pub type PRIORITY_25 = crate::Reg<priority_25::PRIORITY_25_SPEC>;
#[doc = "PRIORITY Register for interrupt id 25"]
pub mod priority_25;
#[doc = "priority_26 register accessor: an alias for `Reg<PRIORITY_26_SPEC>`"]
pub type PRIORITY_26 = crate::Reg<priority_26::PRIORITY_26_SPEC>;
#[doc = "PRIORITY Register for interrupt id 26"]
pub mod priority_26;
#[doc = "priority_27 register accessor: an alias for `Reg<PRIORITY_27_SPEC>`"]
pub type PRIORITY_27 = crate::Reg<priority_27::PRIORITY_27_SPEC>;
#[doc = "PRIORITY Register for interrupt id 27"]
pub mod priority_27;
#[doc = "priority_28 register accessor: an alias for `Reg<PRIORITY_28_SPEC>`"]
pub type PRIORITY_28 = crate::Reg<priority_28::PRIORITY_28_SPEC>;
#[doc = "PRIORITY Register for interrupt id 28"]
pub mod priority_28;
#[doc = "priority_29 register accessor: an alias for `Reg<PRIORITY_29_SPEC>`"]
pub type PRIORITY_29 = crate::Reg<priority_29::PRIORITY_29_SPEC>;
#[doc = "PRIORITY Register for interrupt id 29"]
pub mod priority_29;
#[doc = "priority_30 register accessor: an alias for `Reg<PRIORITY_30_SPEC>`"]
pub type PRIORITY_30 = crate::Reg<priority_30::PRIORITY_30_SPEC>;
#[doc = "PRIORITY Register for interrupt id 30"]
pub mod priority_30;
#[doc = "priority_31 register accessor: an alias for `Reg<PRIORITY_31_SPEC>`"]
pub type PRIORITY_31 = crate::Reg<priority_31::PRIORITY_31_SPEC>;
#[doc = "PRIORITY Register for interrupt id 31"]
pub mod priority_31;
#[doc = "priority_32 register accessor: an alias for `Reg<PRIORITY_32_SPEC>`"]
pub type PRIORITY_32 = crate::Reg<priority_32::PRIORITY_32_SPEC>;
#[doc = "PRIORITY Register for interrupt id 32"]
pub mod priority_32;
#[doc = "priority_33 register accessor: an alias for `Reg<PRIORITY_33_SPEC>`"]
pub type PRIORITY_33 = crate::Reg<priority_33::PRIORITY_33_SPEC>;
#[doc = "PRIORITY Register for interrupt id 33"]
pub mod priority_33;
#[doc = "priority_34 register accessor: an alias for `Reg<PRIORITY_34_SPEC>`"]
pub type PRIORITY_34 = crate::Reg<priority_34::PRIORITY_34_SPEC>;
#[doc = "PRIORITY Register for interrupt id 34"]
pub mod priority_34;
#[doc = "priority_35 register accessor: an alias for `Reg<PRIORITY_35_SPEC>`"]
pub type PRIORITY_35 = crate::Reg<priority_35::PRIORITY_35_SPEC>;
#[doc = "PRIORITY Register for interrupt id 35"]
pub mod priority_35;
#[doc = "priority_36 register accessor: an alias for `Reg<PRIORITY_36_SPEC>`"]
pub type PRIORITY_36 = crate::Reg<priority_36::PRIORITY_36_SPEC>;
#[doc = "PRIORITY Register for interrupt id 36"]
pub mod priority_36;
#[doc = "priority_37 register accessor: an alias for `Reg<PRIORITY_37_SPEC>`"]
pub type PRIORITY_37 = crate::Reg<priority_37::PRIORITY_37_SPEC>;
#[doc = "PRIORITY Register for interrupt id 37"]
pub mod priority_37;
#[doc = "priority_38 register accessor: an alias for `Reg<PRIORITY_38_SPEC>`"]
pub type PRIORITY_38 = crate::Reg<priority_38::PRIORITY_38_SPEC>;
#[doc = "PRIORITY Register for interrupt id 38"]
pub mod priority_38;
#[doc = "priority_39 register accessor: an alias for `Reg<PRIORITY_39_SPEC>`"]
pub type PRIORITY_39 = crate::Reg<priority_39::PRIORITY_39_SPEC>;
#[doc = "PRIORITY Register for interrupt id 39"]
pub mod priority_39;
#[doc = "priority_40 register accessor: an alias for `Reg<PRIORITY_40_SPEC>`"]
pub type PRIORITY_40 = crate::Reg<priority_40::PRIORITY_40_SPEC>;
#[doc = "PRIORITY Register for interrupt id 40"]
pub mod priority_40;
#[doc = "priority_41 register accessor: an alias for `Reg<PRIORITY_41_SPEC>`"]
pub type PRIORITY_41 = crate::Reg<priority_41::PRIORITY_41_SPEC>;
#[doc = "PRIORITY Register for interrupt id 41"]
pub mod priority_41;
#[doc = "priority_42 register accessor: an alias for `Reg<PRIORITY_42_SPEC>`"]
pub type PRIORITY_42 = crate::Reg<priority_42::PRIORITY_42_SPEC>;
#[doc = "PRIORITY Register for interrupt id 42"]
pub mod priority_42;
#[doc = "priority_43 register accessor: an alias for `Reg<PRIORITY_43_SPEC>`"]
pub type PRIORITY_43 = crate::Reg<priority_43::PRIORITY_43_SPEC>;
#[doc = "PRIORITY Register for interrupt id 43"]
pub mod priority_43;
#[doc = "priority_44 register accessor: an alias for `Reg<PRIORITY_44_SPEC>`"]
pub type PRIORITY_44 = crate::Reg<priority_44::PRIORITY_44_SPEC>;
#[doc = "PRIORITY Register for interrupt id 44"]
pub mod priority_44;
#[doc = "priority_45 register accessor: an alias for `Reg<PRIORITY_45_SPEC>`"]
pub type PRIORITY_45 = crate::Reg<priority_45::PRIORITY_45_SPEC>;
#[doc = "PRIORITY Register for interrupt id 45"]
pub mod priority_45;
#[doc = "priority_46 register accessor: an alias for `Reg<PRIORITY_46_SPEC>`"]
pub type PRIORITY_46 = crate::Reg<priority_46::PRIORITY_46_SPEC>;
#[doc = "PRIORITY Register for interrupt id 46"]
pub mod priority_46;
#[doc = "priority_47 register accessor: an alias for `Reg<PRIORITY_47_SPEC>`"]
pub type PRIORITY_47 = crate::Reg<priority_47::PRIORITY_47_SPEC>;
#[doc = "PRIORITY Register for interrupt id 47"]
pub mod priority_47;
#[doc = "priority_48 register accessor: an alias for `Reg<PRIORITY_48_SPEC>`"]
pub type PRIORITY_48 = crate::Reg<priority_48::PRIORITY_48_SPEC>;
#[doc = "PRIORITY Register for interrupt id 48"]
pub mod priority_48;
#[doc = "priority_49 register accessor: an alias for `Reg<PRIORITY_49_SPEC>`"]
pub type PRIORITY_49 = crate::Reg<priority_49::PRIORITY_49_SPEC>;
#[doc = "PRIORITY Register for interrupt id 49"]
pub mod priority_49;
#[doc = "priority_50 register accessor: an alias for `Reg<PRIORITY_50_SPEC>`"]
pub type PRIORITY_50 = crate::Reg<priority_50::PRIORITY_50_SPEC>;
#[doc = "PRIORITY Register for interrupt id 50"]
pub mod priority_50;
#[doc = "priority_51 register accessor: an alias for `Reg<PRIORITY_51_SPEC>`"]
pub type PRIORITY_51 = crate::Reg<priority_51::PRIORITY_51_SPEC>;
#[doc = "PRIORITY Register for interrupt id 51"]
pub mod priority_51;
#[doc = "priority_52 register accessor: an alias for `Reg<PRIORITY_52_SPEC>`"]
pub type PRIORITY_52 = crate::Reg<priority_52::PRIORITY_52_SPEC>;
#[doc = "PRIORITY Register for interrupt id 52"]
pub mod priority_52;
#[doc = "priority_53 register accessor: an alias for `Reg<PRIORITY_53_SPEC>`"]
pub type PRIORITY_53 = crate::Reg<priority_53::PRIORITY_53_SPEC>;
#[doc = "PRIORITY Register for interrupt id 53"]
pub mod priority_53;
#[doc = "priority_54 register accessor: an alias for `Reg<PRIORITY_54_SPEC>`"]
pub type PRIORITY_54 = crate::Reg<priority_54::PRIORITY_54_SPEC>;
#[doc = "PRIORITY Register for interrupt id 54"]
pub mod priority_54;
#[doc = "priority_55 register accessor: an alias for `Reg<PRIORITY_55_SPEC>`"]
pub type PRIORITY_55 = crate::Reg<priority_55::PRIORITY_55_SPEC>;
#[doc = "PRIORITY Register for interrupt id 55"]
pub mod priority_55;
#[doc = "priority_56 register accessor: an alias for `Reg<PRIORITY_56_SPEC>`"]
pub type PRIORITY_56 = crate::Reg<priority_56::PRIORITY_56_SPEC>;
#[doc = "PRIORITY Register for interrupt id 56"]
pub mod priority_56;
#[doc = "priority_57 register accessor: an alias for `Reg<PRIORITY_57_SPEC>`"]
pub type PRIORITY_57 = crate::Reg<priority_57::PRIORITY_57_SPEC>;
#[doc = "PRIORITY Register for interrupt id 57"]
pub mod priority_57;
#[doc = "priority_58 register accessor: an alias for `Reg<PRIORITY_58_SPEC>`"]
pub type PRIORITY_58 = crate::Reg<priority_58::PRIORITY_58_SPEC>;
#[doc = "PRIORITY Register for interrupt id 58"]
pub mod priority_58;
#[doc = "priority_59 register accessor: an alias for `Reg<PRIORITY_59_SPEC>`"]
pub type PRIORITY_59 = crate::Reg<priority_59::PRIORITY_59_SPEC>;
#[doc = "PRIORITY Register for interrupt id 59"]
pub mod priority_59;
#[doc = "priority_60 register accessor: an alias for `Reg<PRIORITY_60_SPEC>`"]
pub type PRIORITY_60 = crate::Reg<priority_60::PRIORITY_60_SPEC>;
#[doc = "PRIORITY Register for interrupt id 60"]
pub mod priority_60;
#[doc = "priority_61 register accessor: an alias for `Reg<PRIORITY_61_SPEC>`"]
pub type PRIORITY_61 = crate::Reg<priority_61::PRIORITY_61_SPEC>;
#[doc = "PRIORITY Register for interrupt id 61"]
pub mod priority_61;
#[doc = "priority_62 register accessor: an alias for `Reg<PRIORITY_62_SPEC>`"]
pub type PRIORITY_62 = crate::Reg<priority_62::PRIORITY_62_SPEC>;
#[doc = "PRIORITY Register for interrupt id 62"]
pub mod priority_62;
#[doc = "priority_63 register accessor: an alias for `Reg<PRIORITY_63_SPEC>`"]
pub type PRIORITY_63 = crate::Reg<priority_63::PRIORITY_63_SPEC>;
#[doc = "PRIORITY Register for interrupt id 63"]
pub mod priority_63;
#[doc = "priority_64 register accessor: an alias for `Reg<PRIORITY_64_SPEC>`"]
pub type PRIORITY_64 = crate::Reg<priority_64::PRIORITY_64_SPEC>;
#[doc = "PRIORITY Register for interrupt id 64"]
pub mod priority_64;
#[doc = "priority_65 register accessor: an alias for `Reg<PRIORITY_65_SPEC>`"]
pub type PRIORITY_65 = crate::Reg<priority_65::PRIORITY_65_SPEC>;
#[doc = "PRIORITY Register for interrupt id 65"]
pub mod priority_65;
#[doc = "priority_66 register accessor: an alias for `Reg<PRIORITY_66_SPEC>`"]
pub type PRIORITY_66 = crate::Reg<priority_66::PRIORITY_66_SPEC>;
#[doc = "PRIORITY Register for interrupt id 66"]
pub mod priority_66;
#[doc = "priority_67 register accessor: an alias for `Reg<PRIORITY_67_SPEC>`"]
pub type PRIORITY_67 = crate::Reg<priority_67::PRIORITY_67_SPEC>;
#[doc = "PRIORITY Register for interrupt id 67"]
pub mod priority_67;
#[doc = "priority_68 register accessor: an alias for `Reg<PRIORITY_68_SPEC>`"]
pub type PRIORITY_68 = crate::Reg<priority_68::PRIORITY_68_SPEC>;
#[doc = "PRIORITY Register for interrupt id 68"]
pub mod priority_68;
#[doc = "priority_69 register accessor: an alias for `Reg<PRIORITY_69_SPEC>`"]
pub type PRIORITY_69 = crate::Reg<priority_69::PRIORITY_69_SPEC>;
#[doc = "PRIORITY Register for interrupt id 69"]
pub mod priority_69;
#[doc = "pending_0 register accessor: an alias for `Reg<PENDING_0_SPEC>`"]
pub type PENDING_0 = crate::Reg<pending_0::PENDING_0_SPEC>;
#[doc = "PENDING Register for interrupt ids 31 to 0"]
pub mod pending_0;
#[doc = "pending_1 register accessor: an alias for `Reg<PENDING_1_SPEC>`"]
pub type PENDING_1 = crate::Reg<pending_1::PENDING_1_SPEC>;
#[doc = "PENDING Register for interrupt ids 63 to 32"]
pub mod pending_1;
#[doc = "pending_2 register accessor: an alias for `Reg<PENDING_2_SPEC>`"]
pub type PENDING_2 = crate::Reg<pending_2::PENDING_2_SPEC>;
#[doc = "PENDING Register for interrupt ids 69 to 64"]
pub mod pending_2;
#[doc = "enable_0_0 register accessor: an alias for `Reg<ENABLE_0_0_SPEC>`"]
pub type ENABLE_0_0 = crate::Reg<enable_0_0::ENABLE_0_0_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 0"]
pub mod enable_0_0;
#[doc = "enable_1_0 register accessor: an alias for `Reg<ENABLE_1_0_SPEC>`"]
pub type ENABLE_1_0 = crate::Reg<enable_1_0::ENABLE_1_0_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 0"]
pub mod enable_1_0;
#[doc = "enable_2_0 register accessor: an alias for `Reg<ENABLE_2_0_SPEC>`"]
pub type ENABLE_2_0 = crate::Reg<enable_2_0::ENABLE_2_0_SPEC>;
#[doc = "ENABLE Register for interrupt ids 69 to 64 for hart 0"]
pub mod enable_2_0;
#[doc = "enable_0_1 register accessor: an alias for `Reg<ENABLE_0_1_SPEC>`"]
pub type ENABLE_0_1 = crate::Reg<enable_0_1::ENABLE_0_1_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 1"]
pub mod enable_0_1;
#[doc = "enable_1_1 register accessor: an alias for `Reg<ENABLE_1_1_SPEC>`"]
pub type ENABLE_1_1 = crate::Reg<enable_1_1::ENABLE_1_1_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 1"]
pub mod enable_1_1;
#[doc = "enable_2_1 register accessor: an alias for `Reg<ENABLE_2_1_SPEC>`"]
pub type ENABLE_2_1 = crate::Reg<enable_2_1::ENABLE_2_1_SPEC>;
#[doc = "ENABLE Register for interrupt ids 69 to 64 for hart 1"]
pub mod enable_2_1;
#[doc = "enable_0_2 register accessor: an alias for `Reg<ENABLE_0_2_SPEC>`"]
pub type ENABLE_0_2 = crate::Reg<enable_0_2::ENABLE_0_2_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 2"]
pub mod enable_0_2;
#[doc = "enable_1_2 register accessor: an alias for `Reg<ENABLE_1_2_SPEC>`"]
pub type ENABLE_1_2 = crate::Reg<enable_1_2::ENABLE_1_2_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 2"]
pub mod enable_1_2;
#[doc = "enable_2_2 register accessor: an alias for `Reg<ENABLE_2_2_SPEC>`"]
pub type ENABLE_2_2 = crate::Reg<enable_2_2::ENABLE_2_2_SPEC>;
#[doc = "ENABLE Register for interrupt ids 69 to 64 for hart 2"]
pub mod enable_2_2;
#[doc = "enable_0_3 register accessor: an alias for `Reg<ENABLE_0_3_SPEC>`"]
pub type ENABLE_0_3 = crate::Reg<enable_0_3::ENABLE_0_3_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 3"]
pub mod enable_0_3;
#[doc = "enable_1_3 register accessor: an alias for `Reg<ENABLE_1_3_SPEC>`"]
pub type ENABLE_1_3 = crate::Reg<enable_1_3::ENABLE_1_3_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 3"]
pub mod enable_1_3;
#[doc = "enable_2_3 register accessor: an alias for `Reg<ENABLE_2_3_SPEC>`"]
pub type ENABLE_2_3 = crate::Reg<enable_2_3::ENABLE_2_3_SPEC>;
#[doc = "ENABLE Register for interrupt ids 69 to 64 for hart 3"]
pub mod enable_2_3;
#[doc = "enable_0_4 register accessor: an alias for `Reg<ENABLE_0_4_SPEC>`"]
pub type ENABLE_0_4 = crate::Reg<enable_0_4::ENABLE_0_4_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 4"]
pub mod enable_0_4;
#[doc = "enable_1_4 register accessor: an alias for `Reg<ENABLE_1_4_SPEC>`"]
pub type ENABLE_1_4 = crate::Reg<enable_1_4::ENABLE_1_4_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 4"]
pub mod enable_1_4;
#[doc = "enable_2_4 register accessor: an alias for `Reg<ENABLE_2_4_SPEC>`"]
pub type ENABLE_2_4 = crate::Reg<enable_2_4::ENABLE_2_4_SPEC>;
#[doc = "ENABLE Register for interrupt ids 69 to 64 for hart 4"]
pub mod enable_2_4;
#[doc = "threshold_0 register accessor: an alias for `Reg<THRESHOLD_0_SPEC>`"]
pub type THRESHOLD_0 = crate::Reg<threshold_0::THRESHOLD_0_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 0"]
pub mod threshold_0;
#[doc = "claimplete_0 register accessor: an alias for `Reg<CLAIMPLETE_0_SPEC>`"]
pub type CLAIMPLETE_0 = crate::Reg<claimplete_0::CLAIMPLETE_0_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 0"]
pub mod claimplete_0;
#[doc = "threshold_1 register accessor: an alias for `Reg<THRESHOLD_1_SPEC>`"]
pub type THRESHOLD_1 = crate::Reg<threshold_1::THRESHOLD_1_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 1"]
pub mod threshold_1;
#[doc = "claimplete_1 register accessor: an alias for `Reg<CLAIMPLETE_1_SPEC>`"]
pub type CLAIMPLETE_1 = crate::Reg<claimplete_1::CLAIMPLETE_1_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 1"]
pub mod claimplete_1;
#[doc = "threshold_2 register accessor: an alias for `Reg<THRESHOLD_2_SPEC>`"]
pub type THRESHOLD_2 = crate::Reg<threshold_2::THRESHOLD_2_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 2"]
pub mod threshold_2;
#[doc = "claimplete_2 register accessor: an alias for `Reg<CLAIMPLETE_2_SPEC>`"]
pub type CLAIMPLETE_2 = crate::Reg<claimplete_2::CLAIMPLETE_2_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 2"]
pub mod claimplete_2;
#[doc = "threshold_3 register accessor: an alias for `Reg<THRESHOLD_3_SPEC>`"]
pub type THRESHOLD_3 = crate::Reg<threshold_3::THRESHOLD_3_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 3"]
pub mod threshold_3;
#[doc = "claimplete_3 register accessor: an alias for `Reg<CLAIMPLETE_3_SPEC>`"]
pub type CLAIMPLETE_3 = crate::Reg<claimplete_3::CLAIMPLETE_3_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 3"]
pub mod claimplete_3;
#[doc = "threshold_4 register accessor: an alias for `Reg<THRESHOLD_4_SPEC>`"]
pub type THRESHOLD_4 = crate::Reg<threshold_4::THRESHOLD_4_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 4"]
pub mod threshold_4;
#[doc = "claimplete_4 register accessor: an alias for `Reg<CLAIMPLETE_4_SPEC>`"]
pub type CLAIMPLETE_4 = crate::Reg<claimplete_4::CLAIMPLETE_4_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 4"]
pub mod claimplete_4;
