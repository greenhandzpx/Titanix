//! Synchronization and interior mutability primitives
mod cond_var;
pub use cond_var::CondVar;

/// Different kinds of mutex
pub mod mutex;
