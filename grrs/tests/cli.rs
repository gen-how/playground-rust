use assert_cmd::cargo; // Used to get the path to the compiled binary.
use predicates::prelude::*; // Used for writing assertions.
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    // Gets the compiled binary target.
    let mut cmd = cargo::cargo_bin_cmd!("grrs");

    cmd.arg("foobar").arg("file/not/found");
    cmd.assert()
        .failure()
        // The error message may vary based on the operating system.
        // .stderr(predicate::str::contains("No such file or directory"));
        .stderr(predicate::str::contains("Could not read file:"));

    Ok(())
}

#[test]
fn test_find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo::cargo_bin_cmd!("grrs");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}
