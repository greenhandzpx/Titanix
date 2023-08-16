pub mod sdcard;
pub mod spi;
pub mod uart;

pub enum IntrSource {
    UART0 = 39,
    SPI2 = 43,
    UnknownIntr,
}

impl From<usize> for IntrSource {
    fn from(value: usize) -> Self {
        match value {
            39 => Self::UART0,
            43 => Self::SPI2,
            _ => Self::UnknownIntr,
        }
    }
}
