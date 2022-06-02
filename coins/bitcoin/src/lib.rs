

use std::str::FromStr;


use bitcoin::{Address as BitcoinAddress};
use bitcoin::network::constants::Network; //TODO(stevenroose) change after https://github.com/rust-bitcoin/rust-bitcoin/pull/181
use bitcoin::util::{bip32, psbt};

use secp256k1::ecdsa::RecoverableSignature;

use unicode_normalization::UnicodeNormalization;

use trezor_client::{Trezor};
use trezor_protos::{self as protos};
use trezor_protos::bitcoin::{InputScriptType, PublicKey, TxRequest, Address, MessageSignature};

mod flows;

mod utils;

mod error;
pub use error::BtcError;


pub struct GetPublicKey {
	pub path: bip32::DerivationPath,
	pub script_type: InputScriptType,
	pub network: Network,
	pub show_display: bool,
}


pub trait Bitcoin {
	fn get_public_key(
		&mut self,
		opts: GetPublicKey,
	) -> Result<bip32::ExtendedPubKey, BtcError>;

	fn get_address(
		&mut self,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
		show_display: bool,
	) -> Result<Address, BtcError>;

	fn sign_tx(
		&mut self,
		psbt: &psbt::PartiallySignedTransaction,
		network: Network,
	) -> Result<SignTxProgress, BtcError>;

	fn sign_message(
		&mut self,
		message: String,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
	) -> Result<(Address, RecoverableSignature), BtcError>;
}

impl Bitcoin for Trezor {
	fn get_public_key(
		&mut self,
		opts: GetPublicKey,
	) -> Result<bip32::ExtendedPubKey, BtcError> {
		let req = protos::bitcoin::GetPublicKey{
			address_n: utils::convert_path(&opts.path),
			show_display: Some(opts.show_display),
			coin_name: Some(utils::coin_name(opts.network)?),
			script_type: Some(opts.script_type as i32),
			..Default::default()
		};
		
		let resp = self.call::<_, PublicKey>(req)?;

		let pk = bip32::ExtendedPubKey::from_str(&resp.xpub)
			.map_err(|e| BtcError::Bip32(e) )?;
		
		Ok(pk)
	}

	//TODO(stevenroose) multisig
	fn get_address(
		&mut self,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
		show_display: bool,
	) -> Result<Address, BtcError> {
		let mut req = protos::bitcoin::GetAddress{
			address_n: utils::convert_path(path),
			coin_name: Some(utils::coin_name(network)?),
			show_display: Some(show_display),
			script_type: Some(script_type as i32),
			..Default::default()
		};

		let a = self.call::<_, Address>(req)?;

		Ok(a)
	}

	fn sign_tx(
		&mut self,
		psbt: &psbt::PartiallySignedTransaction,
		network: Network,
	) -> Result<SignTxProgress, BtcError> {
		let tx = &psbt.unsigned_tx;

		let req = protos::bitcoin::SignTx{
			inputs_count: tx.input.len() as u32,
			outputs_count: tx.output.len() as u32,
			coin_name: Some(utils::coin_name(network)?),
			version: Some(tx.version as u32),
			lock_time: Some(tx.lock_time),
			..Default::default()
		};

		todo!()
	}

	fn sign_message(
		&mut self,
		message: String,
		path: &bip32::DerivationPath,
		script_type: InputScriptType,
		network: Network,
	) -> Result<(Address, RecoverableSignature), BtcError>
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
		
		todo!()
	}
}
