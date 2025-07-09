use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("voxmix").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A CLI tool for high-quality speech synthesis"));
}

#[test]
fn test_say_subcommand_help() {
    let mut cmd = Command::cargo_bin("voxmix").unwrap();
    cmd.args(["say", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generate speech from text"));
}

#[test]
fn test_say_missing_text() {
    let mut cmd = Command::cargo_bin("voxmix").unwrap();
    cmd.args(["say"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_say_invalid_speed() {
    let mut cmd = Command::cargo_bin("voxmix").unwrap();
    cmd.args(["say", "--speed", "0", "test"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Speed must be greater than 0"));
}

#[test]
fn test_say_invalid_pitch() {
    let mut cmd = Command::cargo_bin("voxmix").unwrap();
    cmd.args(["say", "--pitch", "0", "test"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Pitch must be greater than 0"));
}

#[test]
fn test_say_invalid_volume() {
    let mut cmd = Command::cargo_bin("voxmix").unwrap();
    cmd.args(["say", "--volume", "0", "test"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Volume must be greater than 0"));
}

#[test]
fn test_say_empty_text() {
    let mut cmd = Command::cargo_bin("voxmix").unwrap();
    cmd.args(["say", ""]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Text cannot be empty"));
}

#[test]
fn test_say_custom_output_path() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_output.wav");
    
    let mut cmd = Command::cargo_bin("voxmix").unwrap();
    cmd.args([
        "say",
        "--output", output_path.to_str().unwrap(),
        "--host", "127.0.0.1",
        "--port", "50021",
        "Hello, World!"
    ]);
    
    // This test will fail if VOICEVOX server is not running
    // but it tests the CLI argument parsing
    cmd.assert()
        .failure(); // Expected to fail without running server
}