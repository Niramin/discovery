#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::blocking::serial;
use microbit::pac::ppi::chen;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use heapless::Vec;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

   
   // nb::block!(serial.write(b'X')).unwrap();
   let msg = "The quick brown fox jumps over the lazy dog.";
   /*
    for byte in msg
   {
    nb::block!(serial.write(*byte)).unwrap();
   }
    */

   let mut buffer : Vec<u8,32>  = Vec::new();
    loop {
        buffer.clear();

        loop {
            
            let byte = nb::block!(serial.read()).unwrap();

            if buffer.push(byte).is_err()
            {
                
                let errMsg = b"error buffer is full\r \n";
                for errByte in errMsg
                {
                    nb::block!(serial.write(*errByte)).unwrap();
                }
                break;
            }

            if byte == 13 {
                for bufferByte in &buffer{
                    nb::block!(serial.write(*bufferByte)).unwrap();
                }
                break;
                
            }
        }
        nb::block!(serial.flush()).unwrap();
    }
}


