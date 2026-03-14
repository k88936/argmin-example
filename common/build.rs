use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto = manifest_dir.join("../common/proto/helloworld.proto");

    println!("cargo:rerun-if-changed={}", proto.display());

    tonic_prost_build::compile_protos(proto)?;
    Ok(())
}
