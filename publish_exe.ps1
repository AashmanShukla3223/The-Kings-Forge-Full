$ErrorActionPreference = "Stop"

# Define variables
$ProjDir = "d:\Aashman's Storage\Antigravity\Agents Building EXE Files Every Two Days"
$ExeSourcePath = Join-Path $ProjDir "target\release\win32_bootstrap.exe"
$DateStr = Get-Date -Format "yyyy-MM-dd"
$RepoName = "win32-bootstrap-exe-$DateStr"
$TempDir = Join-Path $env:TEMP $RepoName

Write-Host "Starting publish process for $DateStr..."

# Ensure we are in the project directory
Set-Location $ProjDir

# Build the project to get the fresh EXE
Write-Host "Building project..."
cargo build --release

if (-Not (Test-Path $ExeSourcePath)) {
    Write-Error "Executable not found at $ExeSourcePath. Please verify build success."
    exit 1
}

# Create a temporary directory for the new Git repo
if (Test-Path $TempDir) {
    Remove-Item -Recurse -Force $TempDir
}
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

# Copy the EXE to the temp directory
Copy-Item -Path $ExeSourcePath -Destination $TempDir

# Initialize Git and commit the EXE
Set-Location $TempDir
git init
git branch -M main
git add .
git commit -m "Auto-published EXE for $DateStr"

# Create a new repository on GitHub and push the commit
# We capture the output to avoid errors breaking the script if repo already exists
Write-Host "Creating GitHub repository $RepoName..."
try {
    gh repo create $RepoName --public --source=. --remote=origin --push
    Write-Host "Successfully created repo $RepoName and pushed the EXE."
}
catch {
    Write-Warning "Failed to create or push to repo. It might already exist or gh might not be authenticated."
    Write-Warning $_
}

# --- Track cycles in SYSTEM_MEMORY.md ---
$SysMemPath = Join-Path $ProjDir "SYSTEM_MEMORY.md"

if (Test-Path $SysMemPath) {
    $SysMemContent = Get-Content $SysMemPath
    
    # Simple cycle counter tracking logic (assumes Cycle ID pattern C-XXX)
    $CycleRegex = "\|\s+\*\*C-(\d+)\*\*\s+\|"
    $MaxCycle = 0
    
    foreach ($line in $SysMemContent) {
        if ($line -match $CycleRegex) {
            $cycleNum = [int]$matches[1]
            if ($cycleNum -gt $MaxCycle) {
                $MaxCycle = $cycleNum
            }
        }
    }

    $CurrentCycle = $MaxCycle + 1
    Write-Host "Completed Cycle C-$CurrentCycle (Day $($CurrentCycle * 2))"
    
    # Write a reminder if we reach Day 20 (Cycle 10)
    if ($CurrentCycle -ge 10 -and ($CurrentCycle % 10) -eq 0) {
        Write-Host "================================================================" -ForegroundColor Yellow
        Write-Host "DAY 20 REACHED: It is time to refresh SYSTEM_MEMORY.md niches!" -ForegroundColor Yellow
        Write-Host "Please prompt the AI Agent to generate 10 new niche categories." -ForegroundColor Yellow
        Write-Host "================================================================" -ForegroundColor Yellow
        
        # Optionally, we could append a prompt to a "TODO" file or send a notification here.
        $TodoPath = Join-Path $ProjDir "TODO_REFRESH_NICHES.txt"
        "Day 20 Reached on $DateStr. Please provide 10 new target niches to the agent." | Out-File -FilePath $TodoPath -Append
    }
}
else {
    Write-Warning "SYSTEM_MEMORY.md not found, skipping cycle tracking."
}

# Clean up
Set-Location $ProjDir
Remove-Item -Recurse -Force $TempDir
