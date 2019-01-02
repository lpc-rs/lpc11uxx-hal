use cortex_m::peripheral::syst::SystClkSource;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use lpc11uxx::SYST;


pub struct Delay {
    syst: SYST,
    clock_speed_mhz: u32,
}

impl Delay {
    pub fn new(mut syst: SYST, clock_speed_hz: u32) -> Self {
        syst.set_clock_source(SystClkSource::Core);
        let clock_speed_mhz = (clock_speed_hz + 500_000) / 1_000_000;
        Delay { syst, clock_speed_mhz }
    }

    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        // TODO: may overflow
        self.delay_us(ms * 1_000);
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        // TODO: may overflow
        let rvr = us * self.clock_speed_mhz;

        // systick is a 24bit timer
        assert!(rvr < (1<<24));

        self.syst.set_reload(rvr);
        self.syst.clear_current();
        self.syst.enable_counter();

        while !self.syst.has_wrapped() {}
        self.syst.disable_counter();
    }
}
