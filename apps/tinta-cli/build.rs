//! Windows only: embed version info (VERSIONINFO) into `tinta-cli.exe`, so Explorer's
//! Details tab, `(Get-Item tinta-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for `tinta.exe` (`apps/tinta/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PDFCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PDFCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "Tinta")
        .set("FileDescription", "Tinta command-line tool")
        .set("LegalCopyright", "Copyright (c) the Tinta contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "tinta-cli.exe")
        .set("InternalName", "tinta-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("PDFCRAFT_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=tinta-cli.exe built without version resources: {e}");
    }
}
