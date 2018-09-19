use libusb;
use libusb::Device as UsbDevice;

pub use super::device::Device;

const G13_VENDOR_ID: u16 = 0x046d;
const G13_PRODUCT_ID: u16 = 0xc21c;

pub struct DeviceEntry<'a> {
    bus_number: u8,
    address: u8,
    device: UsbDevice<'a>,
}

impl<'a> DeviceEntry<'a> {
    pub fn open(&self) -> Result<Device, libusb::Error> {
        Ok(Device::open(&self.device)?)
    }

    pub fn bus_number(&self) -> u8 {
        self.bus_number
    }

    pub fn address(&self) -> u8 {
        self.address
    }
}

pub fn get_devices<'a>(
    context: &'a libusb::Context,
) -> Result<Vec<DeviceEntry<'a>>, libusb::Error> {
    let mut devices = Vec::new();

    for device in context.devices()?.iter() {
        let desc = device.device_descriptor()?;

        if desc.vendor_id() == G13_VENDOR_ID && desc.product_id() == G13_PRODUCT_ID {
            devices.push(DeviceEntry {
                bus_number: device.bus_number(),
                address: device.address(),
                device,
            });
        }
    }

    Ok(devices)
}
