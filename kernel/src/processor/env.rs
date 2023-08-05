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

// /// use RAII to guard `sie` flag
// pub struct SieGuard {}

// impl SieGuard {
//     #[allow(unused)]
//     pub fn new() -> Self {
//         local_env().sie_dec();
//         Self {}
//     }
// }

// impl Drop for SieGuard {
//     fn drop(&mut self) {
//         local_env().sie_inc();
//     }
// }
