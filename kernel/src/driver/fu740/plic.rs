use core::{ops::BitAnd, panic};

use alloc::sync::Arc;
use fu740_pac::{Peripherals, PLIC};

use crate::{driver::Mutex, println, processor::hart::local_hart};

static PLIC_INSTANCE: Mutex<Option<PLIC>> = Mutex::new(None);

pub fn init_plic() {
    *PLIC_INSTANCE.lock() = Some(unsafe { Peripherals::steal().PLIC });
}

pub enum InterruptSource {
    MSI(u8),
    DMA(u8),
    L2CacheDirError,
    L2CacheDirFail,
    L2CacheDataError,
    L2CacheDataFail,
    GPIO(u8),
    UART0,
    UART1,
    SPI0,
    SPI1,
    SPI2,
    PWM0(u8),
    PWM1(u8),
    I2C0,
    I2C1,
    DDR,
    MAC,
    PCIE(u8),
    BusErrorUnit0,
    BusErrorUnit1,
    BusErrorUnit2,
    BusErrorUnit3,
    BusErrorUnit4,
}

impl From<u8> for InterruptSource {
    fn from(value: u8) -> Self {
        match value {
            1..=10 => Self::MSI(value - 1),
            11..=18 => Self::DMA(value - 11),
            19 => Self::L2CacheDirError,
            20 => Self::L2CacheDirFail,
            21 => Self::L2CacheDataError,
            22 => Self::L2CacheDataFail,
            23..=38 => Self::GPIO(value - 23),
            39 => Self::UART0,
            40 => Self::UART1,
            41 => Self::SPI0,
            42 => Self::SPI1,
            43 => Self::SPI2,
            44..=47 => Self::PWM0(value - 44),
            48..=51 => Self::PWM1(value - 48),
            52 => Self::I2C0,
            53 => Self::I2C1,
            54 => Self::DDR,
            55 => Self::MAC,
            56..=64 => Self::PCIE(value - 56),
            65 => Self::BusErrorUnit0,
            66 => Self::BusErrorUnit1,
            67 => Self::BusErrorUnit2,
            68 => Self::BusErrorUnit3,
            69 => Self::BusErrorUnit4,
            _ => panic!(),
        }
    }
}

impl Into<u8> for InterruptSource {
    fn into(self) -> u8 {
        match self {
            Self::MSI(v) => v + 1,
            Self::DMA(v) => v + 11,
            Self::L2CacheDirError => 19,
            Self::L2CacheDirFail => 20,
            Self::L2CacheDataError => 21,
            Self::L2CacheDataFail => 22,
            Self::GPIO(v) => v + 23,
            Self::UART0 => 39,
            Self::UART1 => 40,
            Self::SPI0 => 41,
            Self::SPI1 => 42,
            Self::SPI2 => 43,
            Self::PWM0(v) => v + 44,
            Self::PWM1(v) => v + 48,
            Self::I2C0 => 52,
            Self::I2C1 => 53,
            Self::DDR => 54,
            Self::MAC => 55,
            Self::PCIE(v) => v + 56,
            Self::BusErrorUnit0 => 65,
            Self::BusErrorUnit1 => 66,
            Self::BusErrorUnit2 => 67,
            Self::BusErrorUnit3 => 68,
            Self::BusErrorUnit4 => 69,
        }
    }
}

struct IntrInfo(u8, u8);

impl From<u8> for IntrInfo {
    fn from(value: u8) -> Self {
        Self(value / 32, value % 32)
    }
}

