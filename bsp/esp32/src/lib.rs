use pal::PalLed;

const TIMES: u32 = 3;
const SECONDS: u64 = 2;

pub struct Esp32Board {
    pub led: PalLed,
}

impl Esp32Board {
    pub fn new() -> Self {
        Self { led: PalLed::new() }
    }

    pub fn run(&mut self) {
        self.led.blink(TIMES, SECONDS);
    }
}
