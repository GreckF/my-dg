mod dg;
mod control;

use btleplug::api::Peripheral;
use std::collections::BTreeSet;
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

// use futures::executor::block_on;
use dg::*;
use dg::types::*;
use control::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

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

  set_power_ab(&device, &bundle, 1000, 1000).await;

  loop { 
    if device.is_connected().await? {
      println!("1");

      set_power_ab(&device, &bundle, 1000, 700).await;

      set_wave_a(&device, &bundle, &from_frequency(800.0, 20)).await;
      set_wave_b(&device, &bundle, &from_frequency(800.0, 20)).await;

      time::sleep(Duration::from_millis(0)).await;
    } else {
      println!("Disconnected"); // todo: reconnect
    }
  }

  Ok(())
}