pub fn plic_intr_priority(intr: InterruptSource) -> u32 {
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    let intr_id = Into::<u8>::into(intr);
    match intr_id {
        1 => plic.priority_1.read().bits(),
        2 => plic.priority_2.read().bits(),
        3 => plic.priority_3.read().bits(),
        4 => plic.priority_4.read().bits(),
        5 => plic.priority_5.read().bits(),
        6 => plic.priority_6.read().bits(),
        7 => plic.priority_7.read().bits(),
        8 => plic.priority_8.read().bits(),
        9 => plic.priority_9.read().bits(),
        10 => plic.priority_10.read().bits(),
        11 => plic.priority_11.read().bits(),
        12 => plic.priority_12.read().bits(),
        13 => plic.priority_13.read().bits(),
        14 => plic.priority_14.read().bits(),
        15 => plic.priority_15.read().bits(),
        16 => plic.priority_16.read().bits(),
        17 => plic.priority_17.read().bits(),
        18 => plic.priority_18.read().bits(),
        19 => plic.priority_19.read().bits(),
        20 => plic.priority_20.read().bits(),
        21 => plic.priority_21.read().bits(),
        22 => plic.priority_22.read().bits(),
        23 => plic.priority_23.read().bits(),
        24 => plic.priority_24.read().bits(),
        25 => plic.priority_25.read().bits(),
        26 => plic.priority_26.read().bits(),
        27 => plic.priority_27.read().bits(),
        28 => plic.priority_28.read().bits(),
        29 => plic.priority_29.read().bits(),
        30 => plic.priority_30.read().bits(),
        31 => plic.priority_31.read().bits(),
        32 => plic.priority_32.read().bits(),
        33 => plic.priority_33.read().bits(),
        34 => plic.priority_34.read().bits(),
        35 => plic.priority_35.read().bits(),
        36 => plic.priority_36.read().bits(),
        37 => plic.priority_37.read().bits(),
        38 => plic.priority_38.read().bits(),
        39 => plic.priority_39.read().bits(),
        40 => plic.priority_40.read().bits(),
        41 => plic.priority_41.read().bits(),
        42 => plic.priority_42.read().bits(),
        43 => plic.priority_43.read().bits(),
        44 => plic.priority_44.read().bits(),
        45 => plic.priority_45.read().bits(),
        46 => plic.priority_46.read().bits(),
        47 => plic.priority_47.read().bits(),
        48 => plic.priority_48.read().bits(),
        49 => plic.priority_49.read().bits(),
        50 => plic.priority_50.read().bits(),
        51 => plic.priority_51.read().bits(),
        52 => plic.priority_52.read().bits(),
        53 => plic.priority_53.read().bits(),
        54 => plic.priority_54.read().bits(),
        55 => plic.priority_55.read().bits(),
        56 => plic.priority_56.read().bits(),
        57 => plic.priority_57.read().bits(),
        58 => plic.priority_58.read().bits(),
        59 => plic.priority_59.read().bits(),
        60 => plic.priority_60.read().bits(),
        61 => plic.priority_61.read().bits(),
        62 => plic.priority_62.read().bits(),
        63 => plic.priority_63.read().bits(),
        64 => plic.priority_64.read().bits(),
        65 => plic.priority_65.read().bits(),
        66 => plic.priority_66.read().bits(),
        67 => plic.priority_67.read().bits(),
        68 => plic.priority_68.read().bits(),
        69 => plic.priority_69.read().bits(),
        _ => panic!(),
    }
}

