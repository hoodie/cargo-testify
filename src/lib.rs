extern crate notify;
extern crate regex;
extern crate notify_rust;
#[macro_use] extern crate error_chain;

mod errors;
mod report;
mod config;
mod reactor;
mod report_builder;
use config::ConfigBuilder;
use reactor::Reactor;


// TODO: implement filter for files like .git, /target, etc..
pub fn run() {
    let project_dir = detect_project_dir();

    let config = ConfigBuilder::new()
        .project_dir(project_dir)
        .build()
        .unwrap();

    Reactor::new(config).start()
}

/// Search for Cargo.toml file starting from the current directory,
/// going with every step to parent directory. If directory with
/// Cargo.toml is found return it, otherwise print error message and
/// terminate the process.
fn detect_project_dir() -> std::path::PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let mut optional_dir = Some(current_dir.as_path());

    while let Some(dir) = optional_dir {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.is_file() { return dir.to_path_buf(); }
        optional_dir = dir.parent();
    }

    eprintln!("Error: could not find `Cargo.toml` in {:?} or any parent directory.", current_dir);
    std::process::exit(1);
}

