extern crate g13_rs;

use g13_rs::Context;

use std::time::SystemTime;

fn main() {
    let context = Context::new().expect("context");

    let devices = context.get_devices().expect("devices");

    let usb_device = devices.first().expect("usb_device");

    let device = usb_device.open().expect("device");

    let start = SystemTime::now();

    loop {
        let elapsed = start.elapsed().expect("duration");
        let time = (((elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64)
            as f32)
            / 1000.0;
        let (r, g, b) = hsv_to_rgb(time % 360.0, 1.0, 1.0);
        device.set_color(r, g, b).expect("color");
    }
}


fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    match (i as u8) % 6 {
        0 => {
            r = v;
            g = t;
            b = p;
        }
        1 => {
            r = q;
            g = v;
            b = p;
        }
        2 => {
            r = p;
            g = v;
            b = t;
        }
        3 => {
            r = p;
            g = q;
            b = v;
        }
        4 => {
            r = t;
            g = p;
            b = v;
        }
        5 => {
            r = v;
            g = p;
            b = q;
        }
        _ => {}
    }

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}