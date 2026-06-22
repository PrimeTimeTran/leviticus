use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

fn read_cargo_toml() -> CargoToml {
    let raw = fs::read_to_string("Cargo.toml").expect("failed to read Cargo.toml");

    toml::from_str(&raw).expect("invalid Cargo.toml")
}

fn write_cargo_toml(cfg: &CargoToml) {
    let toml_str = toml::to_string(cfg).unwrap();

    fs::write("Cargo.toml", toml_str).expect("failed to write Cargo.toml");
}

pub fn sync_project_metadata() {
    let mut cargo = read_cargo_toml();

    cargo.package.version = "0.1.1".to_string();

    write_cargo_toml(&cargo);
}

pub fn derive_runtime_context() {
    let cargo = read_cargo_toml();

    println!("leviticus running in project:");
    println!("  name: {}", cargo.package.name);
    println!("  version: {}", cargo.package.version);
    println!("  edition: {}", cargo.package.edition);

    // later: drive behavior here
}
