mod ws2812;

use crate::ws2812::{setup_rmt, ws2812_send_color};
use esp_idf_sys::*;
use std::{thread, time::Duration};

fn get_random_u8() -> u8 {
    let mut buf = [0u8; 1];
    unsafe { esp_fill_random(buf.as_mut_ptr() as *mut _, 1) };
    buf[0]
}

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let gpio_num = 8;
    let channel = rmt_channel_t_RMT_CHANNEL_0;

    unsafe {
        setup_rmt(gpio_num, channel);

        loop {
            let red: u8 = get_random_u8() % 100;
            let green: u8 = get_random_u8() % 100;
            let blue: u8 = get_random_u8() % 100;

            ws2812_send_color(channel, red, green, blue);
            thread::sleep(Duration::from_millis(1000));
            log::info!("RED :{} , GREEN: {}, BLUE: {}", red, green, blue);
        }
    }
}
