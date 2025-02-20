use domain::blink_led;
use hal::HalLed;

pub struct PalLed {
    hal_led: HalLed,
}

impl PalLed {
    pub fn new() -> Self {
        Self {
            hal_led: HalLed::new(),
        }
    }

    pub fn blink(&mut self, times: u32, delay: u64) {
        blink_led(&mut self.hal_led, times, delay);
    }
}
