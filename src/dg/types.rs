use std::fmt::{Display, Formatter};
use btleplug::api::Characteristic;

pub const BATTERY_LEVEL_UUID : &str = "955a1500-0fe2-f5aa-a094-84b8d4f3e8ad";
pub const POWER_AB_UUID      : &str = "955a1504-0fe2-f5aa-a094-84b8d4f3e8ad";
pub const WAVE_B_UUID        : &str = "955a1505-0fe2-f5aa-a094-84b8d4f3e8ad";
pub const WAVE_A_UUID        : &str = "955a1506-0fe2-f5aa-a094-84b8d4f3e8ad";

pub const DEVICE_MARK: &str = "D-LAB";


#[derive(Debug)]
pub struct UuidBundle<'a>
{ pub battery_level : &'a Characteristic
, pub power_ab      : &'a Characteristic
, pub wave_a        : &'a Characteristic
, pub wave_b        : &'a Characteristic }

impl<'a> Display for UuidBundle<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    writeln!(f, "Useful Characteristics:")?;
    writeln!(f, "  battery level : {}", self.battery_level)?;
    writeln!(f, "  power ab      : {}", self.power_ab     )?;
    writeln!(f, "  wave a        : {}", self.wave_a       )?;
    writeln!(f, "  wave b        : {}", self.wave_b       )?;
    writeln!(f, "")
  }
}

#[derive(Debug)]
pub struct Pulse
{ pub x : i16
, pub y : i16
, pub z : i16 }

impl Display for Pulse {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    writeln!(f, "pulse: ({}, {}, {})", self.x, self.y, self.z)
  }
}

#[derive(Debug)]
pub enum Channel { A, B }

fn change_channel(ch : &Channel) -> Channel {
  match ch 
  { Channel::A => Channel::B
  , Channel::B => Channel::A }
}