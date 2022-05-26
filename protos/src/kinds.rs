//! Helper `TrezorMessage`] trait to provide a MESSAGE_TYPE for all protobuf message objects

use crate::gen::*;

/// This trait extends the protobuf Message trait to also have a constant for the message
/// type code.  This getter is implemented in this file for all the messages we use.
pub trait TrezorMessage: prost::Message + Default {
	const MESSAGE_TYPE: MessageType;
}

/// This macro provides the TrezorMessage trait for a protobuf message.
macro_rules! trezor_message_impl {
	($struct:ty, $mtype:expr) => {
		impl TrezorMessage for $struct {
			const MESSAGE_TYPE: MessageType = $mtype;
		}
	};
}

// Common

trezor_message_impl!(common::Success, MessageType::Success);
trezor_message_impl!(common::Failure, MessageType::Failure);
trezor_message_impl!(common::PinMatrixRequest, MessageType::PinMatrixRequest);
trezor_message_impl!(common::PinMatrixAck, MessageType::PinMatrixAck);
trezor_message_impl!(common::ButtonRequest, MessageType::ButtonRequest);
trezor_message_impl!(common::ButtonAck, MessageType::ButtonAck);
trezor_message_impl!(common::PassphraseRequest, MessageType::PassphraseRequest);
trezor_message_impl!(common::PassphraseAck, MessageType::PassphraseAck);

// Management

trezor_message_impl!(management::Initialize, MessageType::Initialize);
trezor_message_impl!(management::Ping, MessageType::Ping);
trezor_message_impl!(management::ChangePin, MessageType::ChangePin);
trezor_message_impl!(management::WipeDevice, MessageType::WipeDevice);
trezor_message_impl!(management::GetEntropy, MessageType::GetEntropy);
trezor_message_impl!(management::Entropy, MessageType::Entropy);
trezor_message_impl!(management::LoadDevice, MessageType::LoadDevice);
trezor_message_impl!(management::ResetDevice, MessageType::ResetDevice);
trezor_message_impl!(management::Features, MessageType::Features);
trezor_message_impl!(management::Cancel, MessageType::Cancel);
trezor_message_impl!(management::ApplySettings, MessageType::ApplySettings);
trezor_message_impl!(management::ApplyFlags, MessageType::ApplyFlags);
trezor_message_impl!(management::BackupDevice, MessageType::BackupDevice);
trezor_message_impl!(management::EntropyRequest, MessageType::EntropyRequest);
trezor_message_impl!(management::EntropyAck, MessageType::EntropyAck);
trezor_message_impl!(management::RecoveryDevice, MessageType::RecoveryDevice);
trezor_message_impl!(management::WordRequest, MessageType::WordRequest);
trezor_message_impl!(management::WordAck, MessageType::WordAck);
trezor_message_impl!(management::GetFeatures, MessageType::GetFeatures);

// Bootloader

//trezor_message_impl!(SetU2FCounter, MessageType::SetU2FCounter);
trezor_message_impl!(bootloader::FirmwareErase, MessageType::FirmwareErase);
trezor_message_impl!(bootloader::FirmwareUpload, MessageType::FirmwareUpload);
trezor_message_impl!(bootloader::FirmwareRequest, MessageType::FirmwareRequest);
trezor_message_impl!(bootloader::SelfTest, MessageType::SelfTest);

// Bitcoin

trezor_message_impl!(bitcoin::GetPublicKey, MessageType::GetPublicKey);
trezor_message_impl!(bitcoin::PublicKey, MessageType::PublicKey);
trezor_message_impl!(bitcoin::SignTx, MessageType::SignTx);
trezor_message_impl!(bitcoin::TxRequest, MessageType::TxRequest);
trezor_message_impl!(bitcoin::TxAck, MessageType::TxAck);
trezor_message_impl!(bitcoin::GetAddress, MessageType::GetAddress);
trezor_message_impl!(bitcoin::Address, MessageType::Address);
trezor_message_impl!(bitcoin::SignMessage, MessageType::SignMessage);
trezor_message_impl!(bitcoin::VerifyMessage, MessageType::VerifyMessage);
trezor_message_impl!(bitcoin::MessageSignature, MessageType::MessageSignature);

//trezor_message_impl!(CipherKeyValue, MessageType::CipherKeyValue);
//trezor_message_impl!(CipheredKeyValue, MessageType::CipheredKeyValue);
//trezor_message_impl!(SignIdentity, MessageType::SignIdentity);
//trezor_message_impl!(SignedIdentity, MessageType::SignedIdentity);

