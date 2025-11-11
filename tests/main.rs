use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn no_command_line_arguments() {
    cargo_bin_cmd!()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: No command specified"));
}

#[test]
fn help() {
    let cmd = cargo_bin_cmd!().arg("--help").unwrap();
    cmd.assert().success();
}
