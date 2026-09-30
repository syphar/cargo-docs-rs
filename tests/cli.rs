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

#[test]
fn experimental_denies_invalid_html_tags() {
    let crate_dir = fixture("invalid-html-tags");
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

    Command::cargo_bin("cargo-docs-rs")
        .unwrap()
        .current_dir(crate_dir)
        .env("CARGO_TARGET_DIR", target_dir.path())
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env_remove("CARGO")
        .arg("docs-rs")
        .arg("--experimental")
        .assert()
        .failure()
        .stderr(predicates::str::contains("unclosed HTML tag `div`"));
}
