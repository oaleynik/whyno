use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn temp_data_path() -> (TempDir, String) {
    let temp_dir = tempfile::tempdir().unwrap();
    let data_path = temp_dir.path().join("wines.json");
    (temp_dir, data_path.to_str().unwrap().to_string())
}

#[test]
fn test_add_and_list() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

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
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

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
fn test_add_and_update_richer_fields() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "add",
            "Cellar Wine",
            "--price",
            "42.50",
            "--purchase-date",
            "2024-06-01",
            "--drink-by",
            "2030-01-01",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Price: 42.50"))
        .stdout(predicate::str::contains("Purchase date: 2024-06-01"))
        .stdout(predicate::str::contains("Drink by: 2030-01-01"));

    Command::cargo_bin("whyno")
        .unwrap()
        .args([
            "--data",
            data_path,
            "update",
            "1",
            "--price",
            "50",
            "--drink-by",
            "2032-01-01",
        ])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "show", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Price: 50.00"))
        .stdout(predicate::str::contains("Drink by: 2032-01-01"));
}

#[test]
fn test_remove_missing_id() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "remove", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_invalid_rating() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "Bad Rating", "--rating", "99"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Rating must be between 1 and 5"));
}

#[test]
fn test_corrupt_json() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    std::fs::write(data_path, "not json").unwrap();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse"));
}

#[test]
fn test_empty_name() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "", "--rating", "4"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("name cannot be empty"));
}

#[test]
fn test_update_wine() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "Update Test", "--rating", "3"])
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
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

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
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "High Rating", "--rating", "5"])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "Low Rating", "--rating", "2"])
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
fn test_stats_empty() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "stats"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No wines saved yet"));
}

#[test]
fn test_stats_with_unrated_wines() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "Unrated Wine"])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "stats"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Total wines saved: 1"))
        .stdout(predicate::str::contains("Average rating: N/A"));
}

#[test]
fn test_stats_with_wines() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "Wine A", "--rating", "4"])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "add", "Wine B", "--rating", "2"])
        .assert()
        .success();

    Command::cargo_bin("whyno")
        .unwrap()
        .args(["--data", data_path, "stats"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Total wines saved: 2"))
        .stdout(predicate::str::contains("Average rating: 3.00"));
}

#[test]
fn test_tags() {
    let (_temp_dir, data_path) = temp_data_path();
    let data_path = data_path.as_str();

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
