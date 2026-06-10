#!/bin/sh
# Project bootstrap — run ONCE per clone:  sh ./bootstrap.sh
#
# Captures the per-clone setup that git cannot commit for us (hooks are opt-in
# for security, so a committed hook isn't active until core.hooksPath points at
# it). Idempotent — safe to re-run. This is the home for future per-clone setup
# (e.g. creating .tracking/, checking the sysml conda env). Eventually the
# engine's onboarding workflow should run this and track it as a work-item.
set -e

git config core.hooksPath .githooks
echo "bootstrap: core.hooksPath -> .githooks (commit -> auto-push enabled)"

echo "bootstrap: done."
