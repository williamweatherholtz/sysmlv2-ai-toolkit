"""Measurement-only: time the Model clone a cache hit pays. Anchor must be unique or FAIL."""
import io
import sys

p = "keel-cli/src/view/mod.rs"
s = io.open(p, encoding="utf-8").read()
old = "        MODEL_CACHE.lock().ok().and_then(|g| g.as_ref().filter(|(c, _)| *c == fp).map(|(_, m)| m.clone()))\n"
new = "        crate::perf::phase(\"model:cache_clone\", || MODEL_CACHE.lock().ok().and_then(|g| g.as_ref().filter(|(c, _)| *c == fp).map(|(_, m)| m.clone())))\n"
if s.count(old) != 1:
    sys.exit("anchor not found exactly once - refusing to patch")
io.open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new, 1))
print("patched")
