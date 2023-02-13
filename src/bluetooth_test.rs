use btleplug::api::Peripheral;
use futures::channel::mpsc::{Sender, Receiver};
use std::collections::BTreeSet;
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;


use crate::dg::*;
use crate::dg::types::*;
use crate::control::*;


use proc_qq::ClientBuilder;
use proc_qq::DeviceSource::JsonFile;
use proc_qq::{re_exports::ricq_core::protocol::version::*, *};
use proc_qq::Authentication::{QRCode, UinPassword};

use tracing::Level;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn main() -> Result<(), Box<dyn Error>> {

  println!("{}", "400\n".split_whitespace().collect::<Vec<_>>()[0].parse::<i16>().unwrap());

  println!("\nRunning\n");

  let device = get_device(3).await.ok_or("No Device Found!")?;
  
  device.connect().await?;
  device.discover_services().await;

  let chars = device.characteristics();

  let bundle = get_bundle(&device, &chars).await.unwrap();

  println!("{}", bundle);

  let level = get_battery_level(&device, &bundle).await?;
  println!("battery: {}%", level);
  
  loop { 
    if device.is_connected().await? {
      println!("1");

      let mut line = String::new();

      std::io::stdin().read_line(&mut line).unwrap();

      let v 
        = line
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<i16>().unwrap())
        .collect::<Vec<_>>();

      set_power_ab(&device, &bundle, v[0], 700).await.unwrap();
      
      set_wave_a(&device, &bundle, &Pulse { x: v[1], y: v[2], z: v[3] }).await.unwrap();
      set_wave_b(&device, &bundle, &Pulse { x: 3, y: 20, z: 20 }).await.unwrap();

      time::sleep(Duration::from_millis(0)).await;
    } else {
      println!("Disconnected"); // todo: reconnect
    }
  }

  Ok(())
}
