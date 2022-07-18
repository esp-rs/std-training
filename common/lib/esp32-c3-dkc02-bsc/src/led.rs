use std::ptr::{null, null_mut};

use esp_idf_sys::{
    c_types::c_void, esp, rmt_config, rmt_config_t, rmt_config_t__bindgen_ty_1, rmt_driver_install,
    rmt_get_counter_clock, rmt_item32_t, rmt_item32_t__bindgen_ty_1,
    rmt_item32_t__bindgen_ty_1__bindgen_ty_1, rmt_mode_t_RMT_MODE_TX, rmt_translator_init,
    rmt_tx_config_t, rmt_wait_tx_done, rmt_write_sample, size_t, u_int8_t,
};
pub use rgb::RGB8;

const WS2812_T0H_NS: u32 = 350;
const WS2812_T0L_NS: u32 = 1000;
const WS2812_T1H_NS: u32 = 1000;
const WS2812_T1L_NS: u32 = 350;

#[derive(Debug, Default, Clone, Copy)]
struct Ws2812Config {
    t0h_ticks: u32,
    t0l_ticks: u32,
    t1h_ticks: u32,
    t1l_ticks: u32,
}

const FREERTOS_HZ: u32 = 1000;

static mut WS_CONFIG: Option<Ws2812Config> = None;

unsafe extern "C" fn ws2812_to_rmt(
    src: *const c_void,
    dest: *mut rmt_item32_t,
    src_size: size_t,
    wanted_num: size_t,
    translated_size: *mut size_t,
    item_num: *mut size_t,
) {
    if src == null() || dest == null_mut() {
        *translated_size = 0;
        *item_num = 0;
        return;
    }

    let config = WS_CONFIG.unwrap();
    let mut bit0: rmt_item32_t__bindgen_ty_1__bindgen_ty_1 = Default::default();
    bit0.set_duration0(config.t0h_ticks);
    bit0.set_level0(1);
    bit0.set_duration1(config.t0l_ticks);
    bit0.set_level1(0);

    let bit0 = rmt_item32_t {
        __bindgen_anon_1: rmt_item32_t__bindgen_ty_1 {
            __bindgen_anon_1: bit0,
        },
    };

    let mut bit1: rmt_item32_t__bindgen_ty_1__bindgen_ty_1 = Default::default();
    bit1.set_duration0(config.t1h_ticks);
    bit1.set_level0(1);
    bit1.set_duration1(config.t1l_ticks);
    bit1.set_level1(0);

    let bit1 = rmt_item32_t {
        __bindgen_anon_1: rmt_item32_t__bindgen_ty_1 {
            __bindgen_anon_1: bit1,
        },
    };

    let mut size: size_t = 0;
    let mut num = 0;

    let mut psrc = src as *const u_int8_t;
    let mut pdest: *mut rmt_item32_t = dest as _;

    while size < src_size && num < wanted_num {
        for i in 0..8 {
            if *psrc & (1 << (7 - i)) != 0 {
                *pdest = bit1;
            } else {
                *pdest = bit0;
            }
            num += 1;
            pdest = pdest.add(1);
        }
        size += 1;
        psrc = psrc.add(1);
    }
    *translated_size = size;
    *item_num = num;
}

pub struct WS2812RMT {
    config: rmt_config_t,
}
impl WS2812RMT {
    pub fn new() -> anyhow::Result<Self> {
        let rmt_tx_config = rmt_tx_config_t {
            carrier_freq_hz: 38000,
            carrier_level: 1,
            idle_level: 0,
            carrier_duty_percent: 33,
            loop_count: 1,
            carrier_en: false,
            loop_en: false,
            idle_output_en: true,
        };

        let config = rmt_config_t {
            rmt_mode: rmt_mode_t_RMT_MODE_TX,
            channel: 0,
            gpio_num: 2,
            clk_div: 2,
            mem_block_num: 1,
            flags: 0,
            __bindgen_anon_1: rmt_config_t__bindgen_ty_1 {
                tx_config: rmt_tx_config,
            },
        };

        unsafe {
            esp!(rmt_config(&config))?;
            esp!(rmt_driver_install(config.channel, 0, 0))?;
            let mut rmt_clock = 0u32;
            esp!(rmt_get_counter_clock(config.channel, &mut rmt_clock))?;

            let ratio = rmt_clock as f64 / 1e9;

            WS_CONFIG = Some(Ws2812Config {
                t0h_ticks: (ratio * WS2812_T0H_NS as f64) as _,
                t0l_ticks: (ratio * WS2812_T0L_NS as f64) as _,
                t1h_ticks: (ratio * WS2812_T1H_NS as f64) as _,
                t1l_ticks: (ratio * WS2812_T1L_NS as f64) as _,
            });

            esp!(rmt_translator_init(config.channel, Some(ws2812_to_rmt)))?;
        }

        Ok(Self { config })
    }

    pub fn set_pixel(&mut self, color: RGB8) -> anyhow::Result<()> {
        let timeout_ms = 1;
        unsafe {
            esp!(rmt_write_sample(
                self.config.channel,
                &[color.g, color.r, color.b] as *const u8, // WS2812 expects GRB, not RGB
                3,
                true,
            ))?;
            esp!(rmt_wait_tx_done(
                self.config.channel,
                (timeout_ms as u32 * FREERTOS_HZ) / 1000,
            ))?;
        }

        Ok(())
    }
}
