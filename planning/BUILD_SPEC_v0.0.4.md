# Objective: `nt-proc-lens` v0.0.4 (Native Win32 GUI)

Transition `nt-proc-lens` from a Console Application to a true Windowed Application using the pure Win32 API. No high-level UI frameworks allowed.

## Proposed Changes

### Directives & Architecture
#### [NEW] [BUILD_SPEC_v0.0.4.md](file:///d:/Antigravity/Agents/planning/BUILD_SPEC_v0.0.4.md)
Will outline the architecture for the Win32 window class, message loop, and `INITCOMMONCONTROLSEX` required for modern UI controls.
#### [NEW] [DIRECTIVES_0.0.4.md](file:///d:/Antigravity/Agents/directives/DIRECTIVES_0.0.4.md)
Constraints ensuring the `#![windows_subsystem = "windows"]` attribute is used to hide the console window completely, guaranteeing only the GUI is shown.

### Source Generation (Execution Phase)
#### [MODIFY] [Cargo.toml](file:///d:/Antigravity/Agents/execution/Cargo.toml)
Add extensive `windows-sys` features required for drawing windows, processing messages, handling controls (`Win32_UI_WindowsAndMessaging`, `Win32_UI_Controls`, `Win32_Graphics_Gdi`).
#### [MODIFY] [main.rs](file:///d:/Antigravity/Agents/execution/src/main.rs)
Complete rewrite of the core `main()` logic into a WinMain/Win32 event hook architecture.
*   **Window Creation**: `CreateWindowExW`.
*   **List-View Control**: `SysListView32` to render the table.
*   **Context Menu**: `CreatePopupMenu` handling right-clicks on the list items to execute `--kill` or `--restart`.

## Verification Plan

### Automated Tests
*   `cargo build --release` inside `execution/` must succeed.
*   Size must remain under 1.5MB (even with UI rendering enabled).

### Manual Verification
*   Execute `nt-proc-lens.exe`. An actual graphical window must appear with a sortable list of processes.
*   The raw terminal console must NOT appear.
