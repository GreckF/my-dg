mod control;
mod dg;

use btleplug::api::Peripheral;
use futures::channel::mpsc::{Receiver, Sender};
use std::collections::BTreeSet;
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

// use futures::executor::block_on;
use control::*;
use dg::types::*;
use dg::*;

mod bot;
use proc_qq::Authentication::{QRCode, UinPassword};
use proc_qq::ClientBuilder;
use proc_qq::DeviceSource::JsonFile;
use proc_qq::{re_exports::ricq_core::protocol::version::*, *};

use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};



use tokio;

mod bluetooth_test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //bluetooth_test::main().await;
    //
    //return Ok(());

    let (_sender, mut receiver) = tokio::sync::mpsc::channel::<(PoweredPulse, PoweredPulse)>(1);

    unsafe {
        bot::sender = Some(_sender);
    }

    // Bluetooth
    println!("\nRunning\n");

  tokio::spawn(async move {
    let device = get_device(3).await.ok_or("No Device Found!").unwrap();
    device.connect().await.unwrap();
    device.discover_services().await;
    let chars = device.characteristics();
    let bundle = get_bundle(&device, &chars).await.unwrap();
    println!("{}", bundle);
    let level = get_battery_level(&device, &bundle).await.unwrap();
    println!("battery: {}%", level);

    let PoweredPulse {
      pulse: mut apulse,
      power: mut ap,
    } = PoweredPulse::zero();
    let PoweredPulse {
      pulse: mut bpulse,
      power: mut bp,
    } = PoweredPulse::zero();
    loop {
      if device.is_connected().await.unwrap() {
        match receiver.try_recv() {
          Ok(msg) => {
            apulse = msg.0.pulse.clone();
            ap     = msg.0.power;
            bpulse = msg.1.pulse.clone();
            bp     = msg.1.power;
            println!("Bluetooth: set a : pulse = {}, power = {};\n b : pulse = {} power = {}", apulse, ap, bpulse, bp);
          }
          Err(tokio::sync::mpsc::error::TryRecvError::Empty) => { /* println!("nothing recived"); */
          }
          Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
            println!("SENDER DISCONNECT!");
          }
        }

        if ap != 0 || bp != 0 {
          set_power_ab(&device, &bundle, ap, bp).await.unwrap();
        //; println!("Bluetooth: set a : pulse = {}, power = {};\n b : pulse = {} power = {}", apulse, ap, bpulse, bp);
        }
        if ap != 0 {
          set_wave_a(&device, &bundle, &apulse).await.unwrap(); 
        }
        if bp != 0 {
          set_wave_b(&device, &bundle, &bpulse).await.unwrap();
        }
        tokio::time::sleep(Duration::from_millis(100));
      } else {
        println!("Disconnected"); // todo: reconnect
      }
    }
  });

    // Bot
    init_tracing_subscriber();
    ClientBuilder::new()
        .authentication(QRCode) //.authentication(Authentication::UinPassword(uin, pwd.to_string()))
        .device(JsonFile("device.json".into()))
        .version(&ANDROID_WATCH)
        .modules(vec![bot::module()])
        .build()
        .await
        .unwrap()
        .start()
        .await
        .unwrap()
        .unwrap();

    println!("end of main");
    Ok(())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time()
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("ricq", Level::DEBUG)
                .with_target("proc_qq", Level::DEBUG)
                // 这里改成自己的crate名称
                .with_target("qq_bot", Level::DEBUG),
        )
        .init();
}
