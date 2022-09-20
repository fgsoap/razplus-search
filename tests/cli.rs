use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("razplus-search").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("razplus-search"));
}

#[test]
fn test_love() {
    let mut cmd = Command::cargo_bin("razplus-search").unwrap();
    cmd.arg("-n 'love'")
        .assert()
        .success()
        .stdout(predicate::str::contains("El Día de San Valentin"));
}
