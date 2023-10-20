pub const SERVICE: str = "d91714ef-28b9-4f91-ba16-f0d9a604f112";
pub const COMMAND_CHARACTERISTIC: str = "f4bf14a6-c7d5-4b6d-8aa8-df1a7c83adcb";
pub const RESPONSE_CHARACTERISTIC: str = "b41e6675-a329-40e0-aa01-44d2f444babe";

pub const VENDOR_ID: u16 = 0x08f7;
pub const PRODUCT_ID: u16 = 0x0010;

pub const BLE_AUTO_CONNECT_RSSI_THRESHOLD: i8 = -50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GoDirectError {
    NotFound,
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeviceId {
    vendor_id: u16,
    product_id: u16,
}
