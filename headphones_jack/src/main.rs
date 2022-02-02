fn main() {
    for a in ::alsa::card::Iter::new().map(|x| x.unwrap()) {
        use std::ffi::CString;
        use alsa::hctl::HCtl;
        let h = HCtl::open(&CString::new(format!("hw:{}", a.get_index())).unwrap(), false).unwrap();
        h.load().unwrap();
        for b in h.elem_iter() {
            use alsa::ctl::ElemIface;
            let id = b.get_id().unwrap();
            if id.get_interface() != ElemIface::Card { continue; }
            let name = id.get_name().unwrap();
            if !name.ends_with(" Jack") { continue; }
            if name.ends_with(" Phantom Jack") {
                println!("{} is always present", &name[..name.len()-13])
            }
            else { println!("{} is {}", &name[..name.len()-5],
                if b.read().unwrap().get_boolean(0).unwrap() { "plugged in" } else { "unplugged" })
            }
        }
    }
}
