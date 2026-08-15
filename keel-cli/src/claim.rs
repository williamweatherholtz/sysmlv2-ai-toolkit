//! `keel claim` — one contributor's intent to work one item (D0147/D0129 srDcWorkClaim).
//!
//! # The frontier is one global list, and that is the problem
//!
//! `ready` has no owner and no claim, so several contributors — nearly all AI, on separate machines,
//! working asynchronously — rationally select the same top-ranked item, and the duplication surfaces
//! only at integration. A claim makes the intent visible BEFORE the work starts.
//!
//! # Liveness is computed, never stored
//!
//! A `Claim` carries who, what, when and against-which-commit, and nothing else. It is LIVE if it is
//! the earliest un-expired claim on its item; STALE once past the expiry window. Storing a
//! status would be a verdict that disagrees with its own facts the moment the window passes (§1.6),
//! and releasing a claim would then be a write that could be forgotten. Expiry that happens by
//! itself cannot be forgotten.
//!
//! # Exclusion is COMPUTED, because the push does not provide it
//!
//! The obvious design says two contributors claiming one item both push, the remote accepts exactly
//! one because a ref update is a compare-and-swap, and the loser re-syncs and picks again. A
//! two-clone test disproved that here. Claims are written to PER-ACTOR files
//! (srDcPerActorWriteTargets), which deliberately removes the write contention, so both claims merge
//! cleanly and BOTH land. The push rejection is real, but the loser resolves it by merging and
//! retrying, and then holds a claim just as valid as the winner's.
//!
//! So the holder is computed, and it cannot be "whoever landed first" — that is not recoverable from
//! merged history. See [`claims`] for why, and for the rule that replaced it.

use std::path::Path;

/// How long a claim stays live without progress.
///
/// Deliberately generous: the cost of a stale claim is a brief duplication, while the cost of
/// expiring a live one is two contributors on one item each believing they hold it. Wrong in the
/// safe direction.
pub const CLAIM_EXPIRY_DAYS: i64 = 2;

/// A claim as computed, with the liveness the model does not store.
pub struct ClaimView {
    pub name: String,
    pub item: String,
    pub by: String,
    pub at: String,
    pub against: String,
    pub age_days: i64,
    pub live: bool,
    /// Outranked by another LIVE claim on the same item — distinct from stale, and worth separating:
    /// superseded means someone else holds it, stale means nobody does and it is fair to take.
    pub superseded: bool,
}

/// Every claim in the model, holder first per item, with liveness computed against git-derived time.
///
/// # Errors
/// Returns [`crate::view::ViewError`] if a tracking file fails to parse.
pub fn claims(root: &Path) -> Result<Vec<ClaimView>, crate::view::ViewError> {
    let today = crate::view::repo_today_pub(root);
    let mut rows = crate::view::claim_rows(root)?;
    // WHO HOLDS THE ITEM, and why it cannot be "whoever landed first".
    //
    // The process says exclusion comes from the remote accepting exactly one of two concurrent
    // claims, because a ref update is a compare-and-swap. THAT IS NOT TRUE HERE, and the two-clone
    // test proved it: per-actor claim files (srDcPerActorWriteTargets) deliberately remove the write
    // contention, so both claims merge cleanly and BOTH land. Exclusion has to be computed.
    //
    // "Whoever landed first" is then not recoverable. Two claims committed in parallel are SIBLINGS:
    // measured against the merged history they have the same ancestry depth, and their commit
    // timestamps tie whenever the work happens in the same second — which it did. Any rule reading
    // git order either disagrees between clones or falls through to an arbitrary tie-break anyway.
    //
    // So the rule is EARLIEST `claimedAt`, then lowest claim id. The id is a UUID: unbiased, total,
    // and identical in every clone, which is the only property that actually matters — every
    // contributor must compute the same holder without coordinating. Sorting by NAME instead would
    // have handed the item to whoever sorts alphabetically first, which is deterministic and unfair,
    // and the test caught exactly that (alpha held it over beta whichever one landed first).
    let ids = crate::view::claim_ids(root)?;
    rows.sort_by(|a, b| {
        a.3.cmp(&b.3).then_with(|| ids.get(&a.0).cmp(&ids.get(&b.0))).then_with(|| a.0.cmp(&b.0))
    });
    // EXPIRY REMOVES A CLAIM FROM CONTENTION, not merely from the top of the ranking. Ordering first
    // and expiring second looked equivalent and was not: a two-clone test aged the earliest claim past
    // the window and the item became held by NOBODY, because the stale claim still occupied the holder
    // slot and marked the fresh claim `superseded`. An expired claim must not be able to supersede a
    // live one — so eligibility is decided BEFORE the holder is picked, and the holder is the earliest
    // claim among those still inside the window.
    let mut seen_item: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for (name, item, by, at, against) in rows {
        let age = if today.is_empty() || at.is_empty() { -1 } else { crate::view::days_between_pub(&at, &today) };
        let eligible = (0..=CLAIM_EXPIRY_DAYS).contains(&age);
        let superseded = eligible && !seen_item.insert(item.clone());
        out.push(ClaimView {
            live: eligible && !superseded,
            name,
            item,
            by,
            at,
            against,
            age_days: age,
            superseded,
        });
    }
    Ok(out)
}

