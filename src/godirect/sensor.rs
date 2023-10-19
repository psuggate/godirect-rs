/**
 * Go Direct sensor information, from 'Sensor.js'.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueType {
    Real64 = 0,
    Int32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeasMode {
    Periodic = 0,
    Aperiodic,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementInfo {
    pub meas_type: ValueType,
    pub meas_mode: MeasMode,
    pub min_value: f64,   // in sensor units (e.g., mV, Pa, ...)
    pub max_value: f64,   // in sensor units (e.g., mV, Pa, ...)
    pub uncertainty: f64, // in sensor units (e.g., mV, Pa, ...)
    pub min_period: u32,  // milliseconds
    pub max_period: u32,  // milliseconds
    pub typ_period: u32,  // milliseconds
    pub granularity: u32, // milliseconds
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    number: u8,
    name: String,
    unit: String,
    id: String,
    mutual_exclusion_mask: u32,
    measurement_info: MeasurementInfo,
}
