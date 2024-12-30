// P0

use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::{
    board::Edge,
    hal::{gpiote::Gpiote, timer::Instance, Timer},
    pac::{self, interrupt}
};
// use std::sync::LazyLock;

static EDGE: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
pub struct Echo {
    distance: f32,
    at_high: bool,
    inited: bool,
    timer0: pac::TIMER0,
}

impl Echo
// where T: Instance
{
    pub fn init(&mut self, board_gpiote: pac::GPIOTE, hcsr04_echo: Edge, timer: pac::TIMER0) {
        let gpiote = Gpiote::new(board_gpiote);

        let channel0 = gpiote.channel0();
        channel0
            .input_pin(&hcsr04_echo.e00.into_pulldown_input().degrade())
            .toggle()
            .enable_interrupt();
        channel0.reset_events();

        free(move |cs| {
            *EDGE.borrow(cs).borrow_mut() = Some(gpiote);

            unsafe {
                pac::NVIC::unmask(pac::Interrupt::GPIOTE);
            }
            pac::NVIC::unpend(pac::Interrupt::GPIOTE);
        });

        self.inited = true;
        self.at_high = false;
        self.distance = 0.0;
        self.timer0 = timer;
    }
}

static mut DISTANCE: Echo = Echo {
    distance: 0.0,
    at_high: false,
    inited: false,
    timer0: unsafe {
        pac::Peripherals::steal().TIMER0
    },
};

#[pac::interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(gpiote) = EDGE.borrow(cs).borrow().as_ref() {
            let toggled = gpiote.channel0().is_event_triggered();
            let timer = microbit::hal::timer::Timer::new(DISTANCE.timer0);
            if toggled {
                if DISTANCE.at_high {
                    // Start timer

                } else {
                    // Stop timer
                    // let timer = gpiote.channel0().timer();
                    if let Some(duration) = timer.read() {
                        // Calculate distance
                        let distance = duration as f32 * 0.034 / 2.0;
                        if distance < 10.0 {
                            // Do something
                        }
                    }
                }
            }


            gpiote.channel0().reset_events();

        }
    });
}