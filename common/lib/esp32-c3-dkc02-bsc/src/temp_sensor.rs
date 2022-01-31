use esp32c3::{Peripherals, APB_SARADC};
use log::info;

const _XPD_WAIT_DEFAULT: u16 = 0xFF; /* Set wait cycle time(8MHz) from power up to reset enable. */
const ADC_FACTOR: f32 = 0.4386;
const DAC_FACTOR: f32 = 27.88;
const _SYS_OFFSET: f32 = 20.52;

enum DacOffset {
    L0 = 5,  /*< offset = -2, measure range: 50℃ ~ 125℃, error < 3℃. */
    L1 = 7,  /*< offset = -1, measure range: 20℃ ~ 100℃, error < 2℃. */
    L2 = 15, /*< offset =  0, measure range:-10℃ ~  80℃, error < 1℃. */
    L3 = 11, /*< offset =  1, measure range:-30℃ ~  50℃, error < 2℃. */
    L4 = 10, /*< offset =  2, measure range:-40℃ ~  20℃, error < 3℃. */
}

impl Default for DacOffset {
    fn default() -> Self {
        DacOffset::L2
    }
}

impl DacOffset {
    fn offset(&self) -> i8 {
        match self {
            DacOffset::L0 => -2,
            DacOffset::L1 => -1,
            DacOffset::L2 => 0,
            DacOffset::L3 => 1,
            DacOffset::L4 => 2,
        }
    }
}
struct SensorConfig {
    dac_offset: DacOffset,
    clock_divider: u8,
}

impl SensorConfig {
    fn new(dac_offset: DacOffset, clock_divider: u8) -> Self {
        Self {
            dac_offset,
            clock_divider,
        }
    }
}

impl Default for SensorConfig {
    fn default() -> Self {
        Self {
            clock_divider: 6,
            dac_offset: Default::default(),
        }
    }
}

pub struct BoardTempSensor {
    config: SensorConfig,
    efuse_calibration: f32,
}

impl BoardTempSensor {
    pub fn new(peripherals: &mut Peripherals) -> Self {
        // enable TSENS clock
        peripherals
            .SYSTEM
            .perip_clk_en1
            .modify(|_r, w| w.tsens_clk_en().set_bit());

        // select XTAL clock for TSENS:
        /*
        APB_SARADC_TSENS_CLK_SEL
        Choose working clock for temperature sensor. 0: FOSC_CLK. 1: XTAL_CLK. (R/W)
        */
        peripherals
            .APB_SARADC
            .tsens_ctrl2
            .modify(|_r, w| w.tsens_clk_sel().set_bit());

        // power up tsens
        peripherals
            .APB_SARADC
            .apb_tsens_ctrl
            .modify(|_r, w| w.tsens_pu().set_bit());

        // TODO conflicting information - on the one hand,
        /*
        esp_efuse_table.c
        static const esp_efuse_desc_t TEMP_CALIB[] = {
            {EFUSE_BLK2, 131, 9}, 	 // Temperature calibration data,
        };
        */
        // register is 128..160
        // we want 131 .. 131+9
        // -> offset 3, but python definition disagrees:

        // from esptool/blob/master/espressif/efuse/esp32c3/mem_definition.py:
        // # Name                      Category      Block Word Pos Type:len  WR_DIS RD_DIS Class         Description                Dictionary
        // ('TEMP_SENSOR_CAL',         "calibration",   2,  4, 7,   "uint:9",   21,   None, "t_sensor",   "Temperature calibration", None),
        //
        // a "word" is 32 bit it seems, so that's base = 32*4 + 7 = 135, not 131

        let register_contents = peripherals.EFUSE.rd_sys_part1_data4.read().bits();
        info!("raw data: {:b}", register_contents);

        // `as u8` truncates accordingly, otherwise we'd need e.g. & 0xff
        let efuse_calibration_raw = (register_contents >> 7) as u8;

        // TODO this is what the IDF C source does, but is it correct?
        // TODO why does `mem_definition.py` say the length is 9?
        let sign = if efuse_calibration_raw & 0b1000_0000 > 0 {
            -1.
        } else {
            1.
        };

        let efuse_calibration = efuse_calibration_raw as f32 / 10. * sign;

        info!("efuse calibration: {}", efuse_calibration);

        Self {
            config: Default::default(),
            efuse_calibration,
        }
    }

    pub fn read(&self, adc: &mut APB_SARADC) -> f32 {
        let register = adc.apb_tsens_ctrl.read();
        let raw_value = register.tsens_out().bits();
        let value = ADC_FACTOR * (raw_value as f32)
            - DAC_FACTOR * self.config.dac_offset.offset() as f32
            - self.efuse_calibration;

        value
    }
}
