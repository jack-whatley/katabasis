fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/elevator.proto")?;

    #[cfg(all(windows, feature = "winAdmin"))]
    {
        println!("cargo:rustc-link-arg-bin=elevator=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bin=elevator=/MANIFESTUAC:level=\'requireAdministrator\'");
    }

    Ok(())
}
