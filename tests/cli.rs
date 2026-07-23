use assert_cmd::cargo::*; use assert_fs::prelude::FileWriteStr;
// Import cargo_bin_cmd! macro and methods
use predicates::prelude::*; // Used for writing assertions

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("pwtool");

    cmd.arg("foobar").arg("test/file/doesnt/exist");
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

    cmd.arg("my-school").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("Found one entry for search argument"));

    Ok(())
}


#[test]
fn find_content_in_file_when_empty_string() -> Result<(), Box<dyn std::error::Error>> {
    
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    let content = "search_file = \"test-sample.json\"";
    let result = "Found multiple entries for search argument";
    file.write_str(content)?;
    let mut cmd = cargo_bin_cmd!("pwtool");


    cmd.arg("").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(result));

    Ok(())
}