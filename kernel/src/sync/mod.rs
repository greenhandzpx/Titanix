//! Synchronization and interior mutability primitives
mod cond_var;
pub use cond_var::CondVar;

mod mailbox;
/// Different kinds of mutex
pub mod mutex;
pub use mailbox::Event;
pub use mailbox::Mailbox;
