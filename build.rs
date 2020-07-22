fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compiling protos using path on build time
    // tonic_build::compile_protos("proto/dot_service.proto")?;

    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/dot_service.proto"], &["proto"])
        .expect("Compiled");
    Ok(())
}
