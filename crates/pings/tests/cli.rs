//! Integration tests: run the real `pings` binary the way a user would.
//! Cargo builds it first and exposes its path as CARGO_BIN_EXE_<name>.

use std::process::{Command, Output};

fn pings(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pings"))
        .args(args)
        .output()
        .expect("failed to run the pings binary")
}

#[test]
fn version_flag_prints_name_and_version() {
    let out = pings(&["--version"]);
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert_eq!(
        stdout.trim(),
        format!("pings {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn no_arguments_prints_help() {
    let out = pings(&[]);
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("USAGE:"));
}

#[test]
fn unknown_argument_is_a_usage_error() {
    let out = pings(&["--bogus"]);
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("unknown argument '--bogus'"));
}
