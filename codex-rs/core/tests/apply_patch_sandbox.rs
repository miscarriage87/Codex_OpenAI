use codex_apply_patch::ApplyPatchAction;
use codex_core::codex::apply_patch_with_sandbox;
use codex_core::protocol::SandboxPolicy;
use tempfile::TempDir;

#[test]
fn patch_allowed_when_within_policy() {
    let dir = TempDir::new().unwrap();
    let cwd = dir.path();
    let policy = SandboxPolicy::new_read_only_policy_with_writable_roots(&[cwd.to_path_buf()]);
    let path = cwd.join("hello.txt");
    let patch = ApplyPatchAction::new_add_for_test(&path, "hi".to_string());
    apply_patch_with_sandbox(&patch, &policy, cwd).unwrap();
    assert_eq!(std::fs::read_to_string(path).unwrap(), "hi");
}

#[test]
fn patch_denied_outside_policy() {
    let dir = TempDir::new().unwrap();
    let cwd = dir.path();
    let policy = SandboxPolicy::new_read_only_policy();
    let path = cwd.join("hello.txt");
    let patch = ApplyPatchAction::new_add_for_test(&path, "hi".to_string());
    let result = apply_patch_with_sandbox(&patch, &policy, cwd);
    assert!(result.is_err());
    assert!(!path.exists());
}
