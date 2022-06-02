
use bitcoin::util::base58;
use bitcoin_hashes::sha256d;

/// Bitcoin implementation specific errors
#[derive(Debug, thiserror::Error)]
pub enum BtcError {
	/// Error in Base58 decoding
	#[error("Error in Base58 decoding: {0}")]
	Base58(base58::Error),

	/// The device referenced an unknown TXID.
	#[error("The device referenced an unknown TXID: {0}")]
	TxRequestUnknownTxid(sha256d::Hash),

	/// The PSBT is missing the full tx for given input.
	#[error("The PSBT is missing the full tx for given input: {0}")]
	PsbtMissingInputTx(sha256d::Hash),

	/// Error encoding/decoding a Bitcoin data structure.
	#[error("Error encoding/decoding a Bitcoin data structure: {0}")]
	BitcoinEncode(bitcoin::consensus::encode::Error),

	/// Elliptic curve crypto error.
	#[error("Elliptic curve crypto error: {0}")]
	Secp256k1(secp256k1::Error),

	#[error(transparent)]
	Client(trezor_client::Error),
}

impl From<trezor_client::Error> for BtcError {
    fn from(e: trezor_client::Error) -> Self {
        Self::Client(e)
    }
}

impl From<base58::Error> for BtcError {
	fn from(e: base58::Error) -> Self {
		Self::Base58(e)
	}
}

impl From<bitcoin::consensus::encode::Error> for BtcError {
	fn from(e: bitcoin::consensus::encode::Error) -> Self {
		Self::BitcoinEncode(e)
	}
}

impl From<secp256k1::Error> for BtcError {
	fn from(e: secp256k1::Error) -> Self {
		Self::Secp256k1(e)
	}
}
