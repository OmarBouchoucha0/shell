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
    let binary_path = if cfg!(target_os = "windows") {
        "target/debug/shell.exe"
    } else {
        "target/debug/shell"
    };

    assert!(
        std::path::Path::new(binary_path).exists(),
        "Shell binary should exist at {}",
        binary_path
    );
}
