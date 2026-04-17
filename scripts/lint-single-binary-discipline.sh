#!/usr/bin/env bash
# lint-single-binary-discipline.sh
#
# Fails if any tier-conditional cfg attributes are found in src/.
# This enforces the single-binary discipline from CONVENTIONS §2.2:
# no #[cfg(feature = "smart")], #[cfg(feature = "dumb")], or similar
# tier-differentiating cfg gates.
#
# Only matches actual code lines, not comments or doc-comments.
#
# Part of CONVENTIONS §4 CI pipeline.

set -euo pipefail

VIOLATIONS=0

# Search for tier-conditional cfg attributes in non-comment lines.
# We exclude lines that start with // or //! (doc comments and regular comments).
while IFS= read -r line; do
    echo "VIOLATION: $line"
    VIOLATIONS=$((VIOLATIONS + 1))
done < <(grep -rn '#\[cfg(feature' src/ 2>/dev/null | grep -v '^\s*//' | grep -v '//.*#\[cfg(feature' | grep -E '"smart"|"dumb"|"tier"' 2>/dev/null || true)

if [ "$VIOLATIONS" -gt 0 ]; then
    echo ""
    echo "ERROR: Found $VIOLATIONS tier-conditional cfg attribute(s) in src/"
    echo "The single-binary discipline requires runtime capability checks,"
    echo "not compile-time feature gates. See CONVENTIONS §2.2."
    exit 1
fi

echo "OK: No tier-conditional cfg attributes found in src/"
exit 0
