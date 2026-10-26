//! The OpenCode setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with the other setup systems, so a change to
//! behaviour lands once and a change to OpenCode's surface lands here.
//!
//! Unlike its siblings, this harness's routes were not read off the product's
//! documentation alone: `ai-stp` already carries a composition table for
//! `opencode`, and the namespaces below are that table. Where a consumer has
//! already decided where a component goes, agreeing with it is not a shortcut --
//! disagreeing would make every bundle it sends land somewhere it will not look.

use std::process::ExitCode;

mod software;

#[cfg(test)]
mod project_tests;

use harness_runtime::{Harness, LaunchBinding, PreservationSurface, Scoped, Shadow};
use provider_v3::{ComponentKind, ProjectionKind, TargetScope};

/// Everything specific to OpenCode, verified against `opencode-baseline.json`.
pub const OPENCODE: Harness = Harness {
    harness_id: "opencode",
    provider_id: "opencode-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "OpenCode",
    vendor: "Anomaly",
    // XDG-based, and the only harness here whose home is not a dotted directory
    // under `$HOME`. The env override is what a caller should set; nothing is
    // ever inferred from either, because every command takes an explicit target.
    documented_config_home: "~/.config/opencode",
    config_home_env: "OPENCODE_CONFIG_DIR",
    // Measured 2026-08-28 by asking: `debug config` under this variable
    // resolved a `model` value present in no file but the target's own.
    launch_binding: LaunchBinding::Complete {
        how: "measured by asking the product which configuration it resolved",
    },
    // Measured 2026-08-31 in the 1.18.25 artifact. The automatic path
    // reads `if (autoupdate === false || OPENCODE_DISABLE_AUTOUPDATE) return;`
    // -- the key and the variable are alternatives, and only the variable is
    // reachable from a launch this provider controls.
    //
    // **It stops the automatic path and not the `upgrade` subcommand**, whose
    // handler does not consult it. That subcommand is a person typing, and it
    // warns before replacing bytes it cannot attribute to a package manager;
    // claude's `DISABLE_UPDATES` covers both and this one does not, so the two
    // entries in this estate do not mean the same thing.
    updates_off_env: "OPENCODE_DISABLE_AUTOUPDATE",
    // One home, one variable: nothing here is conditional.
    config_home_note: "",
    control_directory: ".opencode-setup-system",
    state_file: "NDDEV-OPENCODE-PROVIDER.json",
    predecessor_state_file: "NDDEV-OPENCODE-SETUP.json",
    profile_id: "opencode/native-files/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    // Each entry is a surface `references/opencode-baseline.json` sources, and
    // the two documented surfaces deliberately left out -- `opencode.jsonc` and
    // `tui.jsonc` -- are in that file's `declined` list with the reason, which
    // a test below also holds.
    //
    // `tui.json` is the TUI half: keybinds, theme, attention and sounds, kept by
    // the product in its own file rather than in `opencode.json`. It was owned
    // by nobody -- not here and not in the consumer's catalog -- so a target
    // configured that way looked empty to `status` and survived `remove`.
    native_namespaces: &[
        "AGENTS.md",
        "opencode.json",
        "tui.json",
        "skills",
        "agents",
        "commands",
        "plugins",
    ],
    // Six names the product reads and this provider does not own -- the first
    // five measured 2026-08-31 by running the 1.18.25 binary against a
    // temporary home, `tui.jsonc` added when the 1.18.33 binary's loader showed
    // both spellings. Declared so `status` can say what it cannot decide.
    shadowing_names: &[
        Shadow {
            name: "opencode.jsonc",
            over: "opencode.json",
            effect: "the product's candidate list joins the two in that order \
                     and keeps the later, so this one wins: with both present \
                     `debug config` returned the JSONC file's value",
        },
        Shadow {
            name: "tui.jsonc",
            over: "tui.json",
            effect: "the TUI loader iterates `fileInDirectory`'s \
                     [tui.json, tui.jsonc] candidates and merges each in turn, \
                     so the JSONC spelling applies last and wins -- measured in \
                     the pinned 1.18.33 binary, where the name is built by a \
                     `${o}.jsonc` template no fixed-string search can see",
        },
        Shadow {
            name: "skill",
            over: "skills",
            effect: "the product globs `{skill,skills}` and one name declared \
                     in both yields one entry; which survives followed the \
                     order the two were created, not the spelling",
        },
        Shadow {
            name: "agent",
            over: "agents",
            effect: "the product globs `{agent,agents}`, read out of the \
                     pinned binary; the collision was measured on skills",
        },
        Shadow {
            name: "command",
            over: "commands",
            effect: "the product globs `{command,commands}`, read out of the \
                     pinned binary; the collision was measured on skills",
        },
        Shadow {
            name: "plugin",
            over: "plugins",
            effect: "the product globs `{plugin,plugins}`, read out of the \
                     pinned binary; the collision was measured on skills",
        },
    ],
    // Owned, and nothing this build can install ever lands here: no
    // component kind routes to them and no setup in this catalogue
    // carries files there. So a posture selecting itself must not empty
    // them -- every posture agrees there is nothing, which makes the
    // emptiness a statement none of them made.
    custody_namespaces: &["tui.json"],
    preservation_surfaces: &[PreservationSurface {
        scope: None,
        roots: &[
            "opencode.jsonc",
            "tui.jsonc",
            "agent",
            "command",
            "skill",
            "plugin",
            "package.json",
            "package-lock.json",
            "bun.lock",
            "node_modules",
        ],
        excluded: &["auth.json", "cache"],
    }],
    // The product's own: credentials and runtime caches. Never read, never
    // written, and never copied into a backup slot.
    never_touch: &["auth.json", "cache"],
    // No near neighbour measured for this product. A marker listed here is a
    // refusal waiting to happen, so nothing is listed without evidence.
    foreign_homes: &[],
    permission_profiles: &["default"],
    // MCP servers are the `mcp` key of `opencode.json`. A key of a file this
    // provider owns is not a surface it can install and restore on its own,
    // so `Mcp` is declared no longer.
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Command,
        ComponentKind::Plugin,
        ComponentKind::Setting,
    ],
    projection_kinds: &[ProjectionKind::NativeFiles, ProjectionKind::Plugin],
    // User-root is `~/.agents`, the one root in this estate
    // that belongs to a convention rather than to a product: a *sibling* of
    // this product's configuration home, not a child, so nothing declared
    // against the target above can reach it. That is what `user_root` is for.
    //
    // The vendor lists *Global agent-compatible: `~/.agents/skills/<name>/SKILL.md`* and the pinned binary carries the literal.
    //
    // **This was a declined row until now, and the reason it carried had
    // stopped being true.** It read *a namespace is removed whole, so a second
    // declaration would make either provider's remove take the other's
    // skills.* Correct when written; false since `written_paths` shipped.
    // Under a scope every verb acts on the files this provider recorded
    // writing -- the removal refuses rather than widening when it cannot read
    // the record, the capture takes ours and not a neighbour's, and a restore
    // leaves a neighbour's file as it was. Five of the seven products read
    // this root, and all five declare it now; the reason was simply not
    // re-read when the thing it described changed.
    scoped_projections: &[
        Scoped {
            target_scope: TargetScope::UserRoot,
            // Distinct from the global identity, because the digest binds a
            // declaration together with the scope it owns.
            profile_id: "opencode/native-files/user-root/1",
            component_kinds: &[ComponentKind::Skill],
            projection_kinds: &[ProjectionKind::NativeFiles],
            // Relative to `~/.agents`, which is the target this scope names -- so a
            // skill is `skills/<name>` rather than `.agents/skills/<name>`. Writing
            // the root into the path would put the skills at
            // `~/.agents/.agents/skills`.
            native_namespaces: &["skills"],
        },
        Scoped {
            target_scope: TargetScope::Project,
            profile_id: "opencode/native-files/project/1",
            component_kinds: &[
                ComponentKind::Instruction,
                ComponentKind::Skill,
                ComponentKind::Agent,
                ComponentKind::Command,
                ComponentKind::Plugin,
                ComponentKind::Setting,
            ],
            projection_kinds: &[ProjectionKind::NativeFiles],
            // Relative to the checkout. The vendor config directory also holds
            // dependencies and runtime state, so never own it as a whole.
            native_namespaces: &[
                "AGENTS.md",
                "opencode.json",
                ".opencode/skills",
                ".opencode/agents",
                ".opencode/commands",
                ".opencode/plugins",
            ],
        },
    ],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    // Generated by `build.rs` from this harness's `setups/` directory, so the
    // binary carries the catalog it is named after instead of hoping to find
    // one on a disk it was never shipped to.
    embedded_setups: include!(concat!(env!("OUT_DIR"), "/embedded_setups.rs")),
    software: Some(software::SOFTWARE),
    instruction_region: Some("AGENTS.md"),
};

