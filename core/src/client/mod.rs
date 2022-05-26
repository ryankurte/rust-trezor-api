
use log::{trace, debug};

use trezor_protos::{self as protos, TrezorMessage, MessageType};
use trezor_protos::management::{Features};
use trezor_protos::common::{Success};

//pub mod common;
//pub use self::common::*;

use super::Model;
use crate::error::{Error, Result};
use crate::transport::{ProtoMessage, Transport};

/// A Trezor device
pub struct Trezor {
	/// Trezor model
	model: Model,
	/// Cached features for later inspection.
	features: Option<protos::management::Features>,
	/// Underlying transport for device comms
	transport: Box<dyn Transport>,
}

/// A response from a Trezor device.  
/// On every message exchange, instead of the expected/desired response, 
/// the Trezor can ask for some user interaction, or can send a failure.
#[derive(Debug)]
pub enum TrezorResponse<R: TrezorMessage> {
	Ok(R),
	ButtonRequest(protos::common::ButtonRequest),
	PinMatrixRequest(protos::common::PinMatrixRequest),
	PassphraseRequest(protos::common::PassphraseRequest),
}

impl <R: TrezorMessage> TrezorResponse<R> {
	pub fn ok(&self) -> Option<&R> {
		match &self {
			Self::Ok(v) => Some(v),
			_ => None,
		}
	}

	pub fn is_ok(&self) -> bool {
		match &self {
			Self::Ok(v) => true,
			_ => false,
		}
	}
}

impl Trezor {
	/// Create a trezor device instance with the provided transport
	pub fn new_with_transport(model: Model, transport: Box<dyn Transport>) -> Self {
		let s = Self {
			model,
			transport,
			features: None,
		};

		// TODO: check comms / initialise / load features

		s
	}
	
	/// Get the model of the Trezor device.
	pub fn model(&self) -> Model {
		self.model
	}

	/// Get the features of the Trezor device.
	pub fn features(&self) -> Option<&protos::management::Features> {
		self.features.as_ref()
	}

	/// Sends a message and returns the raw ProtoMessage struct that was responded by the device.
	/// This method is only exported for users that want to expand the features of this library
	/// f.e. for supporting additional coins etc.
	pub fn call_raw<S: TrezorMessage>(&mut self, message: S) -> Result<ProtoMessage> {
		let proto_msg = ProtoMessage(S::MESSAGE_TYPE, message.encode_to_vec());
		self.transport.write_message(proto_msg).map_err(Error::TransportSendMessage)?;
		self.transport.read_message().map_err(Error::TransportReceiveMessage)
	}

	/// Sends a message and returns a TrezorResponse with either the expected response message,
	/// a failure or an interaction request.
	/// This method is only exported for users that want to expand the features of this library
	/// f.e. for supporting additional coins etc.
	pub fn call<REQ: TrezorMessage, RESP: TrezorMessage>(
		&mut self,
		message: REQ,
	) -> Result<TrezorResponse<RESP>> {
		trace!("Sending {:?} msg: {:?}", REQ::MESSAGE_TYPE, message);
		let resp = self.call_raw(message)?;

		// Parse expected message type if response matches
		if resp.message_type() == RESP::MESSAGE_TYPE {
			let resp_msg = RESP::decode(resp.payload())?;
			trace!("Received {:?} msg: {:?}", RESP::MESSAGE_TYPE, resp_msg);
			return Ok(TrezorResponse::Ok(resp_msg));
		}

		// Otherwise handle default messages
		match resp.message_type() {
			MessageType::ButtonRequest => {
				let req_msg = resp.into_message()?;
				trace!("Received ButtonRequest: {:?}", req_msg);
				Ok(TrezorResponse::ButtonRequest(req_msg))
			},
			MessageType::PinMatrixRequest => {
				let req_msg = resp.into_message()?;
				trace!("Received PinMatrixRequest: {:?}", req_msg);
				Ok(TrezorResponse::PinMatrixRequest(req_msg))
			},
			MessageType::PassphraseRequest => {
				let req_msg = resp.into_message()?;
				trace!("Received PassphraseRequest: {:?}", req_msg);
				Ok(TrezorResponse::PassphraseRequest(req_msg))
			},
			MessageType::Failure => {
				let fail_msg = resp.into_message()?;
				debug!("Received failure: {:?}", fail_msg);
				Err(Error::FailureResponse(fail_msg))
			},
			// Unexpected message type
			mtype => {
				debug!(
					"Received unexpected msg type: {:?}; raw msg: {}",
					mtype,
					hex::encode(resp.into_payload())
				);
				Err(Error::UnexpectedMessageType(mtype))
			}
		}

	}

