use btleplug::{Adapter, Manager, Peripheral};
use std::time::Duration;

// use super::backend::{Backend, GoDirectError};
use super::Device::{Device, DeviceId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GoDirectError {
    NotFound,
    Unknown(String),
}

pub trait Backend {
    fn scan(&mut self) -> Result<Vec<u8>, GoDirectError>;
    fn connect(&mut self, device_id: DeviceId)
        -> Result<Device, GoDirectError>;
    fn try_read(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Vec<u8>>, GoDirectError>;
    fn write(&mut self, bytes: &[u8]) -> Result<usize, GoDirectError>;
    fn close(&mut self);
}

pub const SERVICE: str = "d91714ef-28b9-4f91-ba16-f0d9a604f112";
pub const COMMAND_CHARACTERISTIC: str = "f4bf14a6-c7d5-4b6d-8aa8-df1a7c83adcb";
pub const RESPONSE_CHARACTERISTIC: str = "b41e6675-a329-40e0-aa01-44d2f444babe";

#[derive(Debug, Clone)]
pub struct BleBackend {
    manager: Manager,
    central: Adapter,
    device: Peripheral,
}

impl Backend for BleBackend {
    fn scan(&mut self, enable: bool) -> Result<(), GoDirectError> {
        Ok(())
    }

    fn list_devices(&mut self) -> Result<Vec<DeviceId>, GoDirectError> {
        Ok(Vec::new())
    }

    fn connect(&mut self, device: DeviceId) -> Result<Device, GoDirectError> {
        BleDevice::new(DeviceId)
    }

    fn is_connected(&mut self, device_id: DeviceId) -> bool {
        false
    }

    /*
    fn try_read(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Vec<u8>>, GoDirectError> {
        Ok(None)
    }

    fn write(&mut self, bytes: &[u8]) -> Result<usize, GoDirectError> {
        Ok(0)
    }
    */

    fn close(&mut self) {
        self.adapter.stop().unwrap();
    }
}
