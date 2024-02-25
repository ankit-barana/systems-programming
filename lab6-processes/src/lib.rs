mod proc;

pub use proc::Process;
pub type Result<T> = std::result::Result<T, Box <dyn std::error::Error>>;