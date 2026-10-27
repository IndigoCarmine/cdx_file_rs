# Fetch RDKit's CDXML test files (BSD-3 licensed) into tests/data/rdkit/.
# Uses a shallow, partial, sparse checkout so only *.cdxml files are downloaded.
# Override the pinned commit with $env:RDKIT_REF = '<sha-or-branch>'.
$ErrorActionPreference = 'Stop'

$RdkitUrl = 'https://github.com/rdkit/rdkit.git'
$RdkitRef = if ($env:RDKIT_REF) { $env:RDKIT_REF } else { 'c0f3e73431978c031fb556e68e12db29333e2e22' }

$Root = Split-Path -Parent $PSScriptRoot
$Dest = Join-Path $Root 'tests/data/rdkit'

function Invoke-Git {
    git @args
    if ($LASTEXITCODE -ne 0) { throw "git $args failed with exit code $LASTEXITCODE" }
}

New-Item -ItemType Directory -Force $Dest | Out-Null
Push-Location $Dest
try {
    if (-not (Test-Path .git)) {
        Invoke-Git init -q
        Invoke-Git remote add origin $RdkitUrl
    }
    Invoke-Git sparse-checkout set --no-cone '*.cdxml' '/license.txt'
    Invoke-Git fetch -q --depth 1 --filter=blob:none origin $RdkitRef
    Invoke-Git checkout -q --force FETCH_HEAD

    $count = @(git ls-files | Where-Object { $_ -match '\.cdxml$' }).Count
    Write-Output "RDKit test data ($RdkitRef): $count CDXML files in $Dest"
}
finally {
    Pop-Location
}
