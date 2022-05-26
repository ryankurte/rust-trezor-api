//! # Error Handling


#[cfg(feature = "f_bitcoin")]
use bitcoin;

#[cfg(feature = "f_bitcoin")]
use bitcoin::util::base58;

#[cfg(feature = "f_bitcoin")]
use bitcoin_hashes::sha256d;

#[cfg(feature = "f_bitcoin")]
use secp256k1;

use crate::client::InteractionType;
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
	#[error("failure received")]
	FailureResponse(protos::common::Failure),

	/// An unexpected interaction request was returned by the device.
	#[error("An unexpected interaction request was returned by the device: {0}")]
	UnexpectedInteractionRequest(InteractionType),

	/// Error in Base58 decoding
	#[cfg(feature = "f_bitcoin")]
	#[error("Error in Base58 decoding: {0}")]
	Base58(base58::Error),

	/// The given Bitcoin network is not supported.
	#[error("The given Bitcoin network is not supported.")]
	UnsupportedNetwork,

	/// Provided entropy is not 32 bytes.
	#[error("Provided entropy is not 32 bytes.")]
	InvalidEntropy,

	/// The device referenced a non-existing input or output index.
	#[error("The device referenced a non-existing input or output index.")]
	TxRequestInvalidIndex(usize),

	/// The device referenced an unknown TXID.
	#[cfg(feature = "f_bitcoin")]
	#[error("The device referenced an unknown TXID: {0}")]
	TxRequestUnknownTxid(sha256d::Hash),

	/// The PSBT is missing the full tx for given input.
	#[cfg(feature = "f_bitcoin")]
	#[error("The PSBT is missing the full tx for given input: {0}")]
	PsbtMissingInputTx(sha256d::Hash),

	/// Device produced invalid TxRequest message.
	#[error("Invalid TxRequest")]
	MalformedTxRequest(protos::bitcoin::TxRequest),

	/// User provided invalid PSBT.
	#[error("Invalid PSBT: {0}")]
	InvalidPsbt(String),

	/// Error encoding/decoding a Bitcoin data structure.
	#[cfg(feature = "f_bitcoin")]
	#[error("Error encoding/decoding a Bitcoin data structure: {0}")]
	BitcoinEncode(bitcoin::consensus::encode::Error),

	/// Elliptic curve crypto error.
	#[cfg(feature = "f_bitcoin")]
	#[error("Elliptic curve crypto error: {0}")]
	Secp256k1(secp256k1::Error),
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

#[cfg(feature = "f_bitcoin")]
impl From<base58::Error> for Error {
	fn from(e: base58::Error) -> Error {
		Error::Base58(e)
	}
}

#[cfg(feature = "f_bitcoin")]
impl From<bitcoin::consensus::encode::Error> for Error {
	fn from(e: bitcoin::consensus::encode::Error) -> Error {
		Error::BitcoinEncode(e)
	}
}

#[cfg(feature = "f_bitcoin")]
impl From<secp256k1::Error> for Error {
	fn from(e: secp256k1::Error) -> Error {
		Error::Secp256k1(e)
	}
}

/// Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;
