//! Synchronization and interior mutability primitives

mod futex;

pub use futex::FutexQueue;
pub use futex::FutexFuture;

mod mailbox;
/// Different kinds of mutex
pub mod mutex;
pub use mailbox::Event;
pub use mailbox::Mailbox;