fn main() -> ExitCode {
    harness_runtime::run(&OPENCODE, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    /// The directory name this harness's setups live under in the workspace.
    const TOOL: &str = "opencode";
    /// The declaration under test, named once so the shared test below reads
    /// the same in all seven crates.
    const HARNESS: Harness = OPENCODE;

    /// `build.rs` put the whole catalog in, under the paths it will be read by.
    ///
    /// This does **not** test for staleness, and an earlier version of this
    /// comment claimed it did. It cannot: `build.rs` declares
    /// `rerun-if-changed` on the catalog directory, so editing a setup rebuilds
    /// the table before this runs, and the test would be comparing the tree
    /// with itself. Observed — a deliberately edited setup left it green.
    ///
    /// What it does test is the build script, against a walk written
    /// independently of it: every file present, none invented, bytes exact, and
    /// paths relative and slash-separated. That last one is the one that would
    /// really break — `join("/")` is the only reason these keys are usable on
    /// Windows, and a path built with the platform separator would still look
    /// perfectly correct in the generated source.
    /// The bytes this harness ships, pinned so they cannot change unseen.
    ///
    /// A setup's `definition_digest` is what makes two setups the same setup,
    /// and it appears in `list`, in a plan and in provider state -- and until
    /// this, nothing compared it to anything. A stray character in a setup file
    /// changed what the estate installs and every test stayed green.
    ///
    /// One aggregate rather than one per setup, because the claim is about the
    /// catalogue: sorted definition digests, joined by a newline, hashed. A
    /// deliberate change to a setup updates the line in the baseline, which is
    /// the point -- the peer calls this a golden and it earns itself the first
    /// time a row moves without anyone meaning it to.
    ///
    /// **And it is the three-OS check nothing else makes.** The setups are
    /// embedded with `include_bytes!`, so whatever the checkout holds is what
    /// ships; `.gitattributes` pins `eol=lf` to keep a Windows checkout from
    /// rewriting them, and this is the assertion that would notice if it ever
    /// stopped working. The matrix runs it on all three systems, so a digest
    /// that differed by platform could not stay hidden.
    #[test]
    fn the_catalogue_this_harness_ships_is_the_one_the_baseline_records() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let mut digests: Vec<String> = catalog
            .list()
            .unwrap()
            .iter()
            // **Both digests, because one of them holds nothing a person
            // reads.** `definition_digest` is the payload tree; the manifest --
            // `id`, `sources`, `description` -- was covered by no digest in this
            // estate, and those three are what a consumer renders on the surface
            // that precedes an install. A description was rewritten and the
            // whole gate stayed clean, which is how this was found.
            .map(|setup| format!("{}\n{}", setup.definition_digest, setup.manifest_digest))
            .collect();
        digests.sort();
        let joined = digests.join("\n");
        let aggregate = harness_runtime::digest_of_bytes(&joined);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let recorded = baseline["setup_catalogue_digest"].as_str().unwrap_or("");
        assert_eq!(
            aggregate, recorded,
            "the setups this binary ships are not the ones {TOOL}-baseline.json \
             records; if the change was meant, put this digest there"
        );
    }

    #[test]
    fn the_catalog_this_binary_carries_is_the_one_in_the_tree() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // The workspace holds one directory per harness; a rendered public tree
        // ships one harness and holds it flat. Same two candidates `build.rs`
        // chooses between, asked the same way.
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };

        // Only the setup directories, which is what the reader lists and what
        // `build.rs` embeds. A rendered public tree also carries a
        // `setups/README.md` at the catalog root, which belongs to no setup.
        let mut on_disk = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.join("setup.json").is_file())
            .collect();
        while let Some(directory) = stack.pop() {
            for entry in std::fs::read_dir(&directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    on_disk.push(path);
                }
            }
        }

        assert_eq!(
            HARNESS.embedded_setups.len(),
            on_disk.len(),
            "the binary carries {} files and the tree holds {}",
            HARNESS.embedded_setups.len(),
            on_disk.len()
        );

        for (relative, bytes) in HARNESS.embedded_setups {
            assert!(
                !relative.contains('\\') && !relative.starts_with('/'),
                "{relative:?} is not a relative slash path; a key built with the \
                 platform separator reads correctly on Unix and finds nothing on Windows"
            );
            let path = root.join(relative);
            let found = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("{relative} is compiled in but not in the tree: {e}"));
            assert_eq!(
                &found, bytes,
                "{relative} differs between the binary and the tree"
            );
        }
    }

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = OPENCODE.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.harness_id, "opencode");
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in OPENCODE.never_touch {
            assert!(
                !OPENCODE.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }

    /// Everything this harness claims to own, against the vendor page that
    /// decided it.
    ///
    /// What this replaced only checked that the baseline parsed. The block it
    /// reads now is hand-authored beside the rest of the baseline, and this is
    /// what keeps that block from being decoration: a namespace no vendor
    /// document names, or a declared kind no owned surface routes, is red here.
    ///
    /// Both directions, because the defect it was written for ran both ways --
    /// `~/.cursor/rules` was owned and does not exist, `~/.pi/agent/prompts`
    /// exists and was not owned. Conformance caught neither: its
    /// `declared_native_route_is_compilable` case asks for **one** route, not
    /// every one.
    #[test]
    fn every_surface_this_harness_owns_is_one_the_vendor_documents() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let problems = harness_runtime::surfaces::disagreements(&HARNESS, &baseline);
        assert!(
            problems.is_empty(),
            "the declaration and {TOOL}-baseline.json disagree:
  {}",
            problems.join(
                "
  "
            )
        );
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(OPENCODE.control_directory.contains("setup-system"));
        assert!(OPENCODE.state_file.starts_with("NDDEV-"));
        assert!(!OPENCODE.native_namespaces.contains(&OPENCODE.state_file));
    }

    #[test]
    fn every_settings_document_is_owned_under_exactly_one_name() {
        // OpenCode reads either spelling of both of its configuration files.
        // Owning both spellings would let a target hold two documents that
        // disagree, with the product picking one and this provider reporting
        // the other; owning one keeps the answer single.
        //
        // Written as a loop over the pairs rather than as two assertions,
        // because `tui.json` was added a release after `opencode.json` and the
        // second document arriving without the rule following it is exactly how
        // the first one would have been forgotten.
        for (owned, declined) in [
            ("opencode.json", "opencode.jsonc"),
            ("tui.json", "tui.jsonc"),
        ] {
            assert!(
                OPENCODE.native_namespaces.contains(&owned),
                "{owned} is the spelling this provider owns"
            );
            assert!(
                !OPENCODE.native_namespaces.contains(&declined),
                "{declined} is the same document under another name"
            );
        }
    }
    /// A setup that writes a configuration file says where its format came from.
    ///
    /// The release before this one made the *surfaces* sourced: a path this
    /// provider owns cites the page that documents it. This is the same rule
    /// one level down, and it was written because two of the seven failed it.
    ///
    /// opencode's baseline set `"permission": "ask"` where the product
    /// documents an object of tool names, and antigravity's set
    /// `toolPermissions` where the product reads `toolPermission` with four
    /// values, none of them the one written. Both were valid JSON in the right
    /// file at the right path. Both installed, verified and restored cleanly.
    /// Neither changed anything about the product — a target that looks
    /// configured and is not, which is the failure this estate refuses one
    /// level up and had been shipping one level down.
    /// Two files in one setup that a case-insensitive filesystem would merge.
    ///
    /// macOS and Windows fold case, so such a pair is one file there and two on
    /// Linux -- the setup would install different content depending on the
    /// machine, and its catalogue digest would differ per platform. The bundle
    /// reader has refused this for an arriving bundle since 0.0.11; this is the
    /// same rule applied to what this repository authors.
    /// Every component entry point describes itself.
    ///
    /// A `SKILL.md` or an agent whose frontmatter lost its `description` still
    /// installs, verifies and restores cleanly -- and the product names it after
    /// its directory and gives the model nothing to choose on. Documents under
    /// `references/` and files under `commands/` are exempt, because the
    /// products measured do not read frontmatter from either.
    /// Supporting documents are reachable from an entry point.
    ///
    /// A `references/` folder whose skill has no `SKILL.md` is prose nothing
    /// routes to. A generator in this repository produced exactly that, and
    /// every other guard passed it: the files are documents, so `unsourced`
    /// exempts them, and there is no `SKILL.md`, so `undescribed` has nothing
    /// to check.
    /// Nothing shipped sends a reader to a file this setup does not carry.
    ///
    /// A routing table naming `references/surfaces.md` in a setup that ships no
    /// such file sends the reader nowhere -- and the reader is a model, which
    /// will not say so. The generator here did exactly that: it pointed every
    /// harness's agent at that path, and codex ships no skill at all.
    #[test]
    fn nothing_shipped_names_a_document_it_does_not_carry() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::dangling_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn every_reference_folder_has_an_entry_point() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unreachable_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    /// Nothing inside a skill is a file no reader is sent to.
    ///
    /// Two findings in one hour were of exactly this shape and every guard in
    /// this estate was silent on both: an executable validator shipped into
    /// people's homes that nothing named, and eleven authoring pages written
    /// into four harnesses and routed to from none. The estate asked whether a
    /// *named* file exists and never whether an *existing* file is named.
    #[test]
    fn nothing_inside_a_skill_is_stranded() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let found = harness_runtime::catalog::stranded(
            &harness_runtime::Catalog::at(&root).list().unwrap(),
        );
        assert!(found.problems.is_empty(), "{}", found.problems.join("\n  "));
        // opencode carries 11 file(s) inside its skill. Stated so that a layout change emptying the skill fails here rather than passing a guard with nothing left to walk.
        assert_eq!(
            found.entry_points, 11,
            "the stranded-file guard walked {} files inside skills, not 11",
            found.entry_points
        );
    }

    #[test]
    fn every_component_entry_point_describes_itself() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let examined = harness_runtime::catalog::undescribed(&catalog.list().unwrap());
        assert!(
            examined.problems.is_empty(),
            "{}",
            examined.problems.join("\n  ")
        );
        // opencode ships 2 entry point(s) across its four postures. Stated so that a layout change removing them fails here rather than passing a guard with nothing left to check.
        assert_eq!(
            examined.entry_points, 2,
            "the description guard examined {} entry points, not 2",
            examined.entry_points
        );
    }

    #[test]
    fn no_two_files_in_a_setup_differ_only_in_case() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::colliding(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn a_setup_that_writes_configuration_says_where_its_format_came_from() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unsourced(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Three postures, on every one of the seven.
    ///
    /// `baseline` is a working floor, `minimal` is the shared autonomous posture and nothing else,
    /// and `full-auto` asks nothing and sandboxes nothing. A caller who learns
    /// them on one product knows them on all seven, which is the whole reason
    /// the names are the estate's rather than each harness's.
    ///
    /// The second half of the check is the one worth having: two setups with
    /// the same bytes mean one of them is a posture in name only, and it would
    /// still read as offered in `list`.
    #[test]
    fn the_three_postures_are_offered_and_are_actually_different() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::asymmetric(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Nothing this setup ships tells a reader to run something that is not here.
    ///
    /// A setup carries documents an agent reads and acts on -- a skill, a rule,
    /// a command file -- and nothing was checking them. One shipped
    /// `software-status --target <dir> --json` and `list --json` for six
    /// releases; the binary refuses both, and says so in those words.
    ///
    /// Two refusals: a name belonging to the frozen estate, and any line naming
    /// this provider followed by a verb `into_command` does not accept. English
    /// is not judged -- `install` in a sentence is a word, and only
    /// `<provider> install` is an instruction.
    #[test]
    fn nothing_this_harness_ships_names_a_command_it_refuses() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems =
            harness_runtime::catalog::misdirecting(HARNESS.provider_id, &catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
}
