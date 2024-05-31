#![no_std]
#![no_main]

use core::pin;

use futures::stream::SelectWithStrategy;
use panic_halt as _;
use rp2040_hal as hal;
use hal::pac;
use embedded_hal::digital::v2::{OutputPin, InputPin};
use rp2040_hal::clocks::Clock;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // 16~18番ピンを出力にする
    let mut pin_16 = pins.gpio16.into_push_pull_output();
    let mut pin_17 = pins.gpio17.into_push_pull_output();
    let mut pin_18 = pins.gpio18.into_push_pull_output();


    // 
    let switch_blue = pins.gpio14.into_pull_up_input();
    let switch_red = pins.gpio15.into_pull_up_input();

    let switch_up = pins.gpio0.into_pull_up_input();


    loop {
        if switch_blue.is_low().unwrap() {
            pin_18.set_high().unwrap();
            delay.delay_ms(50);
            pin_18.set_low().unwrap();
            delay.delay_ms(50);
        } else {
            pin_18.set_low().unwrap();
        }

        if switch_red.is_low().unwrap() {
            pin_17.set_high().unwrap();
            delay.delay_ms(50);
            pin_17.set_low().unwrap();
            delay.delay_ms(50);
        } else {
            pin_17.set_low().unwrap();
        }

        if switch_up.is_low().unwrap() {
            pin_16.set_high().unwrap();
            delay.delay_ms(50);
            pin_16.set_low().unwrap();
            delay.delay_ms(50);
        } else {
            pin_16.set_low().unwrap();
        }
    }
}
