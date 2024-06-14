#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;
use rp_pico as _;



#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

// 無限ループする関数
pub fn exit() -> ! {
    loop {
        
    }
}

// メイン関数
#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    exit()
}