pub fn plic_set_intr_priority(intr: InterruptSource, priority: u32) {
    if priority >= 8 {
        panic!()
    }
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    let intr_id = Into::<u8>::into(intr);
    match intr_id {
        1 => plic.priority_1.modify(|_, w| unsafe { w.bits(priority) }),
        2 => plic.priority_2.modify(|_, w| unsafe { w.bits(priority) }),
        3 => plic.priority_3.modify(|_, w| unsafe { w.bits(priority) }),
        4 => plic.priority_4.modify(|_, w| unsafe { w.bits(priority) }),
        5 => plic.priority_5.modify(|_, w| unsafe { w.bits(priority) }),
        6 => plic.priority_6.modify(|_, w| unsafe { w.bits(priority) }),
        7 => plic.priority_7.modify(|_, w| unsafe { w.bits(priority) }),
        8 => plic.priority_8.modify(|_, w| unsafe { w.bits(priority) }),
        9 => plic.priority_9.modify(|_, w| unsafe { w.bits(priority) }),
        10 => plic.priority_10.modify(|_, w| unsafe { w.bits(priority) }),
        11 => plic.priority_11.modify(|_, w| unsafe { w.bits(priority) }),
        12 => plic.priority_12.modify(|_, w| unsafe { w.bits(priority) }),
        13 => plic.priority_13.modify(|_, w| unsafe { w.bits(priority) }),
        14 => plic.priority_14.modify(|_, w| unsafe { w.bits(priority) }),
        15 => plic.priority_15.modify(|_, w| unsafe { w.bits(priority) }),
        16 => plic.priority_16.modify(|_, w| unsafe { w.bits(priority) }),
        17 => plic.priority_17.modify(|_, w| unsafe { w.bits(priority) }),
        18 => plic.priority_18.modify(|_, w| unsafe { w.bits(priority) }),
        19 => plic.priority_19.modify(|_, w| unsafe { w.bits(priority) }),
        20 => plic.priority_20.modify(|_, w| unsafe { w.bits(priority) }),
        21 => plic.priority_21.modify(|_, w| unsafe { w.bits(priority) }),
        22 => plic.priority_22.modify(|_, w| unsafe { w.bits(priority) }),
        23 => plic.priority_23.modify(|_, w| unsafe { w.bits(priority) }),
        24 => plic.priority_24.modify(|_, w| unsafe { w.bits(priority) }),
        25 => plic.priority_25.modify(|_, w| unsafe { w.bits(priority) }),
        26 => plic.priority_26.modify(|_, w| unsafe { w.bits(priority) }),
        27 => plic.priority_27.modify(|_, w| unsafe { w.bits(priority) }),
        28 => plic.priority_28.modify(|_, w| unsafe { w.bits(priority) }),
        29 => plic.priority_29.modify(|_, w| unsafe { w.bits(priority) }),
        30 => plic.priority_30.modify(|_, w| unsafe { w.bits(priority) }),
        31 => plic.priority_31.modify(|_, w| unsafe { w.bits(priority) }),
        32 => plic.priority_32.modify(|_, w| unsafe { w.bits(priority) }),
        33 => plic.priority_33.modify(|_, w| unsafe { w.bits(priority) }),
        34 => plic.priority_34.modify(|_, w| unsafe { w.bits(priority) }),
        35 => plic.priority_35.modify(|_, w| unsafe { w.bits(priority) }),
        36 => plic.priority_36.modify(|_, w| unsafe { w.bits(priority) }),
        37 => plic.priority_37.modify(|_, w| unsafe { w.bits(priority) }),
        38 => plic.priority_38.modify(|_, w| unsafe { w.bits(priority) }),
        39 => plic.priority_39.modify(|_, w| unsafe { w.bits(priority) }),
        40 => plic.priority_40.modify(|_, w| unsafe { w.bits(priority) }),
        41 => plic.priority_41.modify(|_, w| unsafe { w.bits(priority) }),
        42 => plic.priority_42.modify(|_, w| unsafe { w.bits(priority) }),
        43 => plic.priority_43.modify(|_, w| unsafe { w.bits(priority) }),
        44 => plic.priority_44.modify(|_, w| unsafe { w.bits(priority) }),
        45 => plic.priority_45.modify(|_, w| unsafe { w.bits(priority) }),
        46 => plic.priority_46.modify(|_, w| unsafe { w.bits(priority) }),
        47 => plic.priority_47.modify(|_, w| unsafe { w.bits(priority) }),
        48 => plic.priority_48.modify(|_, w| unsafe { w.bits(priority) }),
        49 => plic.priority_49.modify(|_, w| unsafe { w.bits(priority) }),
        50 => plic.priority_50.modify(|_, w| unsafe { w.bits(priority) }),
        51 => plic.priority_51.modify(|_, w| unsafe { w.bits(priority) }),
        52 => plic.priority_52.modify(|_, w| unsafe { w.bits(priority) }),
        53 => plic.priority_53.modify(|_, w| unsafe { w.bits(priority) }),
        54 => plic.priority_54.modify(|_, w| unsafe { w.bits(priority) }),
        55 => plic.priority_55.modify(|_, w| unsafe { w.bits(priority) }),
        56 => plic.priority_56.modify(|_, w| unsafe { w.bits(priority) }),
        57 => plic.priority_57.modify(|_, w| unsafe { w.bits(priority) }),
        58 => plic.priority_58.modify(|_, w| unsafe { w.bits(priority) }),
        59 => plic.priority_59.modify(|_, w| unsafe { w.bits(priority) }),
        60 => plic.priority_60.modify(|_, w| unsafe { w.bits(priority) }),
        61 => plic.priority_61.modify(|_, w| unsafe { w.bits(priority) }),
        62 => plic.priority_62.modify(|_, w| unsafe { w.bits(priority) }),
        63 => plic.priority_63.modify(|_, w| unsafe { w.bits(priority) }),
        64 => plic.priority_64.modify(|_, w| unsafe { w.bits(priority) }),
        65 => plic.priority_65.modify(|_, w| unsafe { w.bits(priority) }),
        66 => plic.priority_66.modify(|_, w| unsafe { w.bits(priority) }),
        67 => plic.priority_67.modify(|_, w| unsafe { w.bits(priority) }),
        68 => plic.priority_68.modify(|_, w| unsafe { w.bits(priority) }),
        69 => plic.priority_69.modify(|_, w| unsafe { w.bits(priority) }),
        _ => panic!(),
    }
}

