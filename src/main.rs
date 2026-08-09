#![no_std]
#![no_main]

use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    main,
    rtc_cntl::{
        Rtc,
        sleep::{RtcioWakeupSource, WakeupLevel},
    },
    time::{Duration, Instant},
};
use esp_println::{print, println};

// GPIO3 is the power button (active low, internal pull-up)
// Hold for 3 seconds to power off; hold again to wake from deep sleep
const POWER_HOLD_SECS: u64 = 3;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    println!("Xteink X4 booted!");

    let mut gpio3 = peripherals.GPIO3;
    let button = Input::new(gpio3.reborrow(), InputConfig::default().with_pull(Pull::Up));

    let mut rtc = Rtc::new(peripherals.LPWR);
    let delay = Delay::new();

    let mut pwd_pressed: Option<Instant> = None;
    let mut count = 0u32;

    loop {
        if button.is_low() {
            if pwd_pressed.is_none() {
                pwd_pressed = Some(Instant::now());
            }
            if let Some(t) = pwd_pressed {
                if t.elapsed() >= Duration::from_secs(POWER_HOLD_SECS) {
                    println!("Power button held 3s — entering deep sleep");
                    drop(button);
                    let wakeup_pins: &mut [(&mut dyn esp_hal::gpio::RtcPinWithResistors, WakeupLevel)] =
                        &mut [(&mut gpio3, WakeupLevel::Low)];
                    let rtcio = RtcioWakeupSource::new(wakeup_pins);
                    // Small delay to let the serial message flush
                    delay.delay_millis(100u32);
                    rtc.sleep_deep(&[&rtcio]);
                }
            }
        } else {
            pwd_pressed = None; 
        }

        if count % 10 == 0{
            println!(". {}s", count);
        } else {
            print!(".");
        }
        count = count.wrapping_add(1);

        let tick_start = Instant::now();
        while tick_start.elapsed() < Duration::from_millis(1000) {}
    }
}
