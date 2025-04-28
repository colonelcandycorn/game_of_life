#![no_main]
#![no_std]

mod life;

use crate::pac::interrupt;

use life::*;
use cortex_m_rt::entry;
#[rustfmt::skip]
use microbit::{
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
        gpio::{self, Floating, Input},
        gpiote::{Gpiote, GpioteChannel},
    },
    pac,
};
use nanorand::{Pcg64, Rng};
use panic_halt as _;


use embedded_hal::delay::DelayNs;

use rtt_target::rtt_init_log;
use critical_section_lock_mut::LockMut;

const MILLISECONDS_PER_FRAME: u32 = 100;

static IS_A_PRESSED: LockMut<bool> = LockMut::new();
static IS_B_PRESSED: LockMut<bool> = LockMut::new();
static GPIO: LockMut<Gpiote> = LockMut::new();


fn init_board(rng: &mut Pcg64) -> [[u8; 5]; 5] {

    let mut randomized_board = [[0u8; 5]; 5];

    for row in &mut randomized_board {
        for cell in row {
            *cell = rng.generate_range(0..=1);
        }
    }

    randomized_board
}

fn complement_board(current_board_state: &mut [[u8; 5]; 5]) {
    for row in current_board_state {
        for cell in row {
            *cell = *cell ^ 1;
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_log!();

    let board = microbit::Board::take().unwrap();

    let gpiote = Gpiote::new(board.GPIOTE);

    let setup_channel = |channel: GpioteChannel, button: &gpio::Pin<Input<Floating>>| {
        channel
            .input_pin(button)
            .hi_to_lo()
            .enable_interrupt();
        channel.reset_events();
    };

    setup_channel(gpiote.channel0(), &board.buttons.button_a.degrade());
    setup_channel(gpiote.channel1(), &board.buttons.button_b.degrade());

    IS_A_PRESSED.init(false);
    IS_B_PRESSED.init(false);
    GPIO.init(gpiote);

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIOTE);
    }
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    
    let mut display = Display::new(board.display_pins);

    let mut timer0 = Timer::new(board.TIMER0);

    let mut hw_rng = HwRng::new(board.RNG);
    let mut rng = nanorand::Pcg64::new_seed(hw_rng.random_u64() as u128);

    let mut current_board_state = init_board(&mut rng);

    log::info!("Initialized board with: {:?}", current_board_state);

    let mut ignore_b_frames = 0;
    let mut frames_waited = 0;

    loop {
        let mut is_a_pressed = false;
        let mut is_b_pressed = false;
        IS_A_PRESSED.with_lock(|a| is_a_pressed = *a);
        IS_B_PRESSED.with_lock(|b| {
            is_b_pressed = *b;
            *b = false;
        });

        if is_a_pressed {
            log::info!("Reinitializing the board");
            current_board_state = init_board(&mut rng);
        } else if is_b_pressed && ignore_b_frames == 0 {
            log::info!("Flipping the board");
            complement_board(&mut current_board_state);
            ignore_b_frames = 5;
        } else if done(&current_board_state) {
            log::info!("Board has completed");
            frames_waited += 1;
            if frames_waited >= 5 {
                log::info!("Board is starting back up");
                current_board_state = init_board(&mut rng);
                frames_waited = 0;
            }
        } else {
            frames_waited = 0;
            life(&mut current_board_state);
        }

        if ignore_b_frames > 0 {
            ignore_b_frames -= 1;
        }

        display.show(&mut timer0, current_board_state, 1000);
        display.clear();
        timer0.delay_ms(MILLISECONDS_PER_FRAME);
    }
}


#[cortex_m_rt::interrupt]
fn GPIOTE() {
    GPIO.with_lock(|gpiote| {
        log::info!("Inside interrupt");

        let button_a_pressed = gpiote.channel0().is_event_triggered();
        let button_b_pressed = gpiote.channel1().is_event_triggered();

        IS_A_PRESSED.with_lock(|a| *a = button_a_pressed);

        if button_b_pressed {
            IS_B_PRESSED.with_lock(|b| *b = true);
        }

        gpiote.channel0().reset_events();
        gpiote.channel1().reset_events();
    });
}