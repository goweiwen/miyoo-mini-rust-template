#[cfg(target_arch = "arm")]
mod miyoo;

#[cfg(not(target_arch = "arm"))]
mod simulator;

#[cfg(target_arch = "arm")]
pub use miyoo::*;

#[cfg(not(target_arch = "arm"))]
pub use simulator::*;
