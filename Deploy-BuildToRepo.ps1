<#
.SYNOPSIS
    Deploys built .exe artifacts to the existing GitHub repository by creating a new GitHub Release.
    Requires Administrative privileges as requested by the King's directive.

.DESCRIPTION
    This script finds all executable (.exe) files built in the target\release directory 
    and uploads them as a new release to the currently configured GitHub repository.
    It uses the GitHub CLI (`gh`). Ensure `gh` is authenticated before running.
#>

# ---- Admin Check ----
$IsAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $IsAdmin) {
    Write-Warning "The King demands elevated privileges. Restarting as Administrator..."
    Start-Process powershell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

Write-Host "[*] The King's Forge: Uploading Executables to the Repository..." -ForegroundColor Cyan

# Ensure GitHub CLI is installed
if (-not (Get-Command "gh" -ErrorAction SilentlyContinue)) {
    Write-Error "GitHub CLI (gh) is not installed or not in PATH. Please install it to proceed."
    exit
}

# Find built .exe files
$TargetDir = Join-Path $PSScriptRoot "target\release"
if (-not (Test-Path $TargetDir)) {
    Write-Warning "Target release directory not found! Ensure a successful build exists."
    exit
}

$ExeFiles = Get-ChildItem -Path $TargetDir -Filter "*.exe" -File
if ($ExeFiles.Count -eq 0) {
    Write-Warning "No .exe files found in $TargetDir. Nothing to upload."
    exit
}

# The user already created a repo, so we assume the current directory is a git repo.
# Check if it's a git repo connecting to a GitHub remote.
$GitOrigin = git remote get-url origin 2>$null
if (-not $GitOrigin) {
    Write-Error "This directory is not a Git repository or has no 'origin' remote."
    exit
}

# Create a timestamp-based release tag (e.g. v2026.03.03.1104)
$Tag = "v" + (Get-Date -Format "yyyy.MM.dd.HHmm")
Write-Host "[*] Creating GitHub Release with Tag: $Tag" -ForegroundColor Green

# Prepare the array of file paths for the gh release command
$FilePaths = $ExeFiles.FullName

# Run the gh release create command
# Syntax: gh release create <tag> <files> --title <title> --notes <notes>
try {
    Write-Host "[*] Uploading $($ExeFiles.Count) executable(s) to the realm's repository..."
    gh release create $Tag $FilePaths --title "Forge Build - $Tag" --notes "Automated release of executables built by The Forge."
    Write-Host "[+] The King is pleased. The upload was successful!" -ForegroundColor Green
} catch {
    Write-Error "Failed to upload the executables. Ensure you are authenticated with 'gh auth login' and have permission to push."
}
