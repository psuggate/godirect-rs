pub use constants::*;
pub use sensor::*;

pub mod backend;
pub mod bluetooth;
pub mod command;
pub mod common;
pub mod constants;
pub mod device;
pub mod sensor;
pub mod usb_device;

/**
 * The godirect module wraps the hidapi and bleak modules to create an easy way to
 * interact with Vernier GoDirect devices.
 */
pub struct GoDirect {
    pub ble_auto_connect_rssi_threshold: i8, // = -50 // closer to zero is a stronger signal
    pub ble_backend: BleBackend,
    pub usb_backend: UsbBackend,
}

impl GoDirect {
    /**
     * Construct a new 'GoDirect' object and initialize backends.
     */
    pub fn new() -> Self {
        pretty_env_logger::init(); // todo ...
        let ble_backend = BleBackend::new(BLE_AUTO_CONNECT_RSSI_THRESHOLD);
        ble_backend.scan().unwrap();

        Self {
            ble_backend,
            usb_backend: UsbBackend::new(),
        }
    }

    pub fn get_version() -> &'static str {
        "0.1.0"
    }

    pub fn list_devices() -> Result<Vec<DeviceId>, GoDirectError> {
        Ok(Vec::new())
    }

    pub fn get_device(device_id: DeviceId) -> Result<Device, GoDirectError> {
        Err(GoDirectError::NotFound)
    }
}
