#!/usr/bin/env bash
# Fetch the ChemDraw file-format sources (BSD-3 licensed) into tools/data/chemdraw/.
# Uses a shallow, partial, sparse checkout so only the headers and sources are downloaded.
# Override the pinned commit with CHEMDRAW_REF=<sha-or-branch>.
set -euo pipefail
# Keep Git Bash (MSYS) from rewriting "/chemdraw/..." patterns into a Windows path.
export MSYS_NO_PATHCONV=1

CHEMDRAW_URL="https://github.com/Glysade/chemdraw.git"
CHEMDRAW_REF="${CHEMDRAW_REF:-5dd743230d75ecacacdf8dcd0b0e022f13173b9c}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/tools/data/chemdraw"

mkdir -p "$DEST"
cd "$DEST"

if [ ! -d .git ]; then
    git init -q
    git remote add origin "$CHEMDRAW_URL"
fi

git sparse-checkout set --no-cone '/chemdraw/*.h' '/chemdraw/*.cpp' '/chemdraw/license.txt'
git fetch -q --depth 1 --filter=blob:none origin "$CHEMDRAW_REF"
git checkout -q --force FETCH_HEAD

count=$(git ls-files chemdraw | wc -l)
echo "ChemDraw sources ($CHEMDRAW_REF): $count files in $DEST"
