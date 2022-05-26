//! Generated protobuf modules

include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.rs"));

include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));

pub mod bootloader {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.bootloader.rs"));
}

pub mod common {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.common.rs"));
}

pub mod management {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.management.rs"));
}

pub mod debug {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.debug.rs"));
}

pub mod bitcoin {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.bitcoin.rs"));
}

pub mod cardano {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.cardano.rs"));
}

pub mod ethereum {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.ethereum.rs"));
}

pub mod ethereum_eip712 {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.ethereum_eip712.rs"));
}

pub mod monero {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.monero.rs"));
}

pub mod nem {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.nem.rs"));
}

pub mod ripple {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.ripple.rs"));
}

pub mod stellar {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.stellar.rs"));
}

pub mod tezos {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.tezos.rs"));
}

pub mod webauthn {
	include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.webauthn.rs"));
}
