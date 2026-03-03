# Objective: `nt-proc-lens` v0.0.3 (Process Killer & Restarter)

Expand `nt-proc-lens` from a passive observer to an active administrator tool capable of traversing and terminating entire process trees, and restarting targeted executables, given a Process ID (PID). 

## User Review Required
> [!IMPORTANT]
> To execute a full process tree kill, the tool will iterate through all processes via `CreateToolhelp32Snapshot`, find processes whose `th32ParentProcessID` matches the target PID, kill the children recursively, and finally terminate the parent. 
> To "restart", it will read `GetModuleFileNameExW` from the target PID before killing it, and spawn a new instance of that exact file path via `CreateProcessW` once termination is confirmed. 
> Is this exact flow approved by the King?

## Proposed Changes

### Directives & Architecture
#### [NEW] [BUILD_SPEC_v0.0.3.md](file:///d:/Antigravity/Agents/planning/BUILD_SPEC_v0.0.3.md)
Will outline the architecture for finding child processes and executing the restart loop.
#### [NEW] [DIRECTIVES_0.0.3.md](file:///d:/Antigravity/Agents/directives/DIRECTIVES_0.0.3.md)
Constraints on safety checks. The tool *must* warn or guard against killing system-critical PIDs (like `csrss.exe` or `wininit.exe`).

### Source Generation (Execution Phase)
#### [MODIFY] [main.rs](file:///d:/Antigravity/Agents/execution/src/main.rs)
Add argument parsing (manual parsing, NO `clap` dependency) for:
*   `--kill <PID>`
*   `--restart <PID>`
Add `OpenProcess`, `TerminateProcess`, and recursive process tree matching.

## Verification Plan

### Automated Tests
*   `cargo build --release` inside `execution/` must succeed.
*   Size must remain under 1.5MB.

### Manual Verification
*   We will spawn dummy processes and command `nt-proc-lens.exe` to terminate and restart them, verifying the new PIDs in the console output.
