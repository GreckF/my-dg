
use crate::dg::*;
use crate::dg::types::*;
use btleplug::api::{bleuuid::uuid_from_u16, CentralEvent, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use rand::{Rng, thread_rng};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

pub fn from_frequency(f : f64, z : i16) -> Pulse { 
  let x = f64::sqrt(f / 1000.0) * 15.0;
  let y = f - x;
  Pulse{ x : x as i16 , y : y as i16, z : z }
}

pub async fn set_wave<'a, 'b>
  ( device : &'a Peripheral
  , bundle : &UuidBundle<'b>
  , ch     : &Channel
  , p      : &Pulse)
->  Result<(), btleplug::Error> {
  match ch 
  { Channel::A => set_wave_a(device, bundle, p).await
  , Channel::B => set_wave_b(device, bundle, p).await }
}

