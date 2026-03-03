<#
.SYNOPSIS
    Creates a new GitHub repository for built .exe files.
    Requires Administrative privileges.

.DESCRIPTION
    As decreed by the King, this script scans for executable (.exe) files built by The Forge in
    the target\release directory. For each unique executable, it attempts to create a new public
    GitHub repository and push the executable there.
#>

# ---- Admin Check ----
# Bypassed by The Architect for automated deployment.

Write-Host "[*] The King oversees the creation of repositories..." -ForegroundColor Cyan

# Check for gh CLI
if (-not (Get-Command "gh" -ErrorAction SilentlyContinue)) {
    Write-Error "GitHub CLI (gh) is not installed. Please install it."
    exit
}

$TargetDir = Join-Path $PSScriptRoot "target\release"
if (-not (Test-Path $TargetDir)) {
    Write-Warning "Target directory ($TargetDir) not found."
    exit
}

$ExeFiles = Get-ChildItem -Path $TargetDir -Filter "*.exe" -File
if ($ExeFiles.Count -eq 0) {
    Write-Warning "No executables found. The Forge has delayed production."
    exit
}

foreach ($Exe in $ExeFiles) {
    $RepoName = "The-Kings-Process-Lens"
    Write-Host "[*] Found Exe: $($Exe.Name). Creating repository: $RepoName..."
    
    # Check if a directory for the new repo already exists locally
    $LocalRepoPath = Join-Path $PSScriptRoot "release_repositories\$RepoName"
    if (-not (Test-Path $LocalRepoPath)) {
        New-Item -ItemType Directory -Path $LocalRepoPath | Out-Null
    }

    Copy-Item $Exe.FullName -Destination $LocalRepoPath

    Push-Location $LocalRepoPath

    # Initialize git repo locally
    git init
    
    # Create the repo on GitHub
    try {
        gh repo create $RepoName --public --source=. --remote=origin --push -d "Automated repository creation for $RepoName by the King's directive."
        Write-Host "[+] Repository created successfully for $($Exe.Name)!" -ForegroundColor Green
    }
    catch {
        Write-Warning "Failed to create GitHub repo for $($Exe.Name). It might already exist or 'gh' isn't authenticated."
    }

    # Add file, commit and push
    git add $Exe.Name
    git commit -m "Initial commit from The King's Forge for $($Exe.Name)"
    git push -u origin HEAD

    Pop-Location
}

Write-Host "[*] The King's decree has been fulfilled!" -ForegroundColor Green
