use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_add_and_list() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "Test Wine", "--rating", "4"])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Wine"));
}

#[test]
fn test_add_and_show() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "Show Test Wine",
            "--rating",
            "5",
            "--notes",
            "Great wine",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Show Test Wine"))
        .stdout(predicate::str::contains("Great wine"));
}

#[test]
fn test_remove_missing_id() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "remove", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_invalid_rating() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "Bad Rating",
            "--rating",
            "99",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Rating must be between 1 and 5"));
}

#[test]
fn test_corrupt_json() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    std::fs::write(temp_file.path(), "not json").unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse"));
}

#[test]
fn test_empty_name() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "", "--rating", "4"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("name cannot be empty"));
}

#[test]
fn test_update_wine() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "Update Test",
            "--rating",
            "3",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "update", "1", "--rating", "5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated"));

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5/5"));
}

#[test]
fn test_filter_by_country() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "French Wine",
            "--country",
            "France",
            "--rating",
            "4",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "Italian Wine",
            "--country",
            "Italy",
            "--rating",
            "3",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "list", "--country", "france"])
        .assert()
        .success()
        .stdout(predicate::str::contains("French Wine"))
        .stdout(predicate::str::contains("Italian Wine").not());
}

#[test]
fn test_filter_by_min_rating() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "High Rating",
            "--rating",
            "5",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "Low Rating",
            "--rating",
            "2",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "list", "--min-rating", "4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("High Rating"))
        .stdout(predicate::str::contains("Low Rating").not());
}

#[test]
fn test_tags() {
    let temp_file = NamedTempFile::new().unwrap();
    let data_path = temp_file.path().to_str().unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "Tagged Wine",
            "--rating",
            "4",
            "--tag",
            "favorite",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("favorite"));
}