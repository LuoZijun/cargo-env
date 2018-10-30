#[macro_use]
extern crate clap;
extern crate colored;

use clap::{App, AppSettings, SubCommand};
use colored::*;
use std::env::var;

const RUST_VARS: &[&str] = &[
    "RUST_TEST_THREADS",
    "RUST_TEST_TASKS",
    "RUST_TEST_STRESS",
    "RUST_TEST_NOCAPTURE",
    "RUST_TEST_SPAWN_UNICODE",
    "RUST_TEST_PROC_SPAWN_UNICODE",
    "RUST_LOG",
    "RUST_BUILD_STAGE",
    "RUST_MAX_CACHED_STACKS",
    "RUST_MIN_STACK",
    "RUST_PATH",
    "RUST_URL",
    "DXR_RUST_TEMP_FOLDER",
    "RUST_BACKTRACE",
    "RUST_THREADS",
    "RUST_BENCH",
    "RUST_POISON_ON_FREE",
    "RUST_NEWRT",
    "RUST_PACKAGE_NAME",
    "RUST_PACKAGE_NAME_AND_TRIPLE",
    "RUST_TARBALL_NAME",
    "RUST_LOCAL_INSTALL_DIR",
    "RUST_LOCAL_INSTALL_SCRIPT",
    "RUST_DOWNLOAD_DIR",
    "RUST_LOCAL_TARBALL",
    "RUST_REGION_GRAPH_NODE",
    "RUST_REGION_GRAPH",
    "RUST_TARGET_PATH",
    "RUST_OBJECT",
    "R_RUST_TEMP_FOLDER",
    "RUST_TEST_STRESS",
    "RUST_ANDROID_DUMMY_H",
    "RUST_BENCH",
    "RUST_HOPEFULLY_THIS_DOESNT_EXIST",
    "RUST_NEWRT",
    "RUST_POISON_ON_FREE",
    "RUST_SRC_PATH",
];

const CARGO_VARS: &[&str] = &[
    "CARGO",
    "CARGO_HOME",
    "CARGO_TARGET_DIR",
    "RUSTC",
    "PROFILE",
    "OPT_LEVEL",
    "DEBUG",
    "RUSTDOC",
    "RUSTC_WRAPPER",
    "RUSTDOCFLAGS",
    "RUSTFLAGS",
    "CARGO_INCREMENTAL",
    "CARGO_CACHE_RUSTC_INFO",
    "CARGO_PKG_VERSION",
    "CARGO_MANIFEST_DIR",
    "CARGO_PKG_VERSION_MAJOR",
    "CARGO_PKG_VERSION_MINOR",
    "CARGO_PKG_VERSION_PATCH",
    "CARGO_PKG_VERSION_PRE",
    "CARGO_PKG_AUTHORS",
    "CARGO_PKG_NAME",
    "CARGO_PKG_DESCRIPTION",
    "CARGO_PKG_HOMEPAGE ",
    "OUT_DIR",
    "TARGET",
    "HOST",
    "CARGO_MANIFEST_LINKS",
    "NUM_JOBS",
    "RUSTC_LINKER",
];

const RUSTUP_VARS: &[&str] = &[
    "RUSTUP_HOME",
    "RUSTUP_TOOLCHAIN",
    "RUSTUP_DIST_SERVER",
    "RUSTUP_DIST_ROOT",
    "RUSTUP_UPDATE_ROOT",
];

fn main() {
    let matches = App::new("cargo")
        .bin_name("cargo")
        .version(crate_version!())
        .about("cargo-env for rust environment variables")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("env").version(crate_version!()))
        .get_matches();
    if matches.subcommand_matches("env").is_some() {
        print_vars("Rust", RUST_VARS);
        print_vars("Cargo", CARGO_VARS);
        print_vars("Rustup", RUSTUP_VARS);
    }
}

fn print_vars(name: &str, vars: &[&str]) {
    let line = format!(
        "---------------- {} Environment Variables -------------",
        name
    );
    println!("{}", line.green().bold());
    for v in vars {
        if let Ok(out) = var(v) {
            println!("{} {} {}", v.blue(), "=".blue(), out);
        } else {
            println!("{} {}", v.blue(), "=".blue());
        }
    }
}
