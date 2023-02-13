use std::fmt::{Display, Formatter};
use btleplug::api::Characteristic;
use crate::control::from_frequency;
use std::str::FromStr;
pub const BATTERY_LEVEL_UUID : &str = "955a1500-0fe2-f5aa-a094-84b8d4f3e8ad";
pub const POWER_AB_UUID      : &str = "955a1504-0fe2-f5aa-a094-84b8d4f3e8ad";
pub const WAVE_B_UUID        : &str = "955a1505-0fe2-f5aa-a094-84b8d4f3e8ad";
pub const WAVE_A_UUID        : &str = "955a1506-0fe2-f5aa-a094-84b8d4f3e8ad";

pub const DEVICE_MARK: &str = "D-LAB";
const DEFAULT_Z : i16 = 15; // [0, 31]


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

#[derive(Debug, Clone)]
pub struct Pulse
{ pub x : i16
, pub y : i16
, pub z : i16 }

impl Pulse {
  pub fn new(x : i16, y : i16, z : i16) -> Pulse {
    Pulse { x: x, y: y, z: z }
  }

  pub fn zero() -> Pulse {
    Pulse { x: 0, y: 0, z: 0 }
  }
}

#[derive(Debug, Clone)]
pub struct PoweredPulse
{ pub pulse : Pulse
, pub power : i16 }

impl PoweredPulse {
    pub fn new(x : i16, y : i16, z : i16, power : i16) -> PoweredPulse {
      PoweredPulse { pulse: Pulse::new(x, y, z), power: power }
    }

    pub fn zero() -> PoweredPulse {
      PoweredPulse { pulse: Pulse { x: 0, y: 0, z: 0 }, power: 0 }
    }

    pub fn test() -> PoweredPulse {
      PoweredPulse { pulse: Pulse { x: 3, y: 37, z: 20 }, power: 1500 }
    }

}

impl Display for Pulse {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "pulse: ({}, {}, {})", self.x, self.y, self.z)
  }
}

impl Display for PoweredPulse {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "pulse: ({}, {}, {}, power : {})", self.pulse.x, self.pulse.y, self.pulse.z, self.power)
  }
}

fn parse_err_env(ls : &Vec<&str>) -> Result<PoweredPulse, <i16 as FromStr>::Err> {
  if ls.len() == 3 {
      
    Ok(PoweredPulse
      { pulse : from_frequency(ls[2].parse::<i16>()? as f64, DEFAULT_Z)
      , power : ls[1].parse::<i16>()? * 7 } 
    )
    
  } else if ls.len() == 4 {

    Ok(PoweredPulse
      { pulse : from_frequency(ls[2].parse::<i16>()? as f64, ls[3].parse()?)
      , power : ls[1].parse::<i16>()? * 7 }
    ) 
    
  } else {
    panic!()
  } 
}

impl FromStr for PoweredPulse {

  type Err = String;

  fn from_str(s : &str) -> Result<Self, <Self as FromStr>::Err> {

    let ls = s.split_whitespace().collect::<Vec<_>>();

    if ls.len() == 3 || ls.len() == 4 {
      
      match parse_err_env(&ls) {
        Ok(x) => Ok(x),
        Err(r) => Err("请输入整数".to_string())
      }

    } else {

      return Err(
        concat!
        ( "语法错误, 请按格式发送 \" #dg 强度 频率 \" 或者 \" #dg 强度 频率 脉冲宽度 \".\n\n"
        , "其中\n 强度 ∈ [0, 290] ∩ ℕ,\n 频率 ∈ [10, 1000] ∩ ℕ,\n 脉冲宽度 ∈ [0, 31] ∩ ℕ.\n\n"
        , "P.S. 当脉冲宽度大于 DEFAULT_Z 时更容易引起刺痛.")
        .to_string()
      );

    }
  }
}

#[derive(Debug)]
pub enum Channel { A, B }

fn change_channel(ch : &Channel) -> Channel {
  match ch 
  { Channel::A => Channel::B
  , Channel::B => Channel::A }
}