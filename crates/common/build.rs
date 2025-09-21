use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // Configure tonic-build
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("voip_descriptor.bin"))
        // Temporarily disabled serde for proto types due to prost_types incompatibility
        // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[
                "proto/common.proto",
                "proto/sip.proto",
                "proto/media.proto",
                "proto/routing.proto",
                "proto/provisioning.proto",
                "proto/auth.proto",
                "proto/monitoring.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}