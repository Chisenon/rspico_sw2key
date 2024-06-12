#![no_std]
#![no_main]

use hal::pac;
use panic_halt as _;
use rp2040_hal as hal;

use rp2040_hal::clocks::Clock;

use embedded_hal::digital::v2::{InputPin, OutputPin};

use usb_device::{class_prelude::*, prelude::*};

use usbd_serial::SerialPort;

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

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));


    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // // Initialize pins 16, 17, 18 & 25 as outputs
    // let mut pin_16 = pins.gpio16.into_push_pull_output();
    // let mut pin_17 = pins.gpio17.into_push_pull_output();
    // let mut pin_18 = pins.gpio18.into_push_pull_output();
    let mut pin_25 = pins.gpio25.into_push_pull_output();

    // Initialize switch pins as inputs with pull-up resistors
    let switch_blue = pins.gpio14.into_pull_up_input();
    let switch_red = pins.gpio15.into_pull_up_input();
    let switch_up = pins.gpio0.into_pull_up_input();

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(2)
        .build();

    loop {
        let blue_active = switch_blue.is_low().unwrap();
        let red_active = switch_red.is_low().unwrap();
        let up_active = switch_up.is_low().unwrap();
        
        let _ = usb_dev.poll(&mut [&mut serial]);

        if !blue_active && !red_active && !up_active {
            pin_25.set_high().unwrap();
            delay.delay_ms(100);
            pin_25.set_low().unwrap();
            delay.delay_ms(100);
        } else {
            let _ = serial.write(b"Hello, world!\r\n");
            pin_25.set_low().unwrap();
        }
        delay.delay_ms(5);

        // let _ = serial.write(b"A");
    }
}
