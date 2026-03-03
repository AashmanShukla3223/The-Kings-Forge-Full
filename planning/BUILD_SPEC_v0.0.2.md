# Objective: `nt-proc-lens` (Micro Process Explorer)

Create a hyper-specific, zero-dependency, native Windows utility that lists running processes, their memory usage, and allows instantaneous termination. It must be blazing fast, requiring zero setup.

## Architecture mapping (PDED)

*   **`planning/`**: Contains this `BUILD_SPEC_v0.0.2.md` and architecture notes.
*   **`directives/`**: High-level constraints and memory dumps from the Architect/King.
*   **`execution/`**: The Rust (or C++ based on Forge preference, defaulting to Rust for memory safety) source code.
*   **`debugging/`**: Logs, crash dumps, and the Gatekeeper artifact outputs.

## User Review Required

> [!IMPORTANT]
> The Forge is configured to use Rust (based on the previous bootsrap `.exe` mentioned in `SYSTEM_MEMORY.md`). Should we proceed with Rust using raw Win32 bindings (e.g., `CreateToolhelp32Snapshot`), or do you prefer raw C/C++?

## Proposed Changes

### Directives & Architecture
#### [NEW] [BUILD_SPEC_v0.0.2.md](file:///d:/Antigravity/Agents/planning/BUILD_SPEC_v0.0.2.md)
Will contain the exact instructions for The Forge:
1. Initialize standard Rust binary project in `execution/`
2. Implement Win32 `CreateToolhelp32Snapshot` to list processes.
3. Keep memory footprint under 2MB.

### Source Generation (Execution Phase)
#### [NEW] [main.rs](file:///d:/Antigravity/Agents/execution/src/main.rs)
Core process looping logic.
#### [NEW] [Cargo.toml](file:///d:/Antigravity/Agents/execution/Cargo.toml)
Project manifest (Zero dependencies other than `windows-sys` or raw FFI).

## Verification Plan

### Automated Tests
*   `cargo build --release` inside `execution/` must succeed.
*   The output binary size must be verified against the 10% delta constraint (though this is a new binary, we establish a baseline < 2MB).

### Manual Verification
*   Run `nt-proc-lens.exe`. It should immediately print a formatted table of `PID | Name | Threads` and exit gracefully.
