extern crate libusb;

mod device;
mod devices;

pub use device::{Device, G13_LCD_COLUMNS, G13_LCD_ROWS, G13_LCD_BUF_SIZE};
pub use devices::DeviceEntry;

pub struct Context {
    context: libusb::Context,
}

impl Context {
    pub fn new() -> Result<Context, libusb::Error> {
        let context = libusb::Context::new()?;

        Ok(Context { context })
    }

    pub fn get_devices<'b>(&self) -> Result<Vec<devices::DeviceEntry>, libusb::Error> {
        devices::get_devices(&self.context)
    }
}
