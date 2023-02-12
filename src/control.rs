use crate::dg::types::Pulse;


pub fn from_frequency(f : f64, z : i16) -> Pulse { 
  let x = f64::sqrt(f / 1000.0) * 15.0;
  let y = f - x;
  Pulse{ x : x as i16 , y : y as i16, z : z }
}

