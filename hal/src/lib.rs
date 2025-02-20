use domain::Led;

pub struct HalLed {
    state: bool,
}

impl HalLed {
    pub fn new() -> Self {
        Self { state: false }
    }
}

impl Led for HalLed {
    
    fn on(&mut self) {
        self.state = true;
        println!("HAL: LED is on");
    }

    fn off(&mut self) {
        self.state = false;
        println!("HAL: LED is off")
    }

    fn toggle(&mut self) {
        self.state = !self.state;
        println!("HAL: LED is {}", if self.state {"on"} else {"off"})
    }
}