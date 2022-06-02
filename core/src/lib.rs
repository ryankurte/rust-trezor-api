//! # Trezor API library
//!
//! ## Connecting
//!
//! Use the public top-level methods `find_devices()` and `unique()` to find devices.  When using
//! `find_devices()`, a list of different available devices is returned.  To connect to one or more
//! of them, use their `connect()` method.
//!
//! ## Logging
//!
//! We use the log package interface, so any logger that supports log can be attached.
//! Please be aware that `trace` logging can contain sensitive data.
//!

use log::{debug};

mod transport;

pub mod client;
pub use client::Trezor;

pub mod error;
pub use error::{Error, Result};

pub use trezor_protos::{self as protos, TrezorMessage};

#[cfg(todo)]
pub use crate::client::{
	ButtonRequest, ButtonRequestType, EntropyRequest, Features, InputScriptType, InteractionType,
	PassphraseRequest, PinMatrixRequest, PinMatrixRequestType, Trezor, TrezorResponse, WordCount,
};

use std::fmt;

/// The different kind of Trezor device models.
#[derive(PartialEq, Eq, Clone, Debug, Copy, strum::Display)]
pub enum Model {
	#[strum(serialize = "Trezor 1")]
	Trezor1,
	#[strum(serialize = "Trezor 2")]
	Trezor2,
	#[strum(serialize = "Trezor 2 Bootloader")]
	Trezor2Bl,
	#[strum(serialize = "Emulator")]
	Emulator,
}

/// A device found by the `find_devices()` method.  It can be connected to using the `connect()`
/// method.
#[derive(Debug)]
pub struct AvailableDevice {
	pub model: Model,
	pub debug: bool,
	transport: transport::AvailableDeviceTransport,
}

impl fmt::Display for AvailableDevice {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} (transport: {}) (debug: {})", self.model, &self.transport, self.debug)
	}
}

impl AvailableDevice {
	/// Connect to the device.
	pub fn connect(self) -> Result<Trezor> {
		let transport = transport::connect(&self).map_err(Error::TransportConnect)?;
		Ok(Trezor::new_with_transport(self.model, transport))
	}
}

/// Search for all available devices.
/// Most devices will show up twice both either debugging enables or disabled.
///
/// Note: This will not show older devices that only support the HID interface.
/// To use those, please use [find_hid_device].
pub fn find_devices(debug: bool) -> Result<Vec<AvailableDevice>> {
	let mut devices = Vec::new();

	use crate::transport::webusb::WebUsbTransport;
	devices.extend(WebUsbTransport::find_devices(debug).map_err(Error::TransportConnect)?);

	use crate::transport::udp::EmulatorTransport;
	devices.extend(EmulatorTransport::find_devices(debug).map_err(Error::TransportConnect)?);

	Ok(devices)
}

/// Search for old HID devices. This should only be used for older devices that don't have the
/// firmware updated to version 1.7.0 yet. Trying to connect to a post-1.7.0 device will fail.
pub fn find_hid_devices() -> Result<Vec<AvailableDevice>> {
	use crate::transport::hid::HidTransport;
	HidTransport::find_devices(true).map_err(Error::TransportConnect)
}

/// Try to get a single device.  Optionally specify whether debug should be enabled or not.
/// Can error if there are multiple or no devices available.
/// For more fine-grained device selection, use `find_devices()`.
/// When using USB mode, the device will show up both with debug and without debug, so it's
/// necessary to specify the debug option in order to find a unique one.
pub fn unique(debug: bool) -> Result<Trezor> {
	let mut devices = find_devices(debug)?;
	match devices.len() {
		0 => Err(Error::NoDeviceFound),
		1 => Ok(devices.remove(0).connect()?),
		_ => {
			debug!("Trezor devices found: {:?}", devices);
			Err(Error::DeviceNotUnique)
		}
	}
}
