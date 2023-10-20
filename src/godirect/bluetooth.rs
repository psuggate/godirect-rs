//
// Wrapper around the try-hard, async/await nonsense used by the 'btleplug'
// crate.
//
// TODO:
//  - place all of the 'async' nonsense into its own thread, and talk to it
//    via queues;
//
use btleplug::{Adapter, CentralEvent, Manager, Peripheral};
use serde::{Deserialize, Serialize};
use std::{error::Error, time::Duration};

use super::common::*;

pub struct Bluetooth {
    commands: Sender<BleCommand>,
    response: Receiver<BleResponse>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum BleCommand {
    Scan,
    Stop,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum BleResponse {
    Ack,
    Nack,
}

impl Bluetooth {
    pub fn new() {
        let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel::<BleCommand>();
        let (res_tx, res_rx) = std::sync::mpsc::channel::<BleResponse>();

        // Spawn the nonsense off into its own OS thread, and communicate via
        // channels.
        std::thread::spawn(move || {
            tokio::spawn(async move {
                let nonsense = Nonsense::new(cmd_rx, res_tx).await?;
                scanner(events).await?;
            });
        });
    }
}

pub struct Nonsense {
    manager: Manager,
    central: Adapter,
    commands: tokio::sync::mpsc::Receiver<BleCommand>,
    response: std::sync::mpsc::Sender<BleResponse>,
}

impl Nonsense {
    #[tokio::main]
    pub async fn new(
        commands: tokio::sync::mpsc::Receiver<BleCommand>,
        response: std::sync::mpsc::Sender<BleRespnse>,
    ) -> Self {
        let manager: Manager = Manager::new().await.unwrap();
        let central: Adapter = manager
            .adapters()
            .await
            .expect("Can not get a list of adapters.")
            .into_iter()
            .next()
            .expect("Unable to find adapters.");

        Self {
            manager,
            central,
            commands,
            response,
        }
    }
}

async fn scanner(
    events: std::pin::Pin<Box<dyn Stream<Item = CentralEvent> + Send>>,
) -> Result<(), Box<dyn Error>> {
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

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                println!("DeviceDiscovered: {:?}", id);
            }
            CentralEvent::DeviceConnected(id) => {
                println!("DeviceConnected: {:?}", id);
            }
            CentralEvent::DeviceDisconnected(id) => {
                println!("DeviceDisconnected: {:?}", id);
            }
            CentralEvent::ManufacturerDataAdvertisement {
                id,
                manufacturer_data,
            } => {
                println!(
                    "ManufacturerDataAdvertisement: {:?}, {:?}",
                    id, manufacturer_data
                );
            }
            CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                println!(
                    "ServiceDataAdvertisement: {:?}, {:?}",
                    id, service_data
                );
            }
            CentralEvent::ServicesAdvertisement { id, services } => {
                let services: Vec<String> =
                    services.into_iter().map(|s| s.to_short_string()).collect();
                println!("ServicesAdvertisement: {:?}, {:?}", id, services);
            }
            _ => {}
        }
    }
    Ok(())
}
