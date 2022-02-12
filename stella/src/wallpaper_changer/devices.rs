extern crate blurz;

use std::thread;
use std::time::Duration;

use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession;
use blurz::bluetooth_session::BluetoothSession;

use std::ffi::CString;
use alsa::hctl::HCtl;

#[derive(Debug, Clone, Copy)]
pub enum Earphone {
    Activated,
    Deactivated,
}

impl std::fmt::Display for Earphone {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

fn check_bluetooth_device() -> Result<bool, Box<dyn std::error::Error>> {

    let bt_session = &BluetoothSession::create_session(None)?;
    let adapter = BluetoothAdapter::init(bt_session)?;
    adapter.set_powered(true)?;
    
    let session = BluetoothDiscoverySession::create_session(&bt_session, adapter.get_id())?;
    thread::sleep(Duration::from_millis(200));
    session.start_discovery()?;
    thread::sleep(Duration::from_millis(800));
    let devices = adapter.get_device_list()?;
    
    // let devices_list: Vec<&str> = Vec::from([""]);
    let mut is_device_connected: bool = false; 
    for d in devices {
        let device = BluetoothDevice::new(bt_session, d.clone()); 
        if device.get_name()? == "AirPods" { 
            is_device_connected = device.is_connected()?;
        }
    }
    
    // always close the connection
    match session.stop_discovery() {
        Ok(_) => (),
        Err(_) => return Ok(false)
    };

    return Ok(is_device_connected)

}

fn check_audio_jack() -> Result<bool, alsa::Error> {

    for a in ::alsa::card::Iter::new().map(|x| x.unwrap()) {
        let h = HCtl::open(&CString::new(format!("hw:{}", a.get_index())).unwrap(), false)?;
        match h.load() {
            Ok(x) => (x),
            Err(x) => println!("Error: {}", x)
        };

        for b in h.elem_iter() {
            let id = b.get_id().unwrap();
            let name = id.get_name().unwrap();
            if name == "Headphone Jack" {
                // println!("Headphone {}", b.read().unwrap().get_boolean(0).unwrap());
                return Ok(b.read().unwrap().get_boolean(0).unwrap()) 
            }
        }
    }
    
    return Ok(false)

}

pub fn check_earphones() -> Result<Earphone, String>  {
    
    let is_jack_in = match check_audio_jack() {
        Ok(jack) => jack,
        Err(err) => {
            println!("Error: {}", err);
            false 
        }
    };

    // let is_ble_in = match check_bluetooth_device() {
    //     Ok(x) => x,
    //     Err(x) => {
    //         println!("Error: {}", x);
    //         false
    //     }
    // };

    if is_jack_in {
        return Ok(Earphone::Activated)
    } 

    // its a default
    return Ok(Earphone::Deactivated)
}
