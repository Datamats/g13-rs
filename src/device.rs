use libusb;
use libusb::Device as UsbDevice;
use libusb::DeviceHandle;

use std::time::Duration;

pub const LIBUSB_REQUEST_TYPE_CLASS: u8 = 0x01 << 5;
pub const LIBUSB_RECIPIENT_INTERFACE: u8 = 0x01;

pub const G13_LCD_COLUMNS: usize = 160;
pub const G13_LCD_ROWS: usize = 48;
pub const G13_LCD_BUF_SIZE: usize = (G13_LCD_COLUMNS * G13_LCD_ROWS) / 8;

pub struct Device<'a> {
    handle: DeviceHandle<'a>,
    timeout: Duration,
}

impl<'a> Device<'a> {
    pub fn open(usb_device: &'a UsbDevice) -> Result<Device<'a>, libusb::Error> {
        let mut handle = usb_device.open()?;
        let timeout = Duration::from_secs(1);

        if handle.kernel_driver_active(0)? {
            handle.detach_kernel_driver(0)?;
        }

        handle.claim_interface(0)?;

        // Init LCD
        handle.write_control(0, 9, 1, 0, &[], timeout)?;

        Ok(Device { handle, timeout })
    }

    pub fn set_color(&self, r: u8, g: u8, b: u8) -> Result<(), libusb::Error> {
        let mut buf: [u8; 5] = [5, 0, 0, 0, 0];

        buf[1] = r;
        buf[2] = g;
        buf[3] = b;

        self.handle.write_control(
            LIBUSB_REQUEST_TYPE_CLASS | LIBUSB_RECIPIENT_INTERFACE,
            9,
            0x307,
            0,
            &buf,
            self.timeout,
        )?;

        Ok(())
    }

    pub fn write_lcd(&self, data: [u8; G13_LCD_BUF_SIZE]) -> Result<(), libusb::Error> {
        let mut buf = Vec::new();
        
        let mut header = [0; 32];
        header[0] = 0x03;    

        buf.extend_from_slice(&header);
        buf.extend_from_slice(&data);

        self.handle.write_interrupt(
            2,
            &buf,
            self.timeout,
        )?;

        Ok(())
    }
}