//trezor_message_impl!(GetECDHSessionKey, MessageType::GetECDHSessionKey);
//trezor_message_impl!(ECDHSessionKey, MessageType::ECDHSessionKey);
//trezor_message_impl!(CosiCommit, MessageType::CosiCommit);
//trezor_message_impl!(CosiCommitment, MessageType::CosiCommitment);
//trezor_message_impl!(CosiSign, MessageType::CosiSign);
//trezor_message_impl!(CosiSignature, MessageType::CosiSignature);

//trezor_message_impl!(DebugLinkDecision, MessageType::DebugLinkDecision);
//trezor_message_impl!(DebugLinkGetState, MessageType::DebugLinkGetState);
//trezor_message_impl!(DebugLinkState, MessageType::DebugLinkState);
//trezor_message_impl!(DebugLinkStop, MessageType::DebugLinkStop);
//trezor_message_impl!(DebugLinkLog, MessageType::DebugLinkLog);
//trezor_message_impl!(DebugLinkMemoryRead, MessageType::DebugLinkMemoryRead);
//trezor_message_impl!(DebugLinkMemory, MessageType::DebugLinkMemory);
//trezor_message_impl!(DebugLinkMemoryWrite, MessageType::DebugLinkMemoryWrite);
//trezor_message_impl!(DebugLinkFlashErase, MessageType::DebugLinkFlashErase);

// Ethereum

trezor_message_impl!(ethereum::EthereumGetAddress, MessageType::EthereumGetAddress);
trezor_message_impl!(ethereum::EthereumAddress, MessageType::EthereumAddress);
trezor_message_impl!(ethereum::EthereumSignTx, MessageType::EthereumSignTx);
trezor_message_impl!(ethereum::EthereumSignTxEip1559, MessageType::EthereumSignTxEip1559);
trezor_message_impl!(ethereum::EthereumTxRequest, MessageType::EthereumTxRequest);
trezor_message_impl!(ethereum::EthereumTxAck, MessageType::EthereumTxAck);
trezor_message_impl!(ethereum::EthereumSignMessage, MessageType::EthereumSignMessage);
trezor_message_impl!(ethereum::EthereumVerifyMessage, MessageType::EthereumVerifyMessage);
trezor_message_impl!(ethereum::EthereumMessageSignature, MessageType::EthereumMessageSignature);
trezor_message_impl!(ethereum_eip712::EthereumSignTypedData, MessageType::EthereumSignTypedData);
trezor_message_impl!(
	ethereum_eip712::EthereumTypedDataStructRequest,
	MessageType::EthereumTypedDataStructRequest
);
trezor_message_impl!(
	ethereum_eip712::EthereumTypedDataStructAck,
	MessageType::EthereumTypedDataStructAck
);
trezor_message_impl!(
	ethereum_eip712::EthereumTypedDataValueRequest,
	MessageType::EthereumTypedDataValueRequest
);
trezor_message_impl!(
	ethereum_eip712::EthereumTypedDataValueAck,
	MessageType::EthereumTypedDataValueAck
);
trezor_message_impl!(ethereum::EthereumTypedDataSignature, MessageType::EthereumTypedDataSignature);

// NEM

//trezor_message_impl!(NEMGetAddress, MessageType::NEMGetAddress);
//trezor_message_impl!(NEMAddress, MessageType::NEMAddress);
//trezor_message_impl!(NEMSignTx, MessageType::NEMSignTx);
//trezor_message_impl!(NEMSignedTx, MessageType::NEMSignedTx);
//trezor_message_impl!(NEMDecryptMessage, MessageType::NEMDecryptMessage);
//trezor_message_impl!(NEMDecryptedMessage, MessageType::NEMDecryptedMessage);
//trezor_message_impl!(LiskGetAddress, MessageType::LiskGetAddress);
//trezor_message_impl!(LiskAddress, MessageType::LiskAddress);
//trezor_message_impl!(LiskSignTx, MessageType::LiskSignTx);
//trezor_message_impl!(LiskSignedTx, MessageType::LiskSignedTx);
//trezor_message_impl!(LiskSignMessage, MessageType::LiskSignMessage);
//trezor_message_impl!(LiskMessageSignature, MessageType::LiskMessageSignature);
//trezor_message_impl!(LiskVerifyMessage, MessageType::LiskVerifyMessage);
//trezor_message_impl!(LiskGetPublicKey, MessageType::LiskGetPublicKey);
//trezor_message_impl!(LiskPublicKey, MessageType::LiskPublicKey);

