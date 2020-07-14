extern crate lalrpop;
extern crate toml;

use toml::{from_str, Value};

fn main() {
    // Preprocess lalrpop grammar files
    lalrpop::process_root().unwrap();

    // Find git revision of current version, if possible
    let revision_str = if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
    {
        let prefix = &output.stdout[..8];
        String::from_utf8(prefix.to_vec()).unwrap()
    } else {
        " release".to_owned()
    };
    println!("cargo:rustc-env=REVISION={}", revision_str);
    // The .git directory mtime should change if something is commited, so we rerun the build
    // script in that case to update the revision.
    println!("cargo:rerun-if-changed=.git");

    // Find current release version (crate version specified in Cargo.toml)
    let file_str = include_str!("Cargo.toml");
    let config: Value = from_str(file_str).unwrap();
    let version_str = config.as_table().unwrap()["package"].as_table().unwrap()["version"]
        .as_str()
        .unwrap();
    println!("cargo:rustc-env=CRATE_VERSION={}", version_str);
}
