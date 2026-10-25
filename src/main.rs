#![no_std]
#![no_main]

use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
    rtc_cntl::{
        Rtc,
        sleep::{RtcioWakeupSource, WakeupLevel},
    },
    spi::master::{Config as SpiConfig, Spi},
    time::{Duration, Instant, Rate},
};
use esp_println::{print, println};

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use ssd1677_driver::{
    ConfigBuilder, Dimensions, Display, Rotation,
    basic_display::DisplayUpdateMode,
    interface::Interface4Pin,
};

// GPIO3 is the power button (active low, internal pull-up)
// Hold for 3 seconds to power off; hold again to wake from deep sleep
const POWER_HOLD_SECS: u64 = 3;

// 800 × 480 px, 1 bit per pixel — kept in static to avoid stack overflow
static mut DISPLAY_BUFFER: [u8; 800 * 480 / 8] = [0xFF; 800 * 480 / 8];

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // SPI2 bus — SCK: GPIO8, MOSI: GPIO10 (display is write-only, no MISO needed)
    let spi_bus = Spi::new(
        peripherals.SPI2,
        SpiConfig::default().with_frequency(Rate::from_mhz(4)),
    )
    .unwrap()
    .with_sck(peripherals.GPIO8)
    .with_mosi(peripherals.GPIO10);

    let cs   = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    let dc   = Output::new(peripherals.GPIO4,  Level::Low,  OutputConfig::default());
    let rst  = Output::new(peripherals.GPIO5,  Level::High, OutputConfig::default());
    let busy = Input::new(peripherals.GPIO6, InputConfig::default());

    let spi       = ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap();
    let interface = Interface4Pin::new(spi, dc, rst, busy);

    let disp_config = ConfigBuilder::new()
        .dimensions(Dimensions { rows: 480, cols: 800 })
        .rotation(Rotation::Rotate0)
        .auto_update(false)
        .build()
        .unwrap();

    let mut delay = Delay::new();
    // SAFETY: single-threaded bare-metal; addr_of_mut avoids the static_mut_refs lint
    let buf: &mut [u8] = unsafe { &mut *core::ptr::addr_of_mut!(DISPLAY_BUFFER) };
    let mut display = Display::new(interface, buf, disp_config);
    display.reset(&mut delay).unwrap();

    // Show "starting.." on the e-ink display at boot
    let text_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    display.clear(BinaryColor::Off).unwrap();
    Text::new("starting..", Point::new(350, 240), text_style)
        .draw(&mut display)
        .unwrap();
    display.update(DisplayUpdateMode::Slow).unwrap();

    println!("Xteink X4 booted!");

    let mut gpio3 = peripherals.GPIO3;
    let button = Input::new(gpio3.reborrow(), InputConfig::default().with_pull(Pull::Up));

    let mut rtc = Rtc::new(peripherals.LPWR);

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

                    // Show "shutdown..." before entering deep sleep
                    display.clear(BinaryColor::Off).unwrap();
                    Text::new("shutdown...", Point::new(345, 240), text_style)
                        .draw(&mut display)
                        .unwrap();
                    display.update(DisplayUpdateMode::Slow).unwrap();

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

        if count % 10 == 0 {
            println!(". {}s", count);
        } else {
            print!(".");
        }
        count = count.wrapping_add(1);

        // Update time every 10 seconds using the existing count variable
        if count % 10 == 0 {
            // Show time on display - very simple approach using integer conversion directly
            display.clear(BinaryColor::Off).unwrap();
            
            let hours = (count / 3600) % 24;
            let minutes = (count / 60) % 60;
            let seconds = count % 60;
            
            // Directly create buffer and convert numbers to ASCII - simpler but less optimal approach
            let mut buffer = [b' '; 9]; // Preallocate 8 chars + null terminator
                                        //
            // Show only the raw numbers to simplify string handling for embedded environment
            // Format manually using character manipulation that should work with embedded constraints
            let time_str = {
                let h = hours;
                let m = minutes;
                let s = seconds;
                
                // Simple manual conversion for hours (assuming single digit for now)
                if h < 10 {
                    buffer[0] = b'0';
                    buffer[1] = b'0' + h as u8;
                } else {
                    buffer[0] = b'0' + (h / 10) as u8;
                    buffer[1] = b'0' + (h % 10) as u8;
                }
                
                buffer[2] = b':';
                
                if m < 10 {
                    buffer[3] = b'0';
                    buffer[4] = b'0' + m as u8;
                } else {
                    buffer[3] = b'0' + (m / 10) as u8;
                    buffer[4] = b'0' + (m % 10) as u8;
                }
                
                buffer[5] = b':';
                
                if s < 10 {
                    buffer[6] = b'0';
                    buffer[7] = b'0' + s as u8;
                } else {
                    buffer[6] = b'0' + (s / 10) as u8;
                    buffer[7] = b'0' + (s % 10) as u8;
                }
                
                // Convert to string slice - this is much simpler and should avoid the lifetime issues
                core::str::from_utf8(&buffer[..8]).unwrap_or("00:00:00")
            };
            
            Text::new(&time_str, Point::new(350, 240), text_style)
                .draw(&mut display)
                .unwrap();
            display.update(DisplayUpdateMode::Slow).unwrap();
        }

        let tick_start = Instant::now();
        while tick_start.elapsed() < Duration::from_millis(1000) {}
    }
}
