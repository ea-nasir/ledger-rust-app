#![cfg_attr(target_os = "nanos", no_std)]
#![cfg_attr(target_os = "nanos", no_main)]

#[cfg(not(target_os = "nanos"))]
fn main() {}

use rust_app::*;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);
