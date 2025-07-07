use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_no_args() {
    Command::cargo_bin("ls_plus").unwrap().assert().success();
}

#[test]
fn test_version() {
    Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_invalid_path() {
    Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--path")
        .arg("non_existent_directory")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Path does not exist"));
}

#[test]
fn test_formats() {
    let formats = ["table", "long", "tree", "json"];
    for format in formats {
        Command::cargo_bin("ls_plus")
            .unwrap()
            .arg("--format")
            .arg(format)
            .assert()
            .success();
    }
}

#[test]
fn test_human_readable() {
    Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--human-readable")
        .assert()
        .success()
        .stdout(predicate::str::contains("KB").or(predicate::str::contains("B")));
}

#[test]
fn test_file_filtering() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files
    File::create(temp_dir.path().join("test.txt")).unwrap();
    File::create(temp_dir.path().join("test.rs")).unwrap();
    std::fs::create_dir(temp_dir.path().join("test_dir")).unwrap();

    // Test extension filter
    Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--extensions")
        .arg("txt")
        .arg("--no-color") // Disable color for consistent output
        .assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        .stdout(predicate::str::contains("test.rs").not());

    // Test dirs only
    Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--dirs-only")
        .arg("--no-color") // Disable color for consistent output
        .assert()
        .success()
        .stdout(predicate::str::contains("test_dir"))
        .stdout(predicate::str::contains("test.txt").not());

    // Test files only
    Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--files-only")
        .arg("--no-color") // Disable color for consistent output
        .assert()
        .success()
        .stdout(predicate::str::contains("test.txt"))
        .stdout(predicate::str::contains("test_dir").not());
}

#[test]
fn test_sorting() {
    let temp_dir = TempDir::new().unwrap();

    // Create test files with different sizes
    let mut file1 = File::create(temp_dir.path().join("a.txt")).unwrap();
    file1.write_all(&[0; 100]).unwrap();

    let mut file2 = File::create(temp_dir.path().join("b.txt")).unwrap();
    file2.write_all(&[0; 200]).unwrap();

    // Test size sorting (descending)
    let output = Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--path")
        .arg(temp_dir.path())
        .arg("--sort")
        .arg("size")
        .arg("--order")
        .arg("desc")
        .arg("--no-color") // Disable color for consistent output
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();

    // Check that b.txt (200 bytes) appears before a.txt (100 bytes)
    let b_txt_index = lines
        .iter()
        .position(|&line| line.contains("b.txt"))
        .unwrap();
    let a_txt_index = lines
        .iter()
        .position(|&line| line.contains("a.txt"))
        .unwrap();
    assert!(
        b_txt_index < a_txt_index,
        "Files not sorted correctly by size"
    );
}

#[test]
fn test_no_color() {
    Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--no-color")
        .assert()
        .success();
}

#[test]
fn test_summary() {
    Command::cargo_bin("ls_plus")
        .unwrap()
        .arg("--summary")
        .assert()
        .success()
        .stdout(predicate::str::contains("Summary"))
        .stdout(predicate::str::contains("Files:"))
        .stdout(predicate::str::contains("Directories:"))
        .stdout(predicate::str::contains("Total Size:"));
}
