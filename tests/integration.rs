use std::fs;
use std::path::Path;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn help_command() {
    let mut cmd = cargo_bin_cmd!("kneeboard");
    let assert = cmd.arg("--help").assert();
    assert
        .success()
        .stderr("")
        .stdout(predicate::str::contains("Checklist Tool"))
        .stdout(predicate::str::contains(
            "Usage: kneeboard [OPTIONS] --checklist-path <CHECKLIST_PATH>",
        ))
        .stdout(predicate::str::contains("Options:"))
        .stdout(predicate::str::contains(
            "-c, --checklist-path <CHECKLIST_PATH>",
        ))
        .stdout(predicate::str::contains("Path to the checklist"))
        .stdout(predicate::str::contains("-s, --save"))
        .stdout(predicate::str::contains(
            "Save and load progress of the checklist",
        ))
        .stdout(predicate::str::contains("-v, --verbose..."))
        .stdout(predicate::str::contains("Turn debugging information on"))
        .stdout(predicate::str::contains("-h, --help"))
        .stdout(predicate::str::contains("Print help"))
        .stdout(predicate::str::contains("-V, --version"))
        .stdout(predicate::str::contains("Print version"))
        .stdout(predicate::str::contains("--headless"))
        .stdout(predicate::str::contains("Headless mode"));
}

#[test]
fn loading_test_checklist() {
    let mut cmd = cargo_bin_cmd!("kneeboard");
    let assert = cmd
        .arg("-vvv")
        .arg("--headless")
        .arg("--checklist-path")
        .arg("__test__/test_checklists/test1.md")
        .assert();
    assert.code(7);
}

#[test]
fn loading_test_checklist_and_saving() {
    let save_path = Path::new("__test__/test_checklists/.5bdafbac94e71e2f.kb.toml");
    if save_path.exists() {
        fs::remove_file(save_path).expect("failed to remove existing save file");
    }

    let mut cmd = cargo_bin_cmd!("kneeboard");
    let assert = cmd
        .arg("-vvv")
        .arg("--save")
        .arg("--headless")
        .arg("--checklist-path")
        .arg("__test__/test_checklists/test2.md")
        .assert();
    assert.code(7);

    let save_path = Path::new("__test__/test_checklists/.5bdafbac94e71e2f.kb.toml");
    assert!(
        save_path.exists(),
        "expected save file does not exist: {:?}",
        save_path
    );

    let content = fs::read_to_string(save_path).expect("failed to read saved toml file");
    let value: toml::Value = toml::from_str(&content).expect("invalid toml in save file");

    let name = value
        .get("name")
        .and_then(|v| v.as_str())
        .expect("missing or non-string 'name' in toml");
    assert_eq!(name, "Test Checklist 2");

    let items = value
        .get("items")
        .and_then(|v| v.as_array())
        .expect("missing or non-array 'items' in toml");
    assert_eq!(items.len(), 8);

    let first_text = items[0]
        .get("text")
        .and_then(|v| v.as_str())
        .expect("missing or non-string 'text' on first item");
    assert_eq!(first_text, "Test Checklist 1 normal item");
}

#[test]
fn loading_test_checklist_and_saving_differing_save_and_mark() {
    // For this test the save file is modified to contain less items than the original checklist
    // This should trigger the merge logic to add the missing items
    // Exit code should be 10
    let mut cmd = cargo_bin_cmd!("kneeboard");
    let assert = cmd
        .arg("-vvv")
        .arg("--save")
        .arg("--headless")
        .arg("--checklist-path")
        .arg("__test__/test_checklists/test3.md")
        .assert();
    assert.code(10);
}
