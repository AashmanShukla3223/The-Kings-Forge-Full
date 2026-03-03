$ErrorActionPreference = "Stop"

$TaskName = "PublishBootstrapExe"
$ScriptPath = "d:\Aashman's Storage\Antigravity\Agents Building EXE Files Every Two Days\publish_exe.ps1"
$Command = "powershell.exe -ExecutionPolicy Bypass -WindowStyle Hidden -File \`"$ScriptPath\`""

Write-Host "Creating scheduled task '$TaskName' to run every 2 days..."

$Arguments = @(
    "/create",
    "/tn", $TaskName,
    "/tr", "$Command",
    "/sc", "daily",
    "/mo", "2",
    "/rl", "HIGHEST",
    "/f"
)

# Run schtasks
$process = Start-Process -FilePath "schtasks.exe" -ArgumentList $Arguments -Wait -NoNewWindow -PassThru

if ($process.ExitCode -eq 0) {
    Write-Host "Successfully created the scheduled task!" -ForegroundColor Green
}
else {
    Write-Host "Failed to create the scheduled task. Make sure you run this script as Administrator." -ForegroundColor Red
}

Read-Host -Prompt "Press Enter to exit"
