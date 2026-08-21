//! `keel new sprint` — the engine scaffolds the ceremony record, so identity and structure are never
//! hand-authored (us019/st013: "we should be programatically creating UUIDs, not via AI diligence").
//!
//! WHAT IT REFUSES TO GUESS. Every id is minted (`write::gen_uuid`); dates come from the real clock;
//! the author comes from the bound actor and is REFUSED when absent (the provenance rule, never
//! defaulted); the charter must name an EXISTING Decision, because an edge to nothing is exactly what
//! the edge-endpoints guard exists to catch. Every judgment-bearing text is set to [`PLACEHOLDER`],
//! which guard 40 (`scaffold-placeholder`) and the fast gate REJECT — an unfilled skeleton cannot
//! pass a gate or be committed, by construction rather than by diligence.
//!
//! No `TestResult`s are generated: results are appended when a gate is actually judged
//! (`append-gate-result`), never pre-created.

use crate::write::gen_uuid;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The token an unfilled scaffold carries. Guard 40 fails any committed `.sysml` containing it, and
/// `keel gate --fast` rejects it per-edit — the token IS the incompleteness marker.
pub const PLACEHOLDER: &str = "KEEL-SCAFFOLD-FILL-ME";

/// The six ceremony gates, in `GATE_ORDER`, with each one's verification method.
const GATES: [(&str, &str, &str); 6] = [
    ("Refine", "inspect", "refine gate (DoR)"),
    ("Standup", "inspect", "standup gate"),
    ("Implement", "test", "implement gate"),
    ("Review", "inspect", "review gate — three passes per D0172"),
    ("CloseOut", "inspect", "closeOut gate (autonomous, D0049)"),
    ("Retro", "analyze", "retro gate (autonomous, D0049)"),
];

/// Scaffold `.tracking/delivery/sprint<number>_<slug>.sysml`. Returns the path written.
///
/// # Errors
/// A used sprint number or existing file, a malformed slug, an unknown charter Decision, or an io
/// failure — each as a message naming the refusal, because every one is a caller mistake this
/// command exists to catch before it becomes a recorded fact.
pub fn sprint(
    root: &Path,
    number: u32,
    slug: &str,
    charter: &str,
    points: u32,
    actor: &str,
) -> Result<PathBuf, String> {
    if !slug.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        || !slug.chars().all(|c| c.is_ascii_alphanumeric())
    {
        return Err(format!(
            "slug `{slug}` must be lowerCamelCase alphanumeric — it becomes item names and the filename"
        ));
    }
    if !decision_exists(root, charter) {
        return Err(format!(
            "charter `{charter}` is not a declared Decision under .engine/decisions — an edge to nothing is a lie the edge-endpoints guard would catch anyway"
        ));
    }
    let dir = root.join(".tracking").join("delivery");
    if let Ok(rd) = std::fs::read_dir(&dir) {
        let prefix = format!("sprint{number}_");
        for e in rd.flatten() {
            if e.file_name().to_string_lossy().starts_with(&prefix) {
                return Err(format!(
                    "sprint {number} already exists: {} — a sprint number is never reused",
                    e.path().display()
                ));
            }
        }
    }
    let path = dir.join(format!("sprint{number}_{slug}.sysml"));
    if path.exists() {
        return Err(format!("refusing to overwrite {}", path.display()));
    }

    let today = chrono_date();
    let mut cap = slug.to_owned();
    if let Some(first) = cap.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    let mut t = String::new();
    let _ = writeln!(
        t,
        "// ProjectDeliveryS{number} — Sprint {number}: {PLACEHOLDER} (one-line purpose). EXECUTE. {points} pts."
    );
    let _ = writeln!(t, "package ProjectDeliveryS{number} {{");
    for imp in ["EngineElement", "EngineWork", "EngineVerification", "EngineRelationships"] {
        let _ = writeln!(t, "    private import {imp}::*;");
    }
    let _ = writeln!(t, "\n    #CharteredBy dependency from {slug}Story to {charter};\n");
    let _ = writeln!(t, "    part {slug}Story : Story {{");
    let _ = writeln!(t, "        :>> id = \"{}\";", gen_uuid());
    let _ = writeln!(t, "        :>> title = \"Sprint {number}: {PLACEHOLDER} ({points} pts)\";");
    let _ = writeln!(
        t,
        "        :>> createdAt = \"{today}\"; :>> createdBy = \"{actor}\"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = \"{actor}\"; :>> estimatedPoints = {points};"
    );
    let _ = writeln!(t, "    }}\n");
    let _ = writeln!(t, "    action def DeliveryRunS{number} {{");
    let _ = writeln!(t, "        action story{cap};");
    let _ = writeln!(
        t,
        "        verification story{cap}DoD : Test {{ :>> id = \"{}\"; :>> method = VerificationMethod::test; :>> procedureText = \"{PLACEHOLDER}: DELIVERED BACKLOG ITEMS: <items>. <what done means, verified how>.\"; }}",
        gen_uuid()
    );
    let _ = writeln!(t, "    }}\n");
    for (g, method, title) in GATES {
        let _ = writeln!(
            t,
            "    verification {slug}{g}Gate : Test {{ :>> id = \"{}\"; :>> title = \"Sprint {number} {title}\"; :>> createdAt = \"{today}\"; :>> createdBy = \"{actor}\"; :>> method = VerificationMethod::{method}; :>> procedureText = \"{PLACEHOLDER}\"; }}",
            gen_uuid()
        );
    }
    let _ = writeln!(t, "}}");

    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    crate::write::write_atomic(&path, &t).map_err(|e| e.to_string())?;
    Ok(path)
}

