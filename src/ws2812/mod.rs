use esp_idf_sys::{
    rmt_channel_t, rmt_config, rmt_config_t, rmt_driver_install,
    rmt_idle_level_t_RMT_IDLE_LEVEL_LOW, rmt_item32_t, rmt_mode_t_RMT_MODE_TX, rmt_wait_tx_done,
    rmt_write_items,
};

const T0H: u32 = 14;
const T1H: u32 = 52;
const TL: u32 = 52;

pub(crate) unsafe fn setup_rmt(gpio_num: i32, channel: rmt_channel_t) {
    let mut rmt_cfg: rmt_config_t = std::mem::zeroed();
    rmt_cfg.rmt_mode = rmt_mode_t_RMT_MODE_TX;
    rmt_cfg.channel = channel;
    rmt_cfg.gpio_num = gpio_num;
    rmt_cfg.mem_block_num = 1;
    rmt_cfg.__bindgen_anon_1.tx_config.loop_en = false;
    rmt_cfg.__bindgen_anon_1.tx_config.carrier_en = false;
    rmt_cfg.__bindgen_anon_1.tx_config.idle_output_en = true;
    rmt_cfg.__bindgen_anon_1.tx_config.idle_level = rmt_idle_level_t_RMT_IDLE_LEVEL_LOW;
    rmt_cfg.__bindgen_anon_1.tx_config.carrier_duty_percent = 50;
    rmt_cfg.clk_div = 1;

    rmt_config(&mut rmt_cfg);
    rmt_driver_install(channel, 0, 0);
}

pub(crate) unsafe fn ws2812_send_color(channel: rmt_channel_t, r: u8, g: u8, b: u8) {
    let bytes = [r, g, b];
    let mut items: Vec<rmt_item32_t> = Vec::with_capacity(24);

    for &byte in &bytes {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            let (high, low) = if bit == 1 { (T1H, TL) } else { (T0H, TL) };
            let mut item: rmt_item32_t = std::mem::zeroed();
            item.__bindgen_anon_1.__bindgen_anon_1.set_duration0(high);
            item.__bindgen_anon_1.__bindgen_anon_1.set_level0(1);
            item.__bindgen_anon_1.__bindgen_anon_1.set_duration1(low);
            item.__bindgen_anon_1.__bindgen_anon_1.set_level1(0);

            items.push(item);
        }
    }

    rmt_write_items(channel, items.as_ptr(), items.len() as i32, true);
    rmt_wait_tx_done(channel, 60);
}
