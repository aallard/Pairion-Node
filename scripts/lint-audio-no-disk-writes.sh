#!/usr/bin/env bash
# lint-audio-no-disk-writes.sh
#
# Fails if any filesystem write patterns are found in src/audio/.
# This enforces Architecture §16.3: raw audio is NEVER written to disk.
#
# Part of CONVENTIONS §4 CI pipeline.

set -euo pipefail

VIOLATIONS=0

# Search for filesystem write patterns in the audio module
while IFS= read -r line; do
    echo "VIOLATION: $line"
    VIOLATIONS=$((VIOLATIONS + 1))
done < <(grep -rn 'std::fs::File::create\|std::io::Write\|tokio::fs\|OpenOptions.*write\|File::create\|write_all\|BufWriter' src/audio/ 2>/dev/null || true)

if [ "$VIOLATIONS" -gt 0 ]; then
    echo ""
    echo "ERROR: Found $VIOLATIONS filesystem write pattern(s) in src/audio/"
    echo "Raw audio is NEVER written to disk. See Architecture §16.3."
    exit 1
fi

echo "OK: No filesystem write patterns found in src/audio/"
exit 0
