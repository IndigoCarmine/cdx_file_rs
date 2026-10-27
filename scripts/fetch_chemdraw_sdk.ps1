# Fetch the ChemDraw file-format sources (BSD-3 licensed) into tools/data/chemdraw/.
# Uses a shallow, partial, sparse checkout so only the headers and sources are downloaded.
# Override the pinned commit with $env:CHEMDRAW_REF = '<sha-or-branch>'.
$ErrorActionPreference = 'Stop'

$ChemdrawUrl = 'https://github.com/Glysade/chemdraw.git'
$ChemdrawRef = if ($env:CHEMDRAW_REF) { $env:CHEMDRAW_REF } else { '5dd743230d75ecacacdf8dcd0b0e022f13173b9c' }

$Root = Split-Path -Parent $PSScriptRoot
$Dest = Join-Path $Root 'tools/data/chemdraw'

function Invoke-Git {
    git @args
    if ($LASTEXITCODE -ne 0) { throw "git $args failed with exit code $LASTEXITCODE" }
}

New-Item -ItemType Directory -Force $Dest | Out-Null
Push-Location $Dest
try {
    if (-not (Test-Path .git)) {
        Invoke-Git init -q
        Invoke-Git remote add origin $ChemdrawUrl
    }
    Invoke-Git sparse-checkout set --no-cone '/chemdraw/*.h' '/chemdraw/*.cpp' '/chemdraw/license.txt'
    Invoke-Git fetch -q --depth 1 --filter=blob:none origin $ChemdrawRef
    Invoke-Git checkout -q --force FETCH_HEAD

    $count = @(git ls-files chemdraw).Count
    Write-Output "ChemDraw sources ($ChemdrawRef): $count files in $Dest"
}
finally {
    Pop-Location
}
