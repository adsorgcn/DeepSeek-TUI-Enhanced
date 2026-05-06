//! System-skill installer: bundles all Enhanced skills and auto-installs on first launch.

use std::fs;
use std::path::Path;

/// Bump this when the bundled skill set changes (new skills added,
/// existing skills updated). The installer compares this against the
/// on-disk marker to decide whether a re-install pass is needed.
const BUNDLED_SKILL_VERSION: &str = "2";

/// All system skills bundled with DeepSeek-TUI Enhanced.
const SYSTEM_SKILLS: &[(&str, &str)] = &[
    (
        "skill-creator",
        include_str!("../../assets/skills/skill-creator/SKILL.md"),
    ),
    (
        "session-guardian",
        include_str!("../../assets/skills/session-guardian/SKILL.md"),
    ),
    (
        "coordinator",
        include_str!("../../assets/skills/coordinator/SKILL.md"),
    ),
    (
        "imprint",
        include_str!("../../assets/skills/imprint/SKILL.md"),
    ),
    (
        "code-review",
        include_str!("../../assets/skills/code-review/SKILL.md"),
    ),
    (
        "project-init",
        include_str!("../../assets/skills/project-init/SKILL.md"),
    ),
];

/// Install bundled system skills into `skills_dir`.
///
/// Behaviour:
/// - Fresh install (no marker): installs all system skills and writes
///   the version marker.
/// - Version bump (marker present with older version): re-installs
///   skills whose directories still exist (user hasn't deleted them),
///   and installs any newly-added skills.
/// - User deleted a skill dir while marker is at current version:
///   leaves it gone (respects user choice).
/// - Idempotent: calling twice with no changes is a no-op.
///
/// Each skill is tracked independently: deleting `session-guardian/`
/// does not prevent `coordinator/` from being installed or updated.
///
/// Errors are I/O errors from the filesystem; the caller should log
/// them but not abort startup.
pub fn install_system_skills(skills_dir: &Path) -> std::io::Result<()> {
    let marker = skills_dir.join(".system-installed-version");

    let installed_version = fs::read_to_string(&marker)
        .ok()
        .map(|s| s.trim().to_string());

    let is_fresh = installed_version.is_none();
    let is_upgrade = installed_version
        .as_deref()
        .is_some_and(|v| v != BUNDLED_SKILL_VERSION);

    if !is_fresh && !is_upgrade {
        // Already at current version. Nothing to do.
        return Ok(());
    }

    fs::create_dir_all(skills_dir)?;

    for &(name, body) in SYSTEM_SKILLS {
        let target_dir = skills_dir.join(name);
        let target_file = target_dir.join("SKILL.md");
        let dir_exists = target_dir.exists();

        let should_install = match (is_fresh, dir_exists) {
            // Fresh install: install everything.
            (true, _) => true,
            // Upgrade, dir still exists: re-install (content may have changed).
            (false, true) => true,
            // Upgrade, dir gone: user deleted it. Respect that choice,
            // UNLESS this is a newly-added skill that didn't exist in
            // the previous version. We install new skills even on upgrade.
            (false, false) => is_new_skill_since(&installed_version, name),
        };

        if should_install {
            fs::create_dir_all(&target_dir)?;
            fs::write(&target_file, body)?;
        }
    }

    fs::write(&marker, BUNDLED_SKILL_VERSION)?;
    Ok(())
}

/// Returns true if `name` was not part of the skill set at `version`.
/// This lets the installer add new skills on upgrade even when the
/// user has previously deleted other skills.
fn is_new_skill_since(installed_version: &Option<String>, name: &str) -> bool {
    match installed_version.as_deref() {
        // Version "1" only had skill-creator. Everything else is new.
        Some("1") => name != "skill-creator",
        // Unknown old version: treat all as potentially new.
        Some(_) => true,
        // No version (fresh): handled by caller, but default to true.
        None => true,
    }
}