	pub fn init_device(&mut self, session_id: Option<Vec<u8>>) -> Result<()> {
		let features = self.initialize(session_id)?;
		self.features = features.ok().map(|v| v.clone() );
		Ok(())
	}

	pub fn initialize(
		&mut self,
		session_id: Option<Vec<u8>>,
	) -> Result<TrezorResponse<Features>> {
		let mut req = protos::management::Initialize::default();
		if let Some(session_id) = session_id {
			req.session_id = Some(session_id);
		}
		self.call(req)
	}

	pub fn ping(&mut self, message: &str) -> Result<TrezorResponse<Success>> {
		let mut req = protos::management::Ping::default();
		req.message = Some(message.to_owned());
		self.call(req)
	}

	pub fn change_pin(&mut self, remove: bool) -> Result<TrezorResponse<Success>> {
		let mut req = protos::management::ChangePin::default();
		req.remove = Some(remove);
		self.call(req)
	}

	pub fn wipe_device(&mut self) -> Result<TrezorResponse<Success>> {
		let req = protos::management::WipeDevice::default();
		self.call(req)
	}

	pub fn recover_device(
		&mut self,
		// TODO(@ryankurte): WordCount enum
		word_count: u32,
		passphrase_protection: bool,
		pin_protection: bool,
		label: String,
		dry_run: bool,
	) -> Result<TrezorResponse<Success>> {
		let req = protos::management::RecoveryDevice{
			word_count: Some(word_count as u32),
			passphrase_protection: Some(passphrase_protection),
			pin_protection: Some(pin_protection),
			label: Some(label),
			enforce_wordlist: Some(true),
			dry_run: Some(dry_run),
			r#type: Some(protos::management::recovery_device::RecoveryDeviceType::ScrambledWords as i32),
			//TODO(stevenroose) support languages
			language: Some("english".to_owned()),
			..Default::default()
		};
		
		self.call(req)
	}

	#[allow(clippy::too_many_arguments)]
	pub fn reset_device(
		&mut self,
		display_random: bool,
		strength: usize,
		passphrase_protection: bool,
		pin_protection: bool,
		label: String,
		skip_backup: bool,
		no_backup: bool,
	) -> Result<TrezorResponse<protos::management::EntropyRequest>> {
		let req = protos::management::ResetDevice {
			display_random: Some(display_random),
			strength: Some(strength as u32),
			passphrase_protection: Some(passphrase_protection),
			pin_protection: Some(pin_protection),
			label: Some(label),
			skip_backup: Some(skip_backup),
			no_backup: Some(no_backup),
			..Default::default()
		};
		self.call(req)
	}

	pub fn backup(&mut self) -> Result<TrezorResponse<Success>> {
		let req = protos::management::BackupDevice::default();
		self.call(req)
	}

	//TODO(stevenroose) support U2F stuff? currently ignored all

	pub fn apply_settings(
		&mut self,
		label: Option<String>,
		use_passphrase: Option<bool>,
		homescreen: Option<Vec<u8>>,
		auto_lock_delay_ms: Option<usize>,
	) -> Result<TrezorResponse<Success>> {
		let mut req = protos::management::ApplySettings::default();
		if let Some(label) = label {
			req.label = Some(label);
		}
		if let Some(use_passphrase) = use_passphrase {
			req.use_passphrase = Some(use_passphrase);
		}
		if let Some(homescreen) = homescreen {
			req.homescreen = Some(homescreen);
		}
		if let Some(auto_lock_delay_ms) = auto_lock_delay_ms {
			req.auto_lock_delay_ms = Some(auto_lock_delay_ms as u32);
		}
		self.call(req)
	}

	#[cfg(todo)]
	pub fn sign_identity(
		&mut self,
		identity: IdentityType,
		digest: Vec<u8>,
		curve: String,
	) -> Result<TrezorResponse<Vec<u8>, SignedIdentity>> {
		let mut req = SignIdentity::default();
		req.set_identity(identity);
		req.set_challenge_hidden(digest);
		req.set_challenge_visual("".to_owned());
		req.set_ecdsa_curve_name(curve);
		self.call(req, Box::new(|_, m| Ok(m.get_signature().to_owned())))
	}
}
