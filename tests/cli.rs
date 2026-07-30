use assert_cmd::cargo::*;
use assert_fs::prelude::FileWriteStr;
// Import cargo_bin_cmd! macro and methods
use predicates::prelude::*; // Used for writing assertions

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("pwtool");

    cmd.arg("--c")
        .arg("test/file/doesnt/exist")
        .arg("get")
        .arg("--P")
        .arg("test");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file_general_case() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("search_file = \"test-sample.json\"")?;
    let mut cmd = cargo_bin_cmd!("pwtool");

    cmd.arg("--c")
        .arg(file.path())
        .arg("get")
        .arg("--P")
        .arg("my-school");
    cmd.assert().success().stdout(predicate::str::contains(
        "Found one entry for search argument",
    ));

    Ok(())
}

#[test]
fn find_content_in_file_when_empty_string() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    let content = "search_file = \"test-sample.json\"";
    let result = "Found multiple entries for search argument";
    file.write_str(content)?;
    let mut cmd = cargo_bin_cmd!("pwtool");

    cmd.arg("--c")
        .arg(file.path())
        .arg("get")
        .arg("--P")
        .arg("");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(result));

    Ok(())
}

#[test]
fn set_content_when_missing_fields() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.toml")?;
    let content = "search_file = \"test-sample.json\"";
    file.write_str(content)?;
    let mut cmd = cargo_bin_cmd!("pwtool");

    cmd.arg("--c")
        .arg(file.path())
        .arg("set")
        .arg("--k")
        .arg("key")
        .arg("--u")
        .arg("username");

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided:",
    ));

    Ok(())
}

#[test]
fn set_content_in_file_non_unique_key() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    let content = "search_file = \"test-sample.json\"";
    let result = "Found multiple entries for search argument";
    file.write_str(content)?;
    let mut cmd = cargo_bin_cmd!("pwtool");

    cmd.arg("--c")
        .arg(file.path())
        .arg("set")
        .arg("--k")
        .arg("my-work")
        .arg("--u")
        .arg("username")
        .arg("--p")
        .arg("password");

    cmd.assert()
        //Tämän ei pitäisi olla success
        .success()
        .stdout(predicate::str::contains(
            "Entry with key:my-work already exists",
        ));

    Ok(())
}

#[test]
fn set_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    let pw_entry_file = assert_fs::NamedTempFile::new("test-entries.json")?;
    let content = format!("search_file = \"{}\"", &pw_entry_file.path().display());
    file.write_str(&content)?;
    pw_entry_file.write_str("[]")?;
    let mut cmd = cargo_bin_cmd!("pwtool");

    cmd.arg("--c")
        .arg(file.path())
        .arg("set")
        .arg("--k")
        .arg("my-new")
        .arg("--u")
        .arg("username")
        .arg("--p")
        .arg("password");

    cmd.assert().success().stdout(predicate::str::contains(
        "Successfully saved new credentials for key 'my-new'",
    ));

    Ok(())
}
