use clap::Parser;

use log::{debug, info, warn, LevelFilter};
use simplelog::SimpleLogger;

#[derive(Clone, PartialEq, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	#[clap(subcommand)]
	pub command: Commands,

	#[clap(short, long, default_value = "0")]
	/// Device index (where multiple devices are available)
	index: usize,

	#[clap(long)]
	/// Use debug enabled devices
	trezor_debug: bool,

	#[clap(long, default_value = "debug")]
	/// Application log level
	pub log_level: LevelFilter,
}

#[derive(Clone, PartialEq, Debug, Parser)]
pub enum Commands {
	/// List available devices
	List,

	/// Fetch device information
	Info,

	/// Set the pin for a connected device
	SetPin,

	// External subcommands
	#[clap(external_subcommand)]
	External(Vec<String>),
}

fn main() -> anyhow::Result<()> {
	// Parse command line arguments
	let args = Args::parse();
	println!("Args: {:?}", args);

	// Setup logging
	let _ = SimpleLogger::init(args.log_level, Default::default());

	// Discover available devices
	let mut devices = trezor_client::find_devices(args.trezor_debug)?;

	// Run commands that do not require a device first
	match args.command {
		Commands::List => {
			if devices.len() == 0 {
				info!("No devices found");
				return Ok(());
			}

			info!("Found devices: {:?}", devices);

			return Ok(());
		}
		_ => (),
	}

	// Check we have a device matching the supplied index
	if devices.len() == 0 {
		warn!("No devices found");
		return Ok(());
	}
	if args.index >= devices.len() {
		warn!("No device for index {} ({} devices)", args.index, devices.len());
		return Ok(());
	}

	// Then connect to the device
	let mut device = devices.remove(args.index).connect()?;
	device.init_device(None)?;

	debug!("Using device {:?}", device.model());

	// Execute commands
	match args.command {
		Commands::Info => {
			if let Some(f) = device.features() {
				info!("Features:");
				info!("vendor: {}", f.fw_vendor());
				info!("version: {}.{}.{}", f.major_version, f.minor_version, f.patch_version,);
				info!("device id: {}", f.device_id());
				info!("label: {}", f.label());
				info!("is initialized: {}", f.initialized());
				info!("pin protection: {}", f.pin_protection());
				info!("passphrase protection: {}", f.passphrase_protection());
			}
		}
		Commands::SetPin => {
			debug!("Setting device pin");

			// Request existing pin state
			let f = match device.features() {
				Some(v) => v,
				None => return Err(anyhow::anyhow!("Could not fetch device features")),
			};

			if f.pin_protection() {
				debug!("Attempting to set new device pin");
			} else {
				debug!("Attempting to change device pin");
			}

			device.change_pin(false)?;

			// Request new pin

			// Request new pin (again)

			todo!()
		}
		Commands::External(_ext) => {
			todo!("call out to coin subcommands")
		}
		_ => todo!("Unhandled or unexpected command: {:?}", args.command),
	}

	Ok(())
}