/// Remove all system skills and the version marker.
///
/// Intended for tests and `deepseek setup --clean`. Ignores missing files.
#[allow(dead_code)]
pub fn uninstall_system_skills(skills_dir: &Path) -> std::io::Result<()> {
    let marker = skills_dir.join(".system-installed-version");

    for &(name, _) in SYSTEM_SKILLS {
        let target_dir = skills_dir.join(name);
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir)?;
        }
    }
    if marker.exists() {
        fs::remove_file(&marker)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // -- helpers --

    fn skill_file(tmp: &TempDir, name: &str) -> std::path::PathBuf {
        tmp.path().join(name).join("SKILL.md")
    }

    fn marker_file(tmp: &TempDir) -> std::path::PathBuf {
        tmp.path().join(".system-installed-version")
    }

    // -- fresh install --

    #[test]
    fn fresh_install_creates_all_enhanced_skills() {
        let tmp = TempDir::new().unwrap();
        install_system_skills(tmp.path()).unwrap();

        for &(name, _) in SYSTEM_SKILLS {
            assert!(
                skill_file(&tmp, name).exists(),
                "{name}/SKILL.md should be created"
            );
        }
        assert!(marker_file(&tmp).exists(), "marker should be created");

        let ver = fs::read_to_string(marker_file(&tmp)).unwrap();
        assert_eq!(ver.trim(), BUNDLED_SKILL_VERSION);
    }

    // -- idempotence --

    #[test]
    fn calling_twice_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        install_system_skills(tmp.path()).unwrap();

        // Overwrite one SKILL.md with sentinel.
        fs::write(skill_file(&tmp, "skill-creator"), "sentinel").unwrap();

        install_system_skills(tmp.path()).unwrap();

        let contents = fs::read_to_string(skill_file(&tmp, "skill-creator")).unwrap();
        assert_eq!(
            contents, "sentinel",
            "second install should not overwrite when version is current"
        );
    }

    // -- user deleted one skill --

    #[test]
    fn user_deleted_one_skill_is_not_recreated() {
        let tmp = TempDir::new().unwrap();
        install_system_skills(tmp.path()).unwrap();

        // User deletes session-guardian but keeps others.
        fs::remove_dir_all(tmp.path().join("session-guardian")).unwrap();

        install_system_skills(tmp.path()).unwrap();

        assert!(
            !skill_file(&tmp, "session-guardian").exists(),
            "deleted skill must not be recreated at same version"
        );
        // Other skills still present.
        assert!(skill_file(&tmp, "coordinator").exists());
    }

    // -- version bump re-installs existing, adds new --

    #[test]
    fn version_bump_reinstalls_all_existing_system_skills() {
        let tmp = TempDir::new().unwrap();

        // Simulate v1: only skill-creator installed.
        let skill_dir = tmp.path().join("skill-creator");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), "old content").unwrap();
        fs::write(marker_file(&tmp), "1").unwrap();

        install_system_skills(tmp.path()).unwrap();

        // skill-creator should be updated.
        let contents = fs::read_to_string(skill_file(&tmp, "skill-creator")).unwrap();
        assert_ne!(contents, "old content", "existing skill should be updated");

        // New skills should be installed even though they didn't exist before.
        for &(name, _) in SYSTEM_SKILLS {
            assert!(
                skill_file(&tmp, name).exists(),
                "{name} should be installed on version bump"
            );
        }

        let ver = fs::read_to_string(marker_file(&tmp)).unwrap();
        assert_eq!(ver.trim(), BUNDLED_SKILL_VERSION);
    }

    // -- uninstall --

    #[test]
    fn uninstall_removes_all_skills_and_marker() {
        let tmp = TempDir::new().unwrap();
        install_system_skills(tmp.path()).unwrap();
        uninstall_system_skills(tmp.path()).unwrap();

        for &(name, _) in SYSTEM_SKILLS {
            assert!(
                !skill_file(&tmp, name).exists(),
                "{name} should be removed"
            );
        }
        assert!(!marker_file(&tmp).exists(), "marker should be removed");
    }

    #[test]
    fn uninstall_on_clean_dir_is_a_noop() {
        let tmp = TempDir::new().unwrap();
        uninstall_system_skills(tmp.path()).unwrap();
    }
}
