

use std::f32::consts::E;
use std::str::FromStr;

use bitcoin;

use bitcoin::util::base58;

use bitcoin_hashes::sha256d;

use secp256k1;

use bitcoin::{Address as BitcoinAddress};
use bitcoin::network::constants::Network; //TODO(stevenroose) change after https://github.com/rust-bitcoin/rust-bitcoin/pull/181
use bitcoin::util::{bip32, psbt};
use secp256k1::ecdsa::RecoverableSignature;

use unicode_normalization::UnicodeNormalization;

use trezor_client::{Trezor, TrezorResponse};
use trezor_protos::{self as protos};
use trezor_protos::bitcoin::{InputScriptType, PublicKey, TxRequest, Address, MessageSignature};


mod flows;
use flows::sign_tx::SignTxProgress;

mod utils;


pub struct GetPublicKey {
	pub path: bip32::DerivationPath,
	pub script_type: InputScriptType,
	pub network: Network,
	pub show_display: bool,
}

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


pub trait Bitcoin {
	fn get_public_key(
		&mut self,
		opts: GetPublicKey,
	) -> Result<TrezorResponse<bip32::ExtendedPubKey, PublicKey>, BtcError>;

	fn get_address(
		&mut self,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
		show_display: bool,
	) -> Result<TrezorResponse<Address, Address>, BtcError>;

	fn sign_tx(
		&mut self,
		psbt: &psbt::PartiallySignedTransaction,
		network: Network,
	) -> Result<TrezorResponse<SignTxProgress, TxRequest>, BtcError>;

	fn sign_message(
		&mut self,
		message: String,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
	) -> Result<TrezorResponse<(Address, RecoverableSignature), MessageSignature>, BtcError>;
}

impl Bitcoin for Trezor {
	fn get_public_key(
		&mut self,
		opts: GetPublicKey,
	) -> Result<TrezorResponse<bip32::ExtendedPubKey, PublicKey>, BtcError> {
		let req = protos::bitcoin::GetPublicKey{
			address_n: utils::convert_path(&opts.path),
			show_display: Some(opts.show_display),
			coin_name: Some(utils::coin_name(opts.network)?),
			script_type: Some(opts.script_type as i32),
			..Default::default()
		};
		
		self.call(req, Box::new(|_, m| {
			let pk = bip32::ExtendedPubKey::from_str(&m.xpub)?;
			Ok(pk)
		}))
	}

	//TODO(stevenroose) multisig
	fn get_address(
		&mut self,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
		show_display: bool,
	) -> Result<TrezorResponse<Address, Address>, BtcError> {
		let mut req = protos::bitcoin::GetAddress{
			address_n: utils::convert_path(path),
			coin_name: Some(utils::coin_name(network)?),
			show_display: Some(show_display),
			script_type: Some(script_type as i32),
			..Default::default()
		};

		self.call(req, Box::new(|_, m| Ok(m)))
	}

	fn sign_tx(
		&mut self,
		psbt: &psbt::PartiallySignedTransaction,
		network: Network,
	) -> Result<TrezorResponse<SignTxProgress, TxRequest>, BtcError> {
		let tx = &psbt.unsigned_tx;

		let req = protos::bitcoin::SignTx{
			inputs_count: tx.input.len() as u32,
			outputs_count: tx.output.len() as u32,
			coin_name: Some(utils::coin_name(network)?),
			version: Some(tx.version as u32),
			lock_time: Some(tx.lock_time),
			..Default::default()
		};

		self.call(req, Box::new(|c, m| Ok(SignTxProgress::new(c, m))))
	}

	fn sign_message(
		&mut self,
		message: String,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
	) -> Result<TrezorResponse<(Address, RecoverableSignature), MessageSignature>, BtcError>
	{
		// Normalize to Unicode NFC.
		let msg_bytes = message.nfc().collect::<String>().into_bytes();

		let req = protos::bitcoin::SignMessage{
			address_n: utils::convert_path(path),
			message: msg_bytes,
			coin_name: Some(utils::coin_name(network)?),
			script_type: Some(script_type as i32),
			..Default::default()
		};
		
		self.call(
			req,
			Box::new(|_, m| {
				let address = m.address;
				let signature = utils::parse_recoverable_signature(&m.signature)?;
				Ok((address, signature))
			}),
		)
	}
}
