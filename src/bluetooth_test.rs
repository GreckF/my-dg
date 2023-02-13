use btleplug::api::Peripheral;
use futures::channel::mpsc::{Sender, Receiver};
use std::collections::BTreeSet;
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;


// use futures::executor::block_on;
use crate::dg::*;
use crate::dg::types::*;
use crate::control::*;


use proc_qq::ClientBuilder;
use proc_qq::DeviceSource::JsonFile;
use proc_qq::{re_exports::ricq_core::protocol::version::*, *};
use proc_qq::Authentication::{QRCode, UinPassword};

use tracing::Level;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/////////////////////////////////////////////////////////////////////////////////////////////
pub async fn main() -> Result<(), Box<dyn Error>> {

  // println!("{}", (2339090) as u8);

  println!("\nRunning\n");

  let device = get_device(3).await.ok_or("No Device Found!")?;
  
  device.connect().await?;
  device.discover_services().await;

  let chars = device.characteristics();

  let bundle = get_bundle(&device, &chars).await.unwrap();

  println!("{}", bundle);

  let level = get_battery_level(&device, &bundle).await?;
  println!("battery: {}%", level);
  for i in 0..1000{
    set_power_ab(&device, &bundle, 1000, 1000).await;
    set_wave_a(&device, &bundle, &from_frequency(800.0, 20)).await.unwrap();
    set_wave_b(&device, &bundle, &from_frequency(800.0, 20)).await.unwrap();
    print!("{i}");
    time::sleep(Duration::from_millis(10)).await;
  }
  
  for i in 0..1000{
    set_power_ab(&device, &bundle, 0, 1000).await;
    set_wave_a(&device, &bundle, &from_frequency(800.0, 20)).await.unwrap();
    set_wave_b(&device, &bundle, &from_frequency(800.0, 20)).await.unwrap();
    print!("{i}");
    time::sleep(Duration::from_millis(10)).await;
  }
  loop { 
    if device.is_connected().await? {
      println!("1");

      // set_power_ab(&device, &bundle, 1000, 700).await.unwrap();
      // 
      // set_wave_a(&device, &bundle, &from_frequency(800.0, 20)).await.unwrap();
      // set_wave_b(&device, &bundle, &from_frequency(800.0, 20)).await.unwrap();

      // time::sleep(Duration::from_millis(0)).await;
    } else {
      println!("Disconnected"); // todo: reconnect
    }
  }

  Ok(())
}
/////////////////////////////////////////////////////////////////////////////////////////////


// use tokio;


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {

//   let (_sender,mut receiver) 
//     = tokio::sync::mpsc::channel::<(PoweredPulse, PoweredPulse)>(1);
    
//   unsafe {
//     bot::sender = Some(_sender);
//   }

//   // Bluetooth
//   println!("\nRunning\n");

//   // let device = get_device(3).await.ok_or("No Device Found!")?;
  
//   // device.connect().await?;
//   // device.discover_services().await;

//   // let chars = device.characteristics();

//   // let bundle = get_bundle(&device, &chars).await.unwrap();

//   // println!("{}", bundle);

//   // let level = get_battery_level(&device, &bundle).await?;
//   // println!("battery: {}%", level);

//   // Bluetooth thread
//   tokio::spawn(async move 
//   { let device = get_device(3).await.ok_or("No Device Found!").unwrap()
  
//   ; device.connect().await.unwrap()
//   ; device.discover_services().await
  
//   ; let chars = device.characteristics()
//   ; let bundle = get_bundle(&device, &chars).await.unwrap()
//   ; println!("{}", bundle)

//   ; let level = get_battery_level(&device, &bundle).await.unwrap()
//   ; println!("battery: {}%", level)

//   ; loop 
//     { if device.is_connected().await.unwrap()
//       { let PoweredPulse { pulse : mut apulse, power : mut ap } = PoweredPulse::zero();
//         let PoweredPulse { pulse : mut bpulse, power : mut bp } = PoweredPulse::zero();        
//       ; match receiver.try_recv() 
//         { Ok(msg) => 
//           { (PoweredPulse{pulse:apulse, power:ap}, PoweredPulse{pulse:bpulse, power:bp}) = msg
//           ; println!("Bluetooth: set a : pulse = {}, power = {};\n b : pulse = {} power = {}", apulse, ap, bpulse, bp);
//           }
//         , Err(tokio::sync::mpsc::error::TryRecvError::Empty) => { /* println!("nothing recived"); */ }
//         , Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => { println!("SENDER DISCONNECT!"); }
//         }
      
//       ; set_power_ab(&device, &bundle, ap, bp).await
      
//       ; set_wave_b(  &device, &bundle, &bpulse).await
//       ; set_wave_a(  &device, &bundle, &apulse).await
//       ; //tokio::time::sleep(Duration::from_millis(1));
//       } else 
//       { println!("Disconnected"); // todo: reconnect
//       }
//     }
//   });


  

//   // Bot
//   init_tracing_subscriber();
//   ClientBuilder::new()
//     .authentication(QRCode) //.authentication(Authentication::UinPassword(uin, pwd.to_string()))
//     .device(JsonFile("device.json".into()))
//     .version(&ANDROID_WATCH)
//     .modules(vec![bot::module()])
//     .build()
//     .await
//     .unwrap()
//     .start()
//     .await
//     .unwrap()
//     .unwrap();

//   println!("end of main");
//   Ok(())
// }

// fn init_tracing_subscriber() {
//   tracing_subscriber::registry()
//     .with(
//       tracing_subscriber::fmt::layer()
//         .with_target(true)
//         .without_time()
//         .with_line_number(true),
//     )
//     .with(
//       tracing_subscriber::filter::Targets::new()
//         .with_target("ricq", Level::DEBUG)
//         .with_target("proc_qq", Level::DEBUG)
//         // 这里改成自己的crate名称
//         .with_target("qq_bot", Level::DEBUG),
//     )
//     .init();
// }
