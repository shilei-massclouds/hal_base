#![cfg_attr(not(target_os = "linux"), no_std)]
#![feature(exclusive_wrapper)]
#![feature(naked_functions)]

mod platform;
pub use platform::terminate;

mod time;
pub use time::current_time;

pub mod arch;
pub mod cpu;
pub mod trap;
pub mod paging;

pub fn init() {
}
