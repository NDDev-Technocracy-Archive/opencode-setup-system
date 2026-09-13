//! Opencode's own program, as measured rather than as described.
//!
//! Generated from the `software_artifacts` block of
//! `references/opencode-baseline.json`. Every member path below was read out
//! of the archive it names, not assumed: codex's carries the target triple and
//! so genuinely differs per platform.
//!
//! Where a `previous_software_artifacts` block is present, it is transcribed
//! too. It is not a second choice: the outgoing current pin is stored there on
//! a bump, so the pair is always two consecutive real releases and there is
//! still exactly one value to keep fresh.
//!
//! Do not edit. The test at the bottom re-reads that baseline and compares it
//! field by field, so an edit here fails rather than silently installing bytes
//! nobody measured.

use harness_runtime::{Artifact, Delivery, Previous, Shape, Software};

/// The artifacts opencode is published as.
pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/opencode-linux-arm64/-/opencode-linux-arm64-1.18.30.tgz",
        bytes: 59_985_069,
        sha256: "sha256:1e20b66e76afbb7c1e5cd9be7515d2744cb4ea3476ff57add1f7e9ad6fb3a32d",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/opencode-linux-x64/-/opencode-linux-x64-1.18.30.tgz",
        bytes: 60_202_013,
        sha256: "sha256:aa31a7e68ce5c73cba302c6a4f31c2e308398db125e49d3dbea32f61862fe6de",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/opencode-darwin-arm64/-/opencode-darwin-arm64-1.18.30.tgz",
        bytes: 45_975_576,
        sha256: "sha256:25b722fcdc8c46aebcb6501e1156dc1dc5a392492cc278c65b7775311674e7f8",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/opencode-darwin-x64/-/opencode-darwin-x64-1.18.30.tgz",
        bytes: 48_153_052,
        sha256: "sha256:d7119bd0adbacf86c1176f07b35d429909d1d702ca0c90dcd31415e66220f590",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/opencode-windows-arm64/-/opencode-windows-arm64-1.18.30.tgz",
        bytes: 58_430_892,
        sha256: "sha256:d26f3759e7c17754e1d6c72020853ffb153c7bb0f6b7d75efdc48466f482c4d4",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/opencode-windows-x64/-/opencode-windows-x64-1.18.30.tgz",
        bytes: 60_117_264,
        sha256: "sha256:1a5cc50f529cf359fbb6d1a1750c6ae521799f463d00a0215e6eff579a08b036",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
];

/// The artifacts 1.18.29 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/opencode-linux-arm64/-/opencode-linux-arm64-1.18.29.tgz",
        bytes: 59_952_206,
        sha256: "sha256:bbc655accf8263d44e5c668ef865520239e58bef515057bf8ecc27d006a6f28a",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/opencode-linux-x64/-/opencode-linux-x64-1.18.29.tgz",
        bytes: 60_170_392,
        sha256: "sha256:a2df564ba242759e3ee27cbd356da786243a3d3fd911cf5aa2c417521aef3042",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/opencode-darwin-arm64/-/opencode-darwin-arm64-1.18.29.tgz",
        bytes: 45_944_097,
        sha256: "sha256:1fc08fee8b4984c1306c826b8c0e2c367fd9eeb4d4c3707469194e3fea047e72",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/opencode-darwin-x64/-/opencode-darwin-x64-1.18.29.tgz",
        bytes: 48_120_807,
        sha256: "sha256:d9e2270b9040d7ce140df629773c68f15222c7a6c882f16921d36aa4c9200ae2",
        shape: Shape::GzipTar,
        member: "package/bin/opencode",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/opencode-windows-arm64/-/opencode-windows-arm64-1.18.29.tgz",
        bytes: 58_399_395,
        sha256: "sha256:ad2f4a063a6d82578edc0b37078eff76fcb2a58f6439a33c34a50e1c795512b8",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/opencode-windows-x64/-/opencode-windows-x64-1.18.29.tgz",
        bytes: 60_086_309,
        sha256: "sha256:19eca6cdead9c67cce26fdc2db165980318edb7318c8c964dfc2ebffe03bb472",
        shape: Shape::GzipTar,
        member: "package/bin/opencode.exe",
    },
];

/// Opencode's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.18.30",
    command: "opencode",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.18.29",
        artifacts: PREVIOUS_ARTIFACTS,
    }),
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    // Named rather than glob-imported: a product delivered by a package manager
    // has no `Artifact` in scope, and the test is the same text for all seven.
    use harness_runtime::{Delivery, Shape};

    use super::SOFTWARE;

    fn measured() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/opencode-baseline.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn every_artifact_compiled_in_is_the_one_the_baseline_measured() {
        let block = &measured()["software_artifacts"];
        assert_eq!(block["version"], SOFTWARE.version);
        assert_eq!(block["command"], SOFTWARE.command);

        let Delivery::Artifacts(compiled) = SOFTWARE.delivery else {
            // A product delivered by a package manager has no artifacts, and
            // the baseline must agree that it has none.
            assert_eq!(block["shape"], "manager");
            assert!(block["platforms"].as_object().unwrap().is_empty());
            return;
        };
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            compiled.len(),
            published.len(),
            "the table and the baseline disagree on how many platforms exist"
        );
        for artifact in compiled {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
            let member = entry.get("member").and_then(serde_json::Value::as_str);
            assert_eq!(
                member.unwrap_or(""),
                artifact.member,
                "{} names a different member",
                artifact.platform
            );
            assert_eq!(
                artifact.shape == Shape::Raw,
                member.is_none(),
                "{} disagrees about whether the bytes are the program",
                artifact.platform
            );
        }
    }

    /// The second pin is the baseline's, or it is absent in both places.
    ///
    /// Asserted from either side rather than only where it exists: a harness
    /// that has never been bumped must compile in `None`, and a build that
    /// dropped the block while the baseline still carried it would otherwise
    /// pass by having nothing to compare.
    #[test]
    fn the_version_this_build_can_move_between_is_the_one_measured_before_it() {
        let baseline = measured();
        let recorded = baseline.get("previous_software_artifacts");
        let Some(earlier) = SOFTWARE.previous else {
            assert!(
                recorded.is_none(),
                "the baseline records a previous release and this build names none"
            );
            return;
        };
        let block = recorded.unwrap_or_else(|| {
            panic!("this build names a previous release the baseline does not record")
        });
        assert_eq!(block["version"], earlier.version);
        assert_ne!(
            earlier.version, SOFTWARE.version,
            "a second pin equal to the first is one version wearing two names"
        );
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            earlier.artifacts.len(),
            published.len(),
            "the previous table and the baseline disagree on how many platforms exist"
        );
        for artifact in earlier.artifacts {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
        }
    }

    #[test]
    fn a_platform_the_vendor_does_not_publish_is_listed_rather_than_missing() {
        let block = &measured()["software_artifacts"];
        let unpublished: Vec<&str> = block
            .get("unpublished")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(unpublished, SOFTWARE.unsupported);
    }

    #[test]
    fn no_release_calls_a_platform_both_published_and_unpublished() {
        let baseline = measured();
        for name in ["software_artifacts", "previous_software_artifacts"] {
            let Some(block) = baseline.get(name) else {
                continue;
            };
            let published = block["platforms"].as_object().unwrap();
            let unpublished = block
                .get("unpublished")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str);
            for platform in unpublished {
                assert!(
                    !published.contains_key(platform),
                    "{name}: {platform} is both published and unpublished"
                );
            }
        }
    }
}
