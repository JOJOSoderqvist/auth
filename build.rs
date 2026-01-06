fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_client(false)
        .out_dir("src/gen")
        .compile_protos(&["proto/auth.proto"], &["proto/"])?;
    Ok(())
}
