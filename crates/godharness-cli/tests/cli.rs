mod common;
use common::godharness;

#[test]
fn init_exits_successfully() {
    let output = godharness()
        .arg("init")
        .output()
        .expect("run godharness init");

    assert!(output.status.success());
}

#[test]
fn check_exits_successfully() {
    let output = godharness()
        .arg("check")
        .output()
        .expect("run godharness check");

    assert!(output.status.success());
}

#[test]
fn doctor_exits_successfully() {
    let output = godharness()
        .arg("doctor")
        .output()
        .expect("run godharness doctor");

    assert!(output.status.success());
}

#[test]
fn context_prints_the_must_read_standards_as_json() {
    let output = godharness()
        .arg("context")
        .arg("--prompt")
        .arg("add a new endpoint")
        .output()
        .expect("run godharness context");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    let parsed: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout should be valid JSON");

    let entries = parsed.as_array().expect("value should be an array");
    let ids: Vec<&str> = entries
        .iter()
        .map(|entry| entry["id"].as_str().expect("id should be a string"))
        .collect();

    assert_eq!(
        ids,
        vec![
            "secrets-and-security",
            "simplify-before-done",
            "verify-through-real-path",
            "leave-code-cleaner-than-you-found-it",
            "concise-communication",
        ]
    );

    let secrets_and_security = &entries[0];
    assert_eq!(secrets_and_security["id"], "secrets-and-security");
    assert_eq!(
        secrets_and_security["path"],
        "standards/secrets-and-security.md"
    );
    assert!(
        secrets_and_security["rule"]
            .as_str()
            .is_some_and(|rule| !rule.is_empty())
    );
    assert_eq!(
        secrets_and_security["relates-to"],
        serde_json::json!(["configuration-boundaries"])
    );
}

#[test]
fn context_accepts_multiple_paths() {
    let output = godharness()
        .arg("context")
        .arg("--paths")
        .arg("src/main.rs")
        .arg("src/lib.rs")
        .output()
        .expect("run godharness context");

    assert!(output.status.success());
}
