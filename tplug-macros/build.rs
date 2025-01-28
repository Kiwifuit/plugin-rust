use anyhow::Context;
use rustc_version::version;

fn main() -> anyhow::Result<()> {
    let rustc_version = version().context("while fetching rustc version")?;
    let rustc_version_major = rustc_version.major;
    let rustc_version_minor = rustc_version.minor;
    let rustc_version_patch = rustc_version.patch;

    println!(
        "cargo::rustc-env=RUSTC_VERSION_MAJOR={}",
        rustc_version_major
    );
    println!(
        "cargo::rustc-env=RUSTC_VERSION_MINOR={}",
        rustc_version_minor
    );
    println!(
        "cargo::rustc-env=RUSTC_VERSION_PATCH={}",
        rustc_version_patch
    );
    println!(
        "cargo::rustc-env=RUSTC_VERSION={}.{}.{}",
        rustc_version_major, rustc_version_minor, rustc_version_patch
    );

    Ok(())
}
