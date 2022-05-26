pub mod messages;
pub mod messages_bitcoin;
pub mod messages_bootloader;
pub mod messages_common;
pub mod messages_crypto;
pub mod messages_debug;
pub mod messages_management;
// unused:
pub mod messages_cardano;
pub mod messages_ethereum;
pub mod messages_ethereum_eip712;
pub mod messages_lisk;
pub mod messages_monero;
pub mod messages_nem;
pub mod messages_ontology;
pub mod messages_ripple;
pub mod messages_stellar;
pub mod messages_tezos;
pub mod messages_tron;

pub use self::messages::*;
pub use self::messages_bitcoin::*;
pub use self::messages_bootloader::*;
pub use self::messages_common::*;
pub use self::messages_crypto::*;
pub use self::messages_debug::*;
pub use self::messages_management::*;
// unused:
pub use self::messages_cardano::*;
pub use self::messages_ethereum::*;
pub use self::messages_ethereum_eip712::*;
pub use self::messages_lisk::*;
pub use self::messages_monero::*;
pub use self::messages_nem::*;
pub use self::messages_ontology::*;
pub use self::messages_ripple::*;
pub use self::messages_stellar::*;
pub use self::messages_tezos::*;
pub use self::messages_tron::*;

pub mod gen {
    //include!(concat!(env!("OUT_DIR"), "/hw.trezor.messages.rs"));
}