/// True when `name` is declared `part <name> : Decision` under `.engine/decisions`.
fn decision_exists(root: &Path, name: &str) -> bool {
    let needle = format!("part {name} : Decision");
    let Ok(rd) = std::fs::read_dir(root.join(".engine").join("decisions")) else { return false };
    rd.flatten().any(|e| {
        std::fs::read_to_string(e.path()).is_ok_and(|text| text.contains(&needle))
    })
}

/// Today as `YYYY-MM-DD` from the system clock — the one authoring field where the real clock IS the
/// truth (a scaffold is created now; only JUDGMENT dates are refused-not-defaulted).
fn chrono_date() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let days = secs / 86_400;
    // civil-from-days (Howard Hinnant's algorithm), UTC
    let z = i64::try_from(days).unwrap_or(0) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::{sprint, PLACEHOLDER};

    fn temp_root(tag: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("keel-scaffold-{tag}"));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".tracking").join("delivery")).expect("mkdir");
        std::fs::create_dir_all(root.join(".engine").join("decisions")).expect("mkdir");
        std::fs::write(
            root.join(".engine").join("decisions").join("9997-t.sysml"),
            "package D9997 { part d9997 : Decision { :>> id = \"e2e00000-0000-4000-8000-000000009997\"; } }\n",
        )
        .expect("write charter");
        root
    }

    /// The `DoD` checks themselves: every minted id is guard-38-shaped and unique; all six ceremony gates
    /// and the Story are present; the placeholder is present; overwrite and bad inputs refuse.
    #[test]
    fn scaffold_mints_wellformed_unique_ids_and_refuses_misuse() {
        let root = temp_root("main");
        let path = sprint(&root, 999, "testRun", "d9997", 3, "claudeOpus5").expect("scaffold");
        let text = std::fs::read_to_string(&path).expect("read back");
        let ids: Vec<&str> = text
            .split(":>> id = \"")
            .skip(1)
            .map(|s| s.split('"').next().unwrap_or(""))
            .collect();
        assert_eq!(ids.len(), 8, "story + DoD + six gates");
        let mut seen = std::collections::HashSet::new();
        for id in &ids {
            assert!(crate::guards::uuid_shaped(id), "guard 38 rejects a scaffolded id: {id}");
            assert!(seen.insert(*id), "duplicate id in one scaffold");
        }
        for g in ["RefineGate", "StandupGate", "ImplementGate", "ReviewGate", "CloseOutGate", "RetroGate"] {
            assert!(text.contains(&format!("testRun{g}")), "missing {g}");
        }
        assert!(text.contains("part testRunStory : Story"));
        assert!(text.contains(PLACEHOLDER), "the incompleteness marker must be present");
        assert!(!text.contains(": TestResult"), "results are appended when judged, never pre-created");

        assert!(sprint(&root, 999, "again", "d9997", 3, "x").is_err(), "sprint number reuse must refuse");
        assert!(sprint(&root, 998, "Bad-Slug", "d9997", 3, "x").is_err(), "non-camel slug must refuse");
        assert!(sprint(&root, 998, "ok", "dNothing", 3, "x").is_err(), "unknown charter must refuse");
    }

    /// Guard 40 rejects the scaffold until it is filled — the whole point of the marker.
    #[test]
    fn the_placeholder_guard_rejects_an_unfilled_scaffold_and_passes_a_filled_one() {
        let root = temp_root("guard");
        let path = sprint(&root, 999, "guardRun", "d9997", 2, "claudeOpus5").expect("scaffold");
        let report = crate::guards::scaffold_placeholder(&root);
        assert!(!report.violations.is_empty(), "an unfilled scaffold must be rejected");
        let filled = std::fs::read_to_string(&path).expect("read").replace(PLACEHOLDER, "filled in");
        std::fs::write(&path, filled).expect("fill");
        assert!(crate::guards::scaffold_placeholder(&root).violations.is_empty(), "a filled scaffold passes");
    }
}
