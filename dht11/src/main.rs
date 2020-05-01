
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
    let mut pin               = gpio.get(23)?.into_io( Output );
    let mut counter           = 0 ;
    let mut bit0_counter      = 0 ;
    let mut wait_counter      = 0 ;
    let mut trace:  [ usize; 40] = [0; 40] ;
    let mut bit_position: usize  = 0b1000_0000 ; // on comence par le point fort
    let mut byte_value:   usize  = 0 ;
    let mut byte_counter:  usize  = 0 ;
    let mut humidity:     f32    = 0.0 ;
    let mut temperatur:   f32    = 0.0 ;
    let mut check_summe:  usize  = 0 ;

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
    bit0_counter += 8 ; // pour evité des faut positif

    //on attend le premier bit
    while pin.read()==Low {
          wait_counter += 1;
          if  wait_counter > 5000 {
              wait_counter = 0;
              break; 
          }
    } 

    //  on lit le premier bit 
    wait_counter = 0 ;
    loop{
         while pin.read()==High {
            wait_counter += 1;
            if  wait_counter > 5000 {
                break; 
            }
           // thread::sleep( bit_time );
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
        if x > &bit0_counter {
            byte_value += bit_position ;
            print!("1");
        }else{
            print!("0");
        }
        bit_position >>= 1 ;
        if bit_position == 0 {
            print!(" ");
            match byte_counter {
                0 =>  { humidity = byte_value as f32 ; check_summe += byte_value ; } 
                1 =>  { humidity += (byte_value as f32 ) / 100.0 ; check_summe += byte_value ; }
                2 =>  { temperatur = byte_value as f32 ; check_summe += byte_value ; } 
                3 =>  { temperatur += (byte_value as f32 ) / 100.0 ; check_summe += byte_value ; }
                4 =>  check_summe -= byte_value,
                _ =>  print!("dht11 send to long data")
            }
            byte_value = 0 ;
            bit_position = 0b1000_0000 ;
            byte_counter  += 1 ;
        }
        
    }
    print!("H = {}, T = {}, CS = {}\n", humidity, temperatur, check_summe);
    Ok(())    
}