// Tezos

trezor_message_impl!(tezos::TezosGetAddress, MessageType::TezosGetAddress);
trezor_message_impl!(tezos::TezosAddress, MessageType::TezosAddress);
trezor_message_impl!(tezos::TezosSignTx, MessageType::TezosSignTx);
trezor_message_impl!(tezos::TezosSignedTx, MessageType::TezosSignedTx);
trezor_message_impl!(tezos::TezosGetPublicKey, MessageType::TezosGetPublicKey);
trezor_message_impl!(tezos::TezosPublicKey, MessageType::TezosPublicKey);

// Stellar

trezor_message_impl!(stellar::StellarSignTx, MessageType::StellarSignTx);
trezor_message_impl!(stellar::StellarTxOpRequest, MessageType::StellarTxOpRequest);
trezor_message_impl!(stellar::StellarGetAddress, MessageType::StellarGetAddress);
trezor_message_impl!(stellar::StellarAddress, MessageType::StellarAddress);
trezor_message_impl!(stellar::StellarCreateAccountOp, MessageType::StellarCreateAccountOp);
trezor_message_impl!(stellar::StellarPaymentOp, MessageType::StellarPaymentOp);
//trezor_message_impl!(stellar::StellarPathPaymentOp, MessageType::StellarPathPaymentOp);
//trezor_message_impl!(stellar::StellarManageOfferOp, MessageType::StellarManageOfferOp);
//trezor_message_impl!(stellar::StellarCreatePassiveOfferOp, MessageType::StellarCreatePassiveOfferOp);
trezor_message_impl!(stellar::StellarSetOptionsOp, MessageType::StellarSetOptionsOp);
trezor_message_impl!(stellar::StellarChangeTrustOp, MessageType::StellarChangeTrustOp);
trezor_message_impl!(stellar::StellarAllowTrustOp, MessageType::StellarAllowTrustOp);
trezor_message_impl!(stellar::StellarAccountMergeOp, MessageType::StellarAccountMergeOp);
trezor_message_impl!(stellar::StellarManageDataOp, MessageType::StellarManageDataOp);
trezor_message_impl!(stellar::StellarBumpSequenceOp, MessageType::StellarBumpSequenceOp);
trezor_message_impl!(stellar::StellarSignedTx, MessageType::StellarSignedTx);

//trezor_message_impl!(TronGetAddress, MessageType::TronGetAddress);
//trezor_message_impl!(TronAddress, MessageType::TronAddress);
//trezor_message_impl!(TronSignTx, MessageType::TronSignTx);
//trezor_message_impl!(TronSignedTx, MessageType::TronSignedTx);

// Cardano

trezor_message_impl!(cardano::CardanoSignTxInit, MessageType::CardanoSignTxInit);
trezor_message_impl!(cardano::CardanoTxInput, MessageType::CardanoTxInput);
trezor_message_impl!(cardano::CardanoGetPublicKey, MessageType::CardanoGetPublicKey);
trezor_message_impl!(cardano::CardanoPublicKey, MessageType::CardanoPublicKey);
trezor_message_impl!(cardano::CardanoGetAddress, MessageType::CardanoGetAddress);
trezor_message_impl!(cardano::CardanoAddress, MessageType::CardanoAddress);
trezor_message_impl!(cardano::CardanoToken, MessageType::CardanoToken);

// Ontology

//trezor_message_impl!(OntologyGetAddress, MessageType::OntologyGetAddress);
//trezor_message_impl!(OntologyAddress, MessageType::OntologyAddress);
//trezor_message_impl!(OntologyGetPublicKey, MessageType::OntologyGetPublicKey);
//trezor_message_impl!(OntologyPublicKey, MessageType::OntologyPublicKey);
//trezor_message_impl!(OntologySignTransfer, MessageType::OntologySignTransfer);
//trezor_message_impl!(OntologySignedTransfer, MessageType::OntologySignedTransfer);
//trezor_message_impl!(OntologySignWithdrawOng, MessageType::OntologySignWithdrawOng);
//trezor_message_impl!(OntologySignedWithdrawOng, MessageType::OntologySignedWithdrawOng);
//trezor_message_impl!(OntologySignOntIdRegister, MessageType::OntologySignOntIdRegister);
//trezor_message_impl!(OntologySignedOntIdRegister, MessageType::OntologySignedOntIdRegister);
//trezor_message_impl!(OntologySignOntIdAddAttributes, MessageType::OntologySignOntIdAddAttributes);
//trezor_message_impl!(OntologySignedOntIdAddAttributes, MessageType::OntologySignedOntIdAddAttributes);

