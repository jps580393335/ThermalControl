
use std::error::Error;
use std::thread;
use std::time;
use thread_priority::*;

use rppal::gpio::Gpio;
use rppal::gpio::Level::High;
use rppal::gpio::Level::Low;
use rppal::gpio::Mode::Input;
use rppal::gpio::Mode::Output;

fn main()-> Result<(), Box<dyn Error>> {
    let start_time = time::Duration::from_millis( 18 );
    //let bit_time   = time::Duration::from_micros( 1 );
    let wait_time  = time::Duration::from_micros( 5 );
    let gpio = Gpio::new()?;
    //let pin_in  = gpio.get(7)?.into_input();
    let mut pin = gpio.get(23)?.into_io( Output );
    let mut counter = 0 ;
    let mut bit0_counter = 0 ;
    let mut wait_counter = 0 ;
    let mut trace:  [ usize; 40] = [0; 40] ;
    let mut bit_position  u8 = 0b1000_0000 ; // on comence par le point fort
    let mut byte_value    u8 = 0 ;
    let mut humidity   float = 0.0 ;
    let mut temperatu  float = 0.0 ;

    // start impuls 18 µsec low 
    if !set_current_thread_priority(ThreadPriority::Max).is_ok(){ return Ok(()) ; }
    pin.set_mode( Output );
    pin.write( Low );
    thread::sleep( start_time );
    pin.write( High );
    thread::sleep( wait_time );
    pin.set_mode( Input );

    // on attend la presence (Low) du DHT11
    while pin.read()==High {
          wait_counter += 1;
          if  wait_counter > 5000 {
              wait_counter = 0;
              break; 
          }
    } 

    //on attend la start block (high)
     while pin.read()==Low {
          wait_counter += 1;
          if  wait_counter  > 5000 {
              wait_counter =0;
              break; 
          }
    }

    // on mesure le bit0_counter
    while pin.read()==High {
          bit0_counter += 1;
          if  bit0_counter > 5000 {
              break; 
          }
    }
    bit0_counter += 1 ; // pour evité des faut positif

    //on attend le premier bit
    while pin.read()==Low {
          wait_counter += 1;
          if  wait_counter > 5000 {
              wait_counter = 0;
              break; 
          }
    } 

    //  on lit le premier bit 
    loop{
         while pin.read()==High {
            wait_counter += 1;
            if  wait_counter > 5000 {
                break; 
            }
            //thread::sleep( bit_time );
         }
         trace[counter] = wait_counter;
         wait_counter = 0;
         counter += 1 ;
         if counter == 40 {
             break;
         }
 
         while pin.read()==Low {
            wait_counter += 1;
            if  wait_counter  > 5000 {
                wait_counter =0;
                break; 
            }
         }
    }
    if !set_current_thread_priority(ThreadPriority::Min).is_ok(){ return Ok(()) ; }
   
    for x in    trace.iter() {
        if x > &bit_counter {
            byte_value += bit_position
        }
      print!("{}, ", x > &bit0_counter );
    }

    println!( "|" );

    Ok(())    
}

