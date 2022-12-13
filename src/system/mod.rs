mod stack;
pub use stack::*;

mod device;
pub use device::*;

mod opcodes;
pub use opcodes::*;

pub fn system_devices() -> (Device, Device) {
	(Device::new(), Device::new())
}
pub fn none_devices() -> (Device, Device) {
	(Device::new(), Device::new())
}
pub fn file_devices() -> (Device, Device) {
	(Device::new(), Device::new())
}