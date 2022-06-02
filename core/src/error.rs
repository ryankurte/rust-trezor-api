//! # Error Handling

use crate::protos;
use crate::transport;

/// Trezor error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Less than one device was plugged in.
	#[error("Less than one device was plugged in.")]
	NoDeviceFound,

	/// More than one device was plugged in.
	#[error("More than one device was plugged in.")]
	DeviceNotUnique,

	/// Transport error connecting to device.
	#[error("Transport error connecting to device.")]
	TransportConnect(transport::error::Error),

	/// Transport error while beginning a session.
	#[error("Transport error while beginning a session.")]
	TransportBeginSession(transport::error::Error),

	/// Transport error while ending a session.
	#[error("Transport error while ending a session.")]
	TransportEndSession(transport::error::Error),

	/// Transport error while sending a message.
	#[error("Transport error while sending a message.")]
	TransportSendMessage(transport::error::Error),

	/// Transport error while receiving a message.
	#[error("Transport error while receiving a message.")]
	TransportReceiveMessage(transport::error::Error),

	/// Received an unexpected message type from the device.
	#[error("Received an unexpected message type from the device.")]
	UnexpectedMessageType(protos::MessageType), //TODO(stevenroose) type alias

	/// Error reading or writing protobuf messages.
	#[error("Error reading or writing protobuf messages.")]
	DecodeError(prost::DecodeError),

	/// Error encoding protobuf messages
	#[error("Error encoding protobuf messages")]
	EncodeError(prost::EncodeError),

	/// A failure message was returned by the device.
	#[error("failure received: {:?}", .0)]
	FailureResponse(protos::common::Failure),

	/// An unexpected interaction request was returned by the device.
	#[cfg(todo)]
	#[error("An unexpected interaction request was returned by the device: {0}")]
	UnexpectedInteractionRequest(InteractionType),

	/// The given Bitcoin network is not supported.
	#[error("The given Bitcoin network is not supported.")]
	UnsupportedNetwork,

	/// Provided entropy is not 32 bytes.
	#[error("Provided entropy is not 32 bytes.")]
	InvalidEntropy,

	/// The device referenced a non-existing input or output index.
	#[error("The device referenced a non-existing input or output index.")]
	TxRequestInvalidIndex(usize),

	/// Device produced invalid TxRequest message.
	#[error("Invalid TxRequest")]
	MalformedTxRequest(protos::bitcoin::TxRequest),

	/// User provided invalid PSBT.
	#[error("Invalid PSBT: {0}")]
	InvalidPsbt(String),

	/// Failed to parse from string
	#[error("Failed to parse '{0}' from string")]
	ToString(String),

	/// Button request
	#[error("User button request")]
	ButtonRequest(protos::common::ButtonRequest),

	/// Pin matrix request
	#[error("Pin matrix request")]
	PinMatrixRequest(protos::common::PinMatrixRequest),

	/// Passphrase request
	#[error("Passphrase request")]
	PassphraseRequest(protos::common::PassphraseRequest),
}

impl From<prost::DecodeError> for Error {
	fn from(e: prost::DecodeError) -> Error {
		Error::DecodeError(e)
	}
}

impl From<prost::EncodeError> for Error {
	fn from(e: prost::EncodeError) -> Error {
		Error::EncodeError(e)
	}
}

/// Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;
