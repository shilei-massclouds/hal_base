#![cfg_attr(not(target_os = "linux"), no_std)]
#![feature(exclusive_wrapper)]

mod platform;
pub use platform::terminate;

mod time;
pub use time::current_time;

pub fn init() {
}
