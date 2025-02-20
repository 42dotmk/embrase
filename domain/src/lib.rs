pub trait Led {
    fn on(&mut self);
    fn off(&mut self);
    fn toggle(&mut self);
}

pub fn blink_led<L: Led>(led: &mut L, times: u32, delay: u64) {
    for _ in 0..times {
        led.on();

        std::thread::sleep(std::time::Duration::from_secs(delay));

        led.off();

        std::thread::sleep(std::time::Duration::from_secs(delay));
    }
}
