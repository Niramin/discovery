#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut state = 1;


    loop {
        //Show light_it_all for 1000ms
        display.show(&mut timer, next_led(state), 30);
        // clear the display again
        display.clear();
        //timer.delay_ms(10_u32);
        state = get_next_state(state, 16);
    }
}

fn next_led(state:i32 ) -> [[u8;5];5]
{
    let mut light_it_all = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    match state {
        1=> light_it_all[0][0] =1,
        2=> light_it_all[0][1] =1,
        3=> light_it_all[0][2] =1,
        4=> light_it_all[0][3] =1,
        5=> light_it_all[0][4] =1,
        6=> light_it_all[1][4] =1,
        7=> light_it_all[2][4] =1,
        8=> light_it_all[3][4] =1,
        9=> light_it_all[4][4] =1,
        10=> light_it_all[4][3] =1,
        11=> light_it_all[4][2] =1,
        12=> light_it_all[4][1] =1,
        13=> light_it_all[4][0] =1,
        14=> light_it_all[3][0] =1,
        15=> light_it_all[2][0] =1,
        16=> light_it_all[1][0] =1,
        _ => light_it_all[1][0] =1,
        
        
    }
    light_it_all
}


fn get_next_state(current:i32,max:i32) ->i32
{
    let mut result = current +1;
    if result >max
    {
        result =1;
    }
    result
    
}