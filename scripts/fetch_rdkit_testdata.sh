#!/usr/bin/env bash
# Fetch RDKit's CDXML test files (BSD-3 licensed) into tests/data/rdkit/.
# Uses a shallow, partial, sparse checkout so only *.cdxml files are downloaded.
# Override the pinned commit with RDKIT_REF=<sha-or-branch>.
set -euo pipefail
# Keep Git Bash (MSYS) from rewriting "/license.txt" into a Windows path.
export MSYS_NO_PATHCONV=1

RDKIT_URL="https://github.com/rdkit/rdkit.git"
RDKIT_REF="${RDKIT_REF:-c0f3e73431978c031fb556e68e12db29333e2e22}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/tests/data/rdkit"

mkdir -p "$DEST"
cd "$DEST"

if [ ! -d .git ]; then
    git init -q
    git remote add origin "$RDKIT_URL"
fi

git sparse-checkout set --no-cone '*.cdxml' '/license.txt'
git fetch -q --depth 1 --filter=blob:none origin "$RDKIT_REF"
git checkout -q --force FETCH_HEAD

count=$(git ls-files | grep -ci '\.cdxml$' || true)
echo "RDKit test data ($RDKIT_REF): $count CDXML files in $DEST"
