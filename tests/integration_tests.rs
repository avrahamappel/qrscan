use std::path::PathBuf;

use assert_cmd::prelude::OutputOkExt;

fn qrscan() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("qrscan").unwrap()
}

pub struct TestFile {
    pub path: PathBuf,
}

impl TestFile {
    pub fn new(id: &str, ext: &str) -> Self {
        let path = PathBuf::from(format!("test_{id}.{}", ext));
        let header = format!("Accept: image/{}", ext);
        let data = format!("foo {}", ext);

        std::process::Command::new("curl")
            .arg("https://qrcode.show")
            .arg("-k")
            .arg("-H")
            .arg(&header)
            .arg("-d")
            .arg(&data)
            .arg("-o")
            .arg(&path)
            .unwrap();

        Self { path }
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path).unwrap();
    }
}

#[test]
fn test_help() {
    qrscan().arg("-h").assert().success();
    qrscan().arg("--help").assert().success();
}

#[test]
#[ignore = "the jpeg endpoint appears to be broken"]
fn test_scan_jpeg_file() {
    let file = TestFile::new("scan_jpeg_file", "jpeg");
    qrscan()
        .arg(&file.path)
        .assert()
        .success()
        .stdout("foo jpeg\n");
}

#[test]
fn test_scan_png_file() {
    let file = TestFile::new("scan_png_file", "png");
    qrscan()
        .arg(&file.path)
        .assert()
        .success()
        .stdout("foo png\n");
}

#[test]
fn test_scan_from_stdin() {
    let file = TestFile::new("scan_from_stdin", "png");
    qrscan()
        .arg("-")
        .pipe_stdin(&file.path)
        .unwrap()
        .assert()
        .success()
        .stdout("foo png\n");
}

#[test]
fn test_scan_no_content() {
    let file = TestFile::new("scan_no_content", "png");
    qrscan()
        .arg(&file.path)
        .arg("-n")
        .assert()
        .success()
        .stdout("");

    qrscan()
        .arg(&file.path)
        .arg("--no-content")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_export_files() {
    let file = TestFile::new("export_files", "png");
    qrscan()
        .arg(&file.path)
        .arg("--ascii")
        .arg("test.ascii")
        .arg("--svg")
        .arg("test.svg")
        .arg("--jpeg")
        .arg("test.jpeg")
        .arg("--png")
        .arg("test.png")
        .assert()
        .success()
        .stdout("foo png\n");

    assert!(PathBuf::from("test.ascii").exists());
    assert!(PathBuf::from("test.svg").exists());
    assert!(PathBuf::from("test.jpeg").exists());
    assert!(PathBuf::from("test.png").exists());

    std::fs::remove_file("test.ascii").unwrap();
    std::fs::remove_file("test.svg").unwrap();
    std::fs::remove_file("test.jpeg").unwrap();
    std::fs::remove_file("test.png").unwrap();
}

#[test]
fn test_err_1() {
    qrscan().arg("-").assert().failure().code(1);
}

#[test]
fn test_err_2() {
    qrscan().arg("/tmp").assert().failure().code(2);
}

#[test]
fn test_err_3() {
    qrscan()
        .arg("/foo/bar/doesntexists")
        .assert()
        .failure()
        .code(3);
}
