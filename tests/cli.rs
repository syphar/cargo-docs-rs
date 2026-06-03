use assert_cmd::Command;
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn builds_docs_for_hello_lib() {
    let crate_dir = fixture("hello-lib");
    let target_dir = tempfile::tempdir().unwrap();

    Command::cargo_bin("cargo-docs-rs")
        .unwrap()
        .current_dir(&crate_dir)
        .env("CARGO_TARGET_DIR", target_dir.path())
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env_remove("CARGO")
        .arg("docs-rs")
        .assert()
        .success();

    // docs.rs always builds for a specific target triple, so output lands under
    // `target/<triple>/doc/<crate>/index.html`.
    let index = target_dir
        .path()
        .join(target_triple::HOST)
        .join("doc")
        .join("hello_lib")
        .join("index.html");
    assert!(index.exists(), "expected rustdoc output at {index:?}");
}
