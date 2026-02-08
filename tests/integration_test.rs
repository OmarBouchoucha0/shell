use std::process::Command;

#[test]
fn test_shell_compiles() {
    let output = Command::new("cargo")
        .args(&["build"])
        .output()
        .expect("Failed to execute cargo build");

    assert!(output.status.success(), "Shell should compile successfully");
}

#[test]
fn test_shell_binary_exists() {
    let debug_path = if cfg!(target_os = "windows") {
        "target/debug/shell.exe"
    } else {
        "target/debug/shell"
    };

    let release_path = if cfg!(target_os = "windows") {
        "target/release/shell.exe"
    } else {
        "target/release/shell"
    };
    assert!(
        std::path::Path::new(debug_path).exists() || std::path::Path::new(release_path).exists(),
        "Neither debug nor release shell binary exists.\nExpected:\n  - {}\n  - {}",
        debug_path,
        release_path
    );
}
