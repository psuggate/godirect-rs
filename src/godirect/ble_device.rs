use btleplug::{Adapter, Manager};
use std::time::Duration;

use super::backend::{BackendError, DeviceBackend};

pub const SERVICE: str = "d91714ef-28b9-4f91-ba16-f0d9a604f112";
pub const COMMAND_CHARACTERISTIC: str = "f4bf14a6-c7d5-4b6d-8aa8-df1a7c83adcb";
pub const RESPONSE_CHARACTERISTIC: str = "b41e6675-a329-40e0-aa01-44d2f444babe";

#[derive(Debug, Clone)]
pub struct BleBackend {
    manager: Manager,
    central: Adapter,
}

impl DeviceBacked for BleBackend {
    fn scan(&mut self) -> Result<Vec<u8>, BackendError> {
        Ok(Vec::new())
    }

    fn connect(&mut self, device: String) -> Result<(), BackendError> {
        Ok(())
    }

    fn try_read(&mut self, timeout: Duration) -> Result<Option<Vec<u8>>, BackendError> {
        Ok(None)
    }

    fn write(&mut self, bytes: &[u8]) -> Result<usize, BackendError> {
        Ok(0)
    }

    fn close(&mut self) {
        self.adapter.stop().unwrap();
    }
}

/*
export default class WebBluetoothDeviceAdapter {
  constructor(webBluetoothNativeDevice) {
    this.webBluetoothNativeDevice = webBluetoothNativeDevice;
    this.maxPacketLength = 20;

    this.deviceCommand = null;
    this.deviceResponse = null;
  }

  get godirectAdapter () {
    return true;
  }

  async writeCommand(commandBuffer) {
    return this.deviceCommand.writeValue(commandBuffer);
  }

  // Todo: bikeshed on name of this function
  async setup({ onClosed, onResponse }) {
    this.webBluetoothNativeDevice.addEventListener(
      'gattserverdisconnected', onClosed
    );

    try {
      const server = await this.webBluetoothNativeDevice.gatt.connect();
      const service = await server.getPrimaryService(SERVICE);
      const characteristics = await service.getCharacteristics();

      characteristics.forEach((characteristic) => {
        switch (characteristic.uuid) {
          case COMMAND_CHARACTERISTIC:
            this.deviceCommand = characteristic;
            break;
          case RESPONSE_CHARACTERISTIC:
            this.deviceResponse = characteristic;
            // Setup handler on the characteristic and start notifications.
            this.deviceResponse.addEventListener('characteristicvaluechanged', (event) => {
              const response = event.target.value;
              onResponse(response);
            });
            this.deviceResponse.startNotifications();
            break;
          default:
            log(`No case found for ${characteristic.uuid}`);
        }
      });
    } catch (err) {
      console.error(err);
    }

    if (!(this.deviceCommand && this.deviceResponse)) {
      throw new Error('Expected command and response characteristics not found');
    }
  }

  async close() {
    return this.webBluetoothNativeDevice.gatt.disconnect();
  }
}
*/
