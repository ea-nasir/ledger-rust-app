#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::my_runner)]
#![no_main]

use nanos_sdk::{debug_print, exit_app};

// Need this to ensure main_mod gets included
use rust_app::*;

fn my_runner(_: &[&i32]) {
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    exit_app(0);
}

// Stub to trigger a build with ts-tests; this will actually be intercepted by speculos-wrapper to
// as a trigger to run the typescript tests.
