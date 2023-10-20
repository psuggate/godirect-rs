use btleplug::{Adapter, Manager, Peripheral};
use std::time::Duration;

// use super::backend::{Backend, GoDirectError};
use super::Device::{Device, DeviceId};

/*
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
*/

#[derive(Debug, Clone)]
pub struct Backend {
    context: rusb::Context,
    manager: Option<Manager>,
    central: Option<Adapter>,
}

pub struct Munger {
    unmung: Option<Manager>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            context: rusb::Context::new(),
            manager: None,
            central: None,
        }
    }

    async fn auto_scan(&mut self, enable: bool) -> Result<(), GoDirectError> {
        let manager: Manager = Manager::new().await.unwrap();
        let central: Adapter = manager
            .adapters()
            .await
            .expect("Can not get a list of adapters.")
            .into_iter()
            .next()
            .expect("Unable to find adapters.");
        let mut events = central.events().await?;

        central.start_scan(ScanFilter::default()).await?;

        std::thread::spawn(move || {
            tokio::spawn(async move {
                scanner(events, device_sender).await?;
            });
        });

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

    fn close(&mut self) {
        self.adapter.stop().unwrap();
    }
}
