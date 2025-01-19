fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-arg-bin=elevator=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bin=elevator=/MANIFESTUAC:level=\'requireAdministrator\'");
    }
}
