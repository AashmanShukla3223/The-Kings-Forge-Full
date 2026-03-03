# EXECUTIVE DIRECTIVES: `nt-proc-lens`

**TARGET:** `nt-proc-lens` (Micro Process Explorer)
**PHASE:** Directives

As per the King's mandate, The Forge **MUST** adhere to the following strict constraints when writing code in the `execution/` phase.

## 1. Zero Dependency Policy
*   You are authorized to use the standard library (`std`).
*   You are authorized to use low-level OS bindings (e.g., `windows-sys` crate or raw FFI).
*   **PROHIBITED:** Any high-level crates (e.g., `sysinfo`, `tokio`, parsing libraries) that add bulk or overhead. You must parse the OS snapshot yourself if necessary.

## 2. Memory & Performance Footprint
*   **Startup Time:** Must be near-instantaneous (< 10ms).
*   **Memory Usage:** The final `.exe` must consume less than 2MB of RAM while running.
*   **Binary Size:** Strip symbols and optimize for size in `Cargo.toml`.

## 3. Mandatory Windows APIs
You **MUST** utilize the following Win32 APIs directly:
*   `CreateToolhelp32Snapshot` (Snapshot of system processes)
*   `Process32FirstW` / `Process32NextW` (Iterating processes)
*   `OpenProcess` (To acquire handles for memory reading or termination)
*   `TerminateProcess` (For the kill switch functionality)
*   `GetProcessMemoryInfo` (From `psapi.dll`, to retrieve `WorkingSetSize`)

## 4. Output Formatting
*   The output must be a strictly formatted console table printed to Standard Out.
*   Format: `[PID] | [MEMORY_KB] | [PROCESS_NAME]`
*   Output must be sortable by the OS pipe (e.g., `nt-proc-lens.exe | sort`).

## 5. Failure Conditions (Gatekeeper Triggers)
The Gatekeeper will fail the build and delete the `execution/` folder if:
1.  The `.exe` size exceeds 1.5MB.
2.  Any unsafe block causes a runtime panic due to unhandled null pointers from the Win32 API.
3.  The process handles are not explicitly closed via `CloseHandle` (Handle Leaks are immediate termination offenses).

---

> **THE FORGE:** You are now authorized to begin code generation in `execution/`.