// Ripple

trezor_message_impl!(ripple::RippleGetAddress, MessageType::RippleGetAddress);
trezor_message_impl!(ripple::RippleAddress, MessageType::RippleAddress);
trezor_message_impl!(ripple::RippleSignTx, MessageType::RippleSignTx);
trezor_message_impl!(ripple::RippleSignedTx, MessageType::RippleSignedTx);

// Monero

trezor_message_impl!(
	monero::MoneroTransactionInitRequest,
	MessageType::MoneroTransactionInitRequest
);
trezor_message_impl!(monero::MoneroTransactionInitAck, MessageType::MoneroTransactionInitAck);
trezor_message_impl!(
	monero::MoneroTransactionSetInputRequest,
	MessageType::MoneroTransactionSetInputRequest
);
trezor_message_impl!(
	monero::MoneroTransactionSetInputAck,
	MessageType::MoneroTransactionSetInputAck
);
trezor_message_impl!(
	monero::MoneroTransactionInputViniRequest,
	MessageType::MoneroTransactionInputViniRequest
);
trezor_message_impl!(
	monero::MoneroTransactionInputViniAck,
	MessageType::MoneroTransactionInputViniAck
);
trezor_message_impl!(
	monero::MoneroTransactionAllInputsSetRequest,
	MessageType::MoneroTransactionAllInputsSetRequest
);
trezor_message_impl!(
	monero::MoneroTransactionAllInputsSetAck,
	MessageType::MoneroTransactionAllInputsSetAck
);
trezor_message_impl!(
	monero::MoneroTransactionSetOutputRequest,
	MessageType::MoneroTransactionSetOutputRequest
);
trezor_message_impl!(
	monero::MoneroTransactionSetOutputAck,
	MessageType::MoneroTransactionSetOutputAck
);
trezor_message_impl!(
	monero::MoneroTransactionAllOutSetRequest,
	MessageType::MoneroTransactionAllOutSetRequest
);
trezor_message_impl!(
	monero::MoneroTransactionAllOutSetAck,
	MessageType::MoneroTransactionAllOutSetAck
);
//trezor_message_impl!(monero::MoneroTransactionMlsagDoneRequest, MessageType::MoneroTransactionMlsagDoneRequest);
//trezor_message_impl!(MoneroTransactionMlsagDoneAck, MessageType::MoneroTransactionMlsagDoneAck);
trezor_message_impl!(
	monero::MoneroTransactionSignInputRequest,
	MessageType::MoneroTransactionSignInputRequest
);
trezor_message_impl!(
	monero::MoneroTransactionSignInputAck,
	MessageType::MoneroTransactionSignInputAck
);
trezor_message_impl!(
	monero::MoneroTransactionFinalRequest,
	MessageType::MoneroTransactionFinalRequest
);
trezor_message_impl!(monero::MoneroTransactionFinalAck, MessageType::MoneroTransactionFinalAck);
trezor_message_impl!(
	monero::MoneroKeyImageExportInitRequest,
	MessageType::MoneroKeyImageExportInitRequest
);
trezor_message_impl!(monero::MoneroKeyImageExportInitAck, MessageType::MoneroKeyImageExportInitAck);
trezor_message_impl!(
	monero::MoneroKeyImageSyncStepRequest,
	MessageType::MoneroKeyImageSyncStepRequest
);
trezor_message_impl!(monero::MoneroKeyImageSyncStepAck, MessageType::MoneroKeyImageSyncStepAck);
trezor_message_impl!(
	monero::MoneroKeyImageSyncFinalRequest,
	MessageType::MoneroKeyImageSyncFinalRequest
);
trezor_message_impl!(monero::MoneroKeyImageSyncFinalAck, MessageType::MoneroKeyImageSyncFinalAck);
trezor_message_impl!(monero::MoneroGetAddress, MessageType::MoneroGetAddress);
trezor_message_impl!(monero::MoneroAddress, MessageType::MoneroAddress);
trezor_message_impl!(monero::MoneroGetWatchKey, MessageType::MoneroGetWatchKey);
trezor_message_impl!(monero::MoneroWatchKey, MessageType::MoneroWatchKey);
trezor_message_impl!(monero::DebugMoneroDiagRequest, MessageType::DebugMoneroDiagRequest);
trezor_message_impl!(monero::DebugMoneroDiagAck, MessageType::DebugMoneroDiagAck);
