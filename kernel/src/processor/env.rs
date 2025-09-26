use crate::processor::{local_irq_disable, local_irq_enable, local_irq_is_enabled};

use super::local_env;

/// use RAII to guard `sum` flag
pub struct SumGuard {}

impl SumGuard {
    pub fn new() -> Self {
        local_env().sum_inc();
        Self {}
    }
}

impl Drop for SumGuard {
    fn drop(&mut self) {
        local_env().sum_dec();
    }
}

/// use RAII to guard `sie` flag
pub struct IrqEnableGuard(bool);

impl IrqEnableGuard {
    #[allow(unused)]
    pub fn new() -> Self {
        Self({
            let before = local_irq_is_enabled();
            local_irq_disable();
            before
        })
    }
}

impl Drop for IrqEnableGuard {
    fn drop(&mut self) {
        if self.0 {
            local_irq_enable();
        }
    }
}
