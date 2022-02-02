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

fn check_bluetooth_device() -> bool {

    let bt_session = &BluetoothSession::create_session(None).unwrap();
    let adapter = BluetoothAdapter::init(bt_session).unwrap();
    adapter.set_powered(true).unwrap();
    
    let session = BluetoothDiscoverySession::create_session(&bt_session, adapter.get_id()).unwrap();
    thread::sleep(Duration::from_millis(200));
    session.start_discovery().unwrap();
    thread::sleep(Duration::from_millis(800));
    let devices = adapter.get_device_list().unwrap();
    
 
    for d in devices {
        let device = BluetoothDevice::new(bt_session, d.clone()); 
        if device.get_name().unwrap() == "AirPods" { 
            return device.is_connected().unwrap()
        }
    }

    return false

}

fn check_audio_jack() -> bool {

    for a in ::alsa::card::Iter::new().map(|x| x.unwrap()) {
        let h = HCtl::open(&CString::new(format!("hw:{}", a.get_index())).unwrap(), false).unwrap();
        match h.load() {
            Ok(x) => (x),
            Err(x) => println!("Error: {}", x)
        };

        for b in h.elem_iter() {
            let id = b.get_id().unwrap();
            let name = id.get_name().unwrap();
            if name == "Headphone Jack" {
                // println!("Headphone {}", b.read().unwrap().get_boolean(0).unwrap());
                return b.read().unwrap().get_boolean(0).unwrap(); 
            }
        }
    }

    return false

}

pub fn check_earphones() -> Result<Earphone, String>  {
    
    if check_audio_jack() || check_bluetooth_device() {
        return Ok(Earphone::Activated)
    } 

    // its a default
    return Ok(Earphone::Deactivated)
}
