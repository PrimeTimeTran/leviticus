use std::path::PathBuf;

pub fn leviticus_root() -> std::path::PathBuf {
    dirs::home_dir().unwrap().join(".leviticus")
}

pub fn project_root() -> std::path::PathBuf {
    std::env::current_dir().unwrap()
}

pub fn leviticus_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".leviticus")
}
