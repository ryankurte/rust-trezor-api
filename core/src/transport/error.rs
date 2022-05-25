//! # Transport error types

/// Transport errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("HID error: {0}")]
	/// Error from hidapi.
	Hid(hidapi_rusb::HidError),

	#[error("USB error: {0}")]
	/// Error from ruusb.
	Usb(rusb::Error),

	#[error("IO error: {0}")]
	/// IO error
	Io(std::io::Error),
	
	#[error("The device to connect to was not found")]
	/// The device to connect to was not found.
	DeviceNotFound,
	
	#[error("The device is no longer available.")]
	/// The device is no longer available.
	DeviceDisconnected,

	#[error("The HID version supported by the device was unknown.")]
	/// The HID version supported by the device was unknown.
	UnknownHidVersion,

	#[error("The device produced a data chunk of unexpected size.")]
	/// The device produced a data chunk of unexpected size.
	UnexpectedChunkSizeFromDevice(usize),

	#[error("Timeout expired while reading from device.")]
	/// Timeout expired while reading from device.
	DeviceReadTimeout,

	#[error("The device sent a chunk with a wrong magic value.")]
	/// The device sent a chunk with a wrong magic value.
	DeviceBadMagic,

	#[error("The device sent a message with a wrong session id.")]
	/// The device sent a message with a wrong session id.
	DeviceBadSessionId,

	#[error("The device sent an unexpected sequence number.")]
	/// The device sent an unexpected sequence number.
	DeviceUnexpectedSequenceNumber,

	#[error("Received a non-existing message type from the device.")]
	/// Received a non-existing message type from the device.
	InvalidMessageType(u32),

	#[error("Unable to determine device serial number.")]
	/// Unable to determine device serial number.
	NoDeviceSerial,
}

impl From<hidapi_rusb::HidError> for Error {
	fn from(e: hidapi_rusb::HidError) -> Error {
		Error::Hid(e)
	}
}

impl From<rusb::Error> for Error {
	fn from(e: rusb::Error) -> Error {
		Error::Usb(e)
	}
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Error {
		Error::Io(e)
	}
}
