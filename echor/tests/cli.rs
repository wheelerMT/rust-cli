use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run_with_args(args: &[&str], expected_file: &str) -> TestResult {
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
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn test_successfully_runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();

    Ok(())
}

#[test]
fn test_hello_newline() -> TestResult {
    run_with_args(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn test_hello_split_args() -> TestResult {
    run_with_args(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn test_hello_no_newline() -> TestResult {
    run_with_args(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn test_hello_split_args_no_newline() -> TestResult {
    run_with_args(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}