extern crate blurz;

use std::thread;
use std::time::Duration;

use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession;
use blurz::bluetooth_session::BluetoothSession;


fn list_ble_devices() -> Result<(), String>  {

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

        println!("Device ID: {}, Address: {:?} Rssi: {:?} Name: {:?}",
            device.get_id(),
            device.get_address(),
            device.get_rssi(),
            device.get_name()
            );

        println!("is connected: {:?}", device.is_connected()); 
        println!("");
    }
    
    session.stop_discovery().unwrap();
    Ok(())
}

fn main() {
   match list_ble_devices() {
       Ok(x) => (x),
       Err(x) => println!("[-] Error {}", x)
   }; 
}