pub fn plic_intr_pending(intr: InterruptSource) -> bool {
    let intr_info = IntrInfo::from(Into::<u8>::into(intr));
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    match intr_info.0 {
        0 => (plic.pending_0.read().bits() >> intr_info.1) == 1,
        1 => (plic.pending_1.read().bits() >> intr_info.1) == 1,
        2 => (plic.pending_2.read().bits() >> intr_info.1) == 1,
        _ => panic!(),
    }
}

pub fn plic_intr_enable(intr: InterruptSource) -> bool {
    let hart = local_hart().hart_id();
    let intr_info = IntrInfo::from(Into::<u8>::into(intr));
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    match (intr_info.0, hart) {
        (0, 1) => (plic.enable_0_1S.read().bits() >> intr_info.1) == 1,
        (0, 2) => (plic.enable_0_2S.read().bits() >> intr_info.1) == 1,
        (0, 3) => (plic.enable_0_3S.read().bits() >> intr_info.1) == 1,
        (0, 4) => (plic.enable_0_4S.read().bits() >> intr_info.1) == 1,
        (1, 1) => (plic.enable_1_1S.read().bits() >> intr_info.1) == 1,
        (1, 2) => (plic.enable_1_2S.read().bits() >> intr_info.1) == 1,
        (1, 3) => (plic.enable_1_3S.read().bits() >> intr_info.1) == 1,
        (1, 4) => (plic.enable_1_4S.read().bits() >> intr_info.1) == 1,
        (2, 1) => (plic.enable_2_1S.read().bits() >> intr_info.1) == 1,
        (2, 2) => (plic.enable_2_2S.read().bits() >> intr_info.1) == 1,
        (2, 3) => (plic.enable_2_3S.read().bits() >> intr_info.1) == 1,
        (2, 4) => (plic.enable_2_4S.read().bits() >> intr_info.1) == 1,
        _ => panic!(),
    }
}

