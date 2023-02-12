pub mod types;

use btleplug::api::Characteristic;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::collections::BTreeSet;
use std::error::Error;
use tokio::time;
use uuid::Uuid;
use tokio::time::Duration;
pub use types::*;


pub async fn get_device(secs_of_wait : u8) -> Option<Peripheral> {

  let manager = Manager::new().await.unwrap();
  let adapters = manager.adapters().await.ok()?;

  let central = match adapters.into_iter().nth(0) 
  { None => { println!("No Bluetooth Adapter."); panic!() }
  , Some(x) => x };

  central.start_scan(ScanFilter::default()).await.ok()?;
  time::sleep(Duration::from_secs(secs_of_wait.into())).await;

  find_device(&central).await
}

// The device must be connected.
pub async fn get_bundle<'a,'b>
  ( device : &'a Peripheral
  , chars  : &'b BTreeSet<Characteristic>) 
->  Option<UuidBundle<'b>> {
  
  let battery_level = Uuid::parse_str(BATTERY_LEVEL_UUID).ok()?;
  let power_ab      = Uuid::parse_str(POWER_AB_UUID     ).ok()?;
  let wave_a        = Uuid::parse_str(WAVE_A_UUID       ).ok()?;
  let wave_b        = Uuid::parse_str(WAVE_B_UUID       ).ok()?;

  println!("All Characteristics:");
  for e in chars.iter() {
    println!("  {}", e);
  } println!("");

  Some(UuidBundle
  { battery_level : chars.iter().find(|c| c.uuid == battery_level).unwrap()
  , power_ab      : chars.iter().find(|c| c.uuid == power_ab     ).unwrap()
  , wave_a        : chars.iter().find(|c| c.uuid == wave_a       ).unwrap()
  , wave_b        : chars.iter().find(|c| c.uuid == wave_b       ).unwrap() })
}

pub async fn find_device(central: &Adapter) -> Option<Peripheral> {

  for p in central.peripherals().await.unwrap() {

    match p.properties().await.unwrap().unwrap().local_name 
    { None => { println!("No Device"); panic!() }
    , Some(s) => println!("{s}") }
    println!("");
    
    if p.properties()
      .await
      .unwrap()
      .unwrap()
      .local_name
      .iter()
      .any(|name| name.contains(DEVICE_MARK))
    {
      println!("Device Detected");
      return Some(p);
    }
  }
  None
}

// async fn get_battery_level(device: Peripheral, battery_char :&Characteristic) -> Option<u8> {

//   let level = device.read(battery_char).await.ok()?;

//   println!("battery: {}", level[0]);

//   Some(level[0])

// }

pub async fn get_battery_level<'a, 'b>(device: &'a Peripheral, bundle : &UuidBundle<'b>) -> Result<u8, btleplug::Error> {

  let level = device.read(bundle.battery_level).await?;
  Ok(level[0])

}

pub async fn set_power_ab<'a, 'b>
  (device : &'a Peripheral, bundle : &UuidBundle<'b>, a : i16, b : i16) 
-> Result<(), btleplug::Error> {
  /*
    00AAAAAA AAAAABBB BBBBBBBB
    BBBBBBBB AAAAABBB 00AAAAAA
  */
  let mut pa : i16 = a;
  let mut pb : i16 = b;
  
  if a != 0 { pa = a + 63; }
  if b != 0 { pb = b + 63; }

  let bytes : Vec<u8> = vec!
  [ (pb % 256)                     as u8
  , ((pb >> 8) + ((pa % 32) << 3)) as u8
  , (pa >> 5)                      as u8 ];

  device.write(bundle.power_ab, &bytes, WriteType::WithoutResponse).await
}
/*
    public void setWaveA(int x, int y, int z) {
        /**
         *         [00000000,00000000,00000000]
         *         [0000ZZZZ,ZYYYYYYY,YYYXXXXX]
         */
        byte[] bytes = new byte[3];
        bytes[0] = (byte) (z >> 1)
        bytes[1] = (byte) ((z & 1) + (y >> 3))
        bytes[2] = (byte) (((y % 8) << 5) + (x % 32))
        BluetoothGattCharacteristic characteristic = characteristicList.get(10);
        characteristic.setValue(bytes);
        boolean b = bluetoothGatt.writeCharacteristic(characteristic);
        Log.i("setWaveA", b + "");


    }
*/

pub async fn set_wave_a<'a, 'b>
  (device : &'a Peripheral, bundle : &UuidBundle<'b>, p : &Pulse)
-> Result<(), btleplug::Error> {

  let bytes : Vec<u8> = vec!
  [ (p.z >> 1)                      as u8
  , ((p.z & 1) + (p.y >> 3))        as u8
  , (((p.y % 8) << 5) + (p.x % 32)) as u8 ];
  
  device.write(bundle.wave_a, &bytes, WriteType::WithoutResponse).await
}

pub async fn set_wave_b<'a, 'b>
  (device : &'a Peripheral, bundle : &UuidBundle<'b>, p : &Pulse)
-> Result<(), btleplug::Error> {

  let bytes : Vec<u8> = vec!
  [ (p.z >> 1)                      as u8
  , ((p.z & 1) + (p.y >> 3))        as u8
  , (((p.y % 8) << 5) + (p.x % 32)) as u8 ];
  
  device.write(bundle.wave_b, &bytes, WriteType::WithoutResponse).await
}
