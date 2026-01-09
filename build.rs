fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=migrations");

    tonic_prost_build::configure()
        .build_client(false)
        .out_dir("src/gen")
        .compile_protos(&["proto/auth.proto"], &["proto/"])?;
    Ok(())
}
