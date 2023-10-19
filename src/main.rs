use btleplug::api::{
    bleuuid::BleUuid, Central, CentralEvent, Manager as _, ScanFilter,
};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use log::info;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    info!("Hello, world!");
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
