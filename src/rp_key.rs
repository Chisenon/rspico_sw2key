#![no_main]
#![no_std]

use bsp::hal::{self, usb::UsbBus};
use cortex_m_rt::entry;
use defmt_rtt as _;
use hal::pac;
use panic_probe as _;
use rp_pico as bsp;
use usb_device as usbd;

use embedded_hal::digital::v2::InputPin;

use usbd::{
    class_prelude::UsbBusAllocator,
    device::{UsbDeviceBuilder, UsbVidPid},
};

use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::{
        HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass,
        ProtocolModeConfig,
    },
};

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(p.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        p.XOSC,
        p.CLOCKS,
        p.PLL_SYS,
        p.PLL_USB,
        &mut p.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let bus = UsbBus::new(
        p.USBCTRL_REGS,
        p.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut p.RESETS,
    );

    let bus_allocator = UsbBusAllocator::new(bus);
    let vid_pid = UsbVidPid(0x3104, 0x0418);
    let mut hid = HIDClass::new_with_settings(
        &bus_allocator,
        KeyboardReport::desc(),
        10,
        HidClassSettings {
            subclass: HidSubClass::NoSubClass,
            protocol: HidProtocol::Keyboard,
            config: ProtocolModeConfig::ForceReport,
            locale: HidCountryCode::NotSupported,
        },
    );
    let mut dev = UsbDeviceBuilder::new(&bus_allocator, vid_pid)
        .build();

    let sio = hal::Sio::new(p.SIO);
    let pins = bsp::Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);

    let ble_btn = pins.gpio16.into_pull_up_input();
    let red_btn = pins.gpio17.into_pull_up_input();
    let up_btn = pins.gpio18.into_pull_up_input();
    let dwn_btn = pins.gpio19.into_pull_up_input();
    let lft_btn = pins.gpio20.into_pull_up_input();
    let rgt_btn = pins.gpio21.into_pull_up_input();

    // defmt::println!("Hello, world!");// コンパイル確認用

    loop {
        let mut keys = [0u8; 6];
        let num_keys = 0;

        dev.poll(&mut [&mut hid]);


        if ble_btn.is_low().unwrap() {
            keys[num_keys] = 0x1D;// z キー
            // num_keys += 1;

            defmt::println!("Blue");
        }
        
        if red_btn.is_low().unwrap() {
            keys[num_keys] = 0x1B;// x キー
            // num_keys += 1;

            defmt::println!("red");
        }

        if up_btn.is_low().unwrap() {
            // keys[num_keys] = 0x52;// Up キー
            keys[num_keys] = 0x0C;// i キー
            // num_keys += 1;

            defmt::println!("Up");
        }

        if dwn_btn.is_low().unwrap() {
            // keys[num_keys] = 0x51;// Down キー
            keys[num_keys] = 0x0E;// j キー
            // num_keys += 1;

            defmt::println!("Down");
        }

        if lft_btn.is_low().unwrap() {
            // keys[num_keys] = 0x50;// Left キー
            keys[num_keys] = 0x0D;// k キー
            // num_keys += 1;

            defmt::println!("Left");
        }

        if rgt_btn.is_low().unwrap() {
            // keys[num_keys] = 0x4F; // Right キー
            keys[num_keys] = 0x0F;// l キー
            // num_keys += 1;

            defmt::println!("Rigt");
        }

        let report = usbd_hid::descriptor::KeyboardReport{
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: keys,
        };

        
        hid.push_input(&report).ok();
    }
}