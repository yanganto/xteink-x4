#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    main,
    time::{Duration, Instant},
};
use esp_println::println;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    println!("Xteink X4 booted!");

    let mut count = 0u32;
    loop {
        println!("Tick {}", count);
        count = count.wrapping_add(1);

        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(1000) {}
    }
}
