
use clap::Parser;

use log::{debug, info, warn, LevelFilter};
use simplelog::SimpleLogger;


#[derive(Clone, PartialEq, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	#[clap(subcommand)]
	pub command: Commands,

	#[clap(short, long, default_value="0")]
    /// Device index (where multiple devices are available)
    index: usize,

    #[clap(long)]
    /// Use debug enabled devices
    trezor_debug: bool,

	#[clap(long, default_value="debug")]
	/// Application log level
	pub log_level: LevelFilter,
}

#[derive(Clone, PartialEq, Debug, Parser)]
pub enum Commands {
	List,
	Info,

	// External subcommands
	#[clap(external_subcommand)]
	External(Vec<String>)
}

fn main() -> anyhow::Result<()> {
	// Parse command line arguments
	let args = Args::parse();
	println!("Args: {:?}", args);

	// Setup logging
	let _ = SimpleLogger::init(args.log_level, Default::default());

	// Execute commands
	match args.command {
		Commands::List => {
            let devices = trezor_client::find_devices(args.trezor_debug)?;
            if devices.len() == 0 {
                info!("No devices found");
                return Ok(())
            }

            info!("Found devices: {:?}", devices);
        },
        Commands::Info => {
            let mut devices = trezor_client::find_devices(args.trezor_debug)?;

            if args.index >= devices.len() {
                warn!("No device for index {} ({} devices)", args.index, devices.len());
                return Ok(())
            }

            let mut device = devices.remove(args.index).connect()?;
            device.init_device(None)?;

            debug!("Device {:?}", device.model());

            if let Some(f) = device.features() {
                info!("Features:");
                info!("vendor: {}", f.get_vendor());
                info!(
                    "version: {}.{}.{}",
                    f.get_major_version(),
                    f.get_minor_version(),
                    f.get_patch_version()
                );
                info!("device id: {}", f.get_device_id());
                info!("label: {}", f.get_label());
                info!("is initialized: {}", f.get_initialized());
                info!("pin protection: {}", f.get_pin_protection());
                info!("passphrase protection: {}", f.get_passphrase_protection());
            }
        }
		Commands::External(_ext) => {
			todo!("call out to coin subcommands")
		},
		_ => todo!("Unhandled command: {:?}", args.command),
	}

	Ok(())
}
