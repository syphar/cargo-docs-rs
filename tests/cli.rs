use assert_cmd::Command;
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn resolve_crate_name(package_name: &str) -> String {
    package_name.replace("-", "_")
}

fn check_docs_build(package_name: &str, triple: &str) {
    let crate_dir = fixture(package_name);
    let target_dir = tempfile::tempdir().unwrap();

    Command::cargo_bin("cargo-docs-rs")
        .unwrap()
        .current_dir(&crate_dir)
        .env("CARGO_TARGET_DIR", target_dir.path())
        .env_remove("RUSTUP_TOOLCHAIN")
        .env_remove("CARGO")
        .arg("docs-rs")
        .assert()
        .success();

    // docs.rs always builds for a specific target triple, so output lands under
    // `target/<triple>/doc/<crate>/index.html`.
    let index = target_dir
        .path()
        .join(triple)
        .join("doc")
        .join(resolve_crate_name(package_name))
        .join("index.html");
    assert!(index.exists(), "expected rustdoc output at {index:?}");
}

#[test]
fn builds_docs_for_hello_lib() {
    check_docs_build("hello-lib", target_tuple::HOST);
}

#[test]
fn builds_docs_for_wasm_lib() {
    check_docs_build("wasm-lib", "wasm32-unknown-unknown");
}
