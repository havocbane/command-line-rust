use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn help_message() -> TestResult {
    Command::cargo_bin("echor")?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn version() -> TestResult {
    Command::cargo_bin("echor")?
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("echor ".to_owned() + env!("CARGO_PKG_VERSION")));
    Ok(())
}

#[test]
fn hello_world() -> TestResult {
    Command::cargo_bin("echor")?
        .arg("Hello, World!")
        .assert()
        .success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    // Run the program with a single string value as input
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    // Run the program with two string values as input
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    // Run the program with a single string value as input and the -n flag to omit the final newline
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    // Run the program with two strings as input and the -n flag appearing first
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
