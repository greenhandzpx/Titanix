pub mod uart;
pub mod virtio_blk;

pub enum IntrSource {
    UART0 = 10,
    VIRTIO0 = 1,
    UnknownIntr,
}

impl From<usize> for IntrSource {
    fn from(value: usize) -> Self {
        match value {
            10 => Self::UART0,
            1 => Self::VIRTIO0,
            _ => Self::UnknownIntr,
        }
    }
}
