//! The project profile consumed by ai-stp must describe vendor-loaded paths.

#![allow(
    clippy::expect_used,
    reason = "integration fixtures require Cargo's binary"
)]

use serde_json::Value;

fn owns(namespaces: &[Value], path: &str) -> bool {
    namespaces.iter().filter_map(Value::as_str).any(|root| {
        path == root
            || path
                .strip_prefix(root)
                .is_some_and(|tail| tail.starts_with('/'))
    })
}

#[test]
fn public_project_profile_routes_supported_kinds_without_claiming_runtime_or_credentials() {
    let executable = std::env::var("CARGO_BIN_EXE_opencode-setup-system")
        .expect("Cargo must expose the OpenCode installation component");
    let info = harness_runtime::probe::provider_info(std::path::Path::new(&executable))
        .expect("the built installation component must return public provider-info JSON");
    let profiles = info["scoped_projection_profiles"]
        .as_array()
        .expect("scoped profiles must be exposed");
    let projects: Vec<_> = profiles
        .iter()
        .filter(|profile| profile["target_scope"] == "project")
        .collect();
    assert_eq!(
        projects.len(),
        1,
        "OpenCode must expose one project profile before ai-stp can compile a project bundle"
    );
    let profile = projects[0];
    assert_ne!(
        profile["profile_id"],
        info["projection_profile"]["profile_id"]
    );
    assert_ne!(profile["digest"], info["projection_profile"]["digest"]);
    let kinds = profile["component_kinds"]
        .as_array()
        .expect("component kinds");
    let namespaces = profile["native_namespaces"]
        .as_array()
        .expect("native namespaces");
    for (kind, path) in [
        ("instruction", "AGENTS.md"),
        ("skill", ".opencode/skills/review/SKILL.md"),
        ("agent", ".opencode/agents/review.md"),
        ("plugin", ".opencode/plugins/review.ts"),
        ("command", ".opencode/commands/review.md"),
        ("setting", "opencode.json"),
    ] {
        assert!(kinds.iter().any(|value| value == kind), "{kind} is absent");
        assert!(owns(namespaces, path), "project route {path} is absent");
    }
    for path in [
        ".opencode/auth.json",
        ".opencode/cache/keep",
        ".opencode/node_modules/keep",
        ".opencode/package.json",
        ".opencode/skills-neighbour/keep.md",
        ".agents/skills/someone-else/SKILL.md",
        "auth.json",
        "src/main.rs",
        "opencode.jsonc",
        "tui.json",
    ] {
        assert!(!owns(namespaces, path), "project profile claims {path}");
    }
}
