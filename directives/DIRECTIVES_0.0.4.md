# EXECUTIVE DIRECTIVES: `nt-proc-lens` v0.0.4

**TARGET:** `nt-proc-lens` (Native Win32 GUI Process Explorer)
**PHASE:** Directives

## 1. Console Suppression
*   **MANDATE:** The Rust attribute `#![windows_subsystem = "windows"]` MUST be present to suppress the console window entirely.

## 2. Mandatory Win32 APIs for GUI
*   `RegisterClassExW` / `CreateWindowExW` (Window creation)
*   `GetMessageW` / `TranslateMessage` / `DispatchMessageW` (Event loop)
*   `InitCommonControlsEx` with `ICC_LISTVIEW_CLASSES` (Modern controls)
*   `CreateWindowExW` with class `SysListView32` (ListView control)
*   `SendMessageW` with `LVM_INSERTCOLUMN`, `LVM_INSERTITEM`, `LVM_SETITEMTEXT` (Populating data)
*   `CreatePopupMenu` / `TrackPopupMenu` (Right-click context menu)
*   `SetTimer` / `WM_TIMER` (Auto-refresh every 2 seconds)

## 3. Zero Dependency Policy
*   Same as prior cycles. Only `windows-sys` crate allowed.

## 4. Safety Guards
*   PIDs 0 and 4 remain protected from termination.
*   Context menu actions must confirm before killing.

---

> **THE FORGE:** You are now authorized to begin GUI code generation.