/// Item names held LIVE by someone other than `actor`.
///
/// # Errors
/// Returns [`crate::view::ViewError`] if a tracking file fails to parse.
pub fn held_by_others(root: &Path, actor: &str) -> Result<Vec<(String, String)>, crate::view::ViewError> {
    Ok(claims(root)?
        .into_iter()
        .filter(|c| c.live && c.by != actor)
        .map(|c| (c.item, c.by))
        .collect())
}

/// `keel claim <item>` / `keel claim --list` / `keel claim --mine`.
#[must_use]
pub fn cmd(args: &[String], root: &Path) -> i32 {
    let actor = crate::actor::resolve(root, None);
    if args.iter().any(|a| a == "--list") || args.is_empty() {
        let Ok(all) = claims(root) else {
            eprintln!("error: cannot read claims");
            return 1;
        };
        if all.is_empty() {
            println!("no claims recorded. `keel claim <item>` to take one.");
            return 0;
        }
        println!("claims ({} recorded, expiry {CLAIM_EXPIRY_DAYS}d, liveness COMPUTED not stored):", all.len());
        for c in &all {
            let state = if c.live {
                "LIVE     "
            } else if c.superseded {
                "superseded"
            } else {
                "stale    "
            };
            println!("  [{state}] {} by {} ({}d old, against {})", c.item, c.by, c.age_days, c.against);
        }
        return 0;
    }
    let Ok(actor) = actor else {
        eprintln!("{}", crate::actor::unresolved_message());
        return 2;
    };
    if args.iter().any(|a| a == "--mine") {
        let Ok(all) = claims(root) else { return 1 };
        let mine: Vec<&ClaimView> = all.iter().filter(|c| c.by == actor && c.live).collect();
        println!("{} live claim(s) held by {actor}:", mine.len());
        for c in mine {
            println!("  {} ({}d old)", c.item, c.age_days);
        }
        return 0;
    }
    let Some(item) = args.first().filter(|a| !a.starts_with("--")) else {
        eprintln!("usage: keel claim <item> | --list | --mine");
        return 2;
    };
    // Refuse to take an item someone else holds LIVE. A stale one is fair to take — that is what
    // expiry is for — and the message says which case this is.
    match held_by_others(root, &actor) {
        Ok(held) => {
            if let Some((_, holder)) = held.iter().find(|(i, _)| i == item) {
                eprintln!("error: '{item}' is held LIVE by {holder}.");
                eprintln!("  Choose different work — `keel whats-next` excludes what others hold. A claim held past");
                eprintln!("  {CLAIM_EXPIRY_DAYS} days without progress computes as STALE and is fair to take (D0129).");
                return 1;
            }
        }
        Err(e) => {
            eprintln!("error reading existing claims: {e}");
            return 1;
        }
    }
    match crate::write::record_claim(root, item, &actor) {
        Ok((name, path)) => {
            println!("claimed '{item}' as {name} -> {path}");
            println!("  LAND IT NOW: a claim is only exclusion once it is visible remotely. `keel land` pushes it,");
            println!("  and if the push is rejected another contributor claimed first — re-sync and pick again.");
            println!("  That rejection is the mechanism working, not an error.");
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}
