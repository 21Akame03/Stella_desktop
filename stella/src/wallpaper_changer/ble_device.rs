extern crate blurz;

use std::thread;
use std::time::Duration;

use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession;
use blurz::bluetooth_session::BluetoothSession;

#[derive(Debug, Clone, Copy)]
pub enum BleEarphone {
    Activated,
    Deactivated,
}

pub fn check_earphones() -> Result<BleEarphone, String>  {

    let bt_session = &BluetoothSession::create_session(None).unwrap();
    let adapter = BluetoothAdapter::init(bt_session).unwrap();
    adapter.set_powered(true).unwrap();
    
    let session = BluetoothDiscoverySession::create_session(&bt_session, adapter.get_id()).unwrap();
    thread::sleep(Duration::from_millis(200));
    session.start_discovery().unwrap();
    thread::sleep(Duration::from_millis(800));
    let devices = adapter.get_device_list().unwrap();
    
    println!("number of ble devices: {}", devices.len());

    for d in devices {
        let device = BluetoothDevice::new(bt_session, d.clone());
        
        if device.get_name().unwrap() == "Airpods" {
            match device.is_connected().unwrap() {
                true => return Ok(BleEarphone::Activated),
                false => return Ok(BleEarphone::Deactivated)
            }
        }

    }
    
    session.stop_discovery().unwrap();

    // its a default
    return Ok(BleEarphone::Deactivated)
}
