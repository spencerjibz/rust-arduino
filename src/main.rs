// main.rs

#![no_std]
#![no_main]
use panic_halt as _;
#[allow(unused_imports)]
use arduino_uno::hal::port::mode::Output;
#[allow(unused_imports)]
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;
use dht11::Dht11;
extern crate  ufmt;
extern crate nb;
use atmega328p_hal::{delay,clock};


#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();
     
    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);
    let mut delay = delay::Delay::<clock::MHz16>::new();
    let  sensor= pins.d2.into_tri_state (&mut pins.ddr);
    // initialize DHT data transfer
    
#[allow(unused_must_use)] 
    
  
    let mut serial = arduino_uno::Serial::new(
        peripherals.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );


  
    
   // create an instance of the sensor
   let mut dht11 = Dht11::new(sensor);
  
    
     ufmt::uwriteln!(&mut serial," Here the reading from sensors !\r").void_unwrap();
    
    loop {
           match dht11.perform_measurement(&mut delay) {
              Ok(readings) => { 
   // write result to serial 
   ufmt::uwriteln!(&mut serial, "{}", readings.temperature ).void_unwrap();
              },
              Err(_) => { 
           //   ufmt::uwriteln!(&mut serial,  "failed to read").void_unwrap();
              }
    
           }
           
           
           arduino_uno::delay_ms(1500);
         }
         
    }

   
/* fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}
*/