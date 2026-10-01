// Modified for Agents Go by Agents Go contributors, 2026-10-01. See NOTICE.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_CHANNEL");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_ID");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_COMMIT");
    println!(
        "cargo:warning=Agents Go is based on Herdr; read CONTRIBUTING.md before contributing."
    );
}
