# EXECUTIVE DIRECTIVES: `nt-proc-lens` v0.0.3

**TARGET:** `nt-proc-lens` (Micro Process Explorer & Restarter)
**PHASE:** Directives

The Forge **MUST** adhere to the following when extending functionality in `execution/src/main.rs`.

## 1. Zero Dependency Policy (CLI Parsing)
*   **PROHIBITED:** Crates like `clap` or `structopt`. They add >500KB of bloat.
*   **MANDATE:** You must parse `std::env::args()` manually.
    *   `nt-proc-lens` -> Lists all processes.
    *   `nt-proc-lens --kill <PID>` -> Kills the process tree.
    *   `nt-proc-lens --restart <PID>` -> Kills the process tree and spawns a new instance of the root PID.

## 2. Mandatory Windows APIs for Termination & Restarting
*   `OpenProcess` (With `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_TERMINATE`)
*   `TerminateProcess` (To kill)
*   `GetModuleFileNameExW` (From `psapi.dll`, to obtain the absolute path of the `.exe` before killing it).
*   `CreateProcessW` (To spawn the new instance during a restart).

## 3. The Recursion Safety Guard
*   **CRITICAL:** When building the process tree, The Forge MUST ensure we do not infinitely loop if PIDs are reused.
*   **CRITICAL:** The Forge MUST hardcode a safety guard rejecting the termination of PID `0` (Idle) and PID `4` (System). Killing these will blue-screen the OS.

## 4. Output Formatting Update
*   If listing processes, maintain the table format.
*   If an action (`--kill` or `--restart`) is specified, the tool should *only* print the action logs (e.g., `[+] Terminated PID 1234`, `[+] Restarted C:\...`), not the entire process table.

---

> **THE FORGE:** You are now authorized to begin code generation in `execution/`.
