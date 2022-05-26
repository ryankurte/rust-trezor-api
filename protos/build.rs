//! Compile protobuf files from trezor common

fn main() -> anyhow::Result<()> {
	// Rebuild on proto file changes
	println!("cargo:rerun-if-changed=../vendor/trezor-common/protob/*.proto");

	// Glob for proto files
	let files: Vec<_> =
		glob::glob("../vendor/trezor-common/protob/*.proto")?.filter_map(|f| f.ok()).collect();

	// Build proto files
	prost_build::compile_protos(&files, &["../vendor/trezor-common/protob/"])?;

	Ok(())
}