pub fn plic_set_intr_enable(intr: InterruptSource) {
    let hart = local_hart().hart_id();
    let intr_info = IntrInfo::from(Into::<u8>::into(intr));
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    match (intr_info.0, hart) {
        (0, 1) => plic
            .enable_0_1S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (0, 2) => plic
            .enable_0_2S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (0, 3) => plic
            .enable_0_3S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (0, 4) => plic
            .enable_0_4S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (1, 1) => plic
            .enable_1_1S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (1, 2) => plic
            .enable_1_2S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (1, 3) => plic
            .enable_1_3S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (1, 4) => plic
            .enable_1_4S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (2, 1) => plic
            .enable_2_1S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (2, 2) => plic
            .enable_2_2S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (2, 3) => plic
            .enable_2_3S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        (2, 4) => plic
            .enable_2_4S
            .modify(|r, w| unsafe { w.bits(r.bits() | (1_u32 << intr_info.1)) }),
        _ => panic!(),
    }
}

pub fn plic_set_intr_disable(intr: InterruptSource) {
    let hart = local_hart().hart_id();
    let intr_info = IntrInfo::from(Into::<u8>::into(intr));
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    match (intr_info.0, hart) {
        (0, 1) => plic
            .enable_0_1S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (0, 2) => plic
            .enable_0_2S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (0, 3) => plic
            .enable_0_3S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (0, 4) => plic
            .enable_0_4S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (1, 1) => plic
            .enable_1_1S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (1, 2) => plic
            .enable_1_2S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (1, 3) => plic
            .enable_1_3S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (1, 4) => plic
            .enable_1_4S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (2, 1) => plic
            .enable_2_1S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (2, 2) => plic
            .enable_2_2S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (2, 3) => plic
            .enable_2_3S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        (2, 4) => plic
            .enable_2_4S
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1_u32 << intr_info.1)) }),
        _ => panic!(),
    }
}

pub fn plic_intr_threshold() -> u32 {
    let hart = local_hart().hart_id();
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    match hart {
        1 => plic.threshold_1S.read().bits(),
        2 => plic.threshold_2S.read().bits(),
        3 => plic.threshold_3S.read().bits(),
        4 => plic.threshold_4S.read().bits(),
        _ => panic!(),
    }
}

pub fn plic_set_intr_threshold(threshold: u32) {
    if threshold >= 8 {
        panic!()
    }
    let hart = local_hart().hart_id();
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    match hart {
        1 => plic
            .threshold_1S
            .modify(|_, w| unsafe { w.bits(threshold) }),
        2 => plic
            .threshold_2S
            .modify(|_, w| unsafe { w.bits(threshold) }),
        3 => plic
            .threshold_3S
            .modify(|_, w| unsafe { w.bits(threshold) }),
        4 => plic
            .threshold_4S
            .modify(|_, w| unsafe { w.bits(threshold) }),
        _ => panic!(),
    }
}

pub fn plic_claim() -> InterruptSource {
    let hart = local_hart().hart_id();
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    let intr_id = match hart {
        1 => plic.claimplete_1S.read().bits(),
        2 => plic.claimplete_2S.read().bits(),
        3 => plic.claimplete_3S.read().bits(),
        4 => plic.claimplete_4S.read().bits(),
        _ => panic!(),
    };
    From::<u8>::from(intr_id as u8)
}

pub fn plic_complete(intr: InterruptSource) {
    let hart = local_hart().hart_id();
    let intr_id = Into::<u8>::into(intr) as u32;
    let plic = PLIC_INSTANCE.lock();
    let plic = plic.as_ref().unwrap();
    unsafe {
        match hart {
            1 => plic.claimplete_1S.write_with_zero(|w| w.bits(intr_id)),
            _ => panic!(),
        }
    }
}

pub fn plicinit() {
    plic_set_intr_priority(InterruptSource::UART0, 2);
    plic_set_intr_priority(InterruptSource::SPI2, 2);
}

pub fn plic_inithart() {
    plic_set_intr_enable(InterruptSource::UART0);
    plic_set_intr_enable(InterruptSource::SPI2);
    plic_set_intr_threshold(0);
}
