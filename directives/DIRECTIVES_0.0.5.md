# EXECUTIVE DIRECTIVES: `nt-proc-lens` v0.0.5

**TARGET:** `nt-proc-lens` (Search, Refresh, & Bugfixes)
**PHASE:** Directives

## 1. GUI Layout & Controls
*   **MANDATE:** Remove `WM_TIMER` auto-refresh.
*   **MANDATE:** Render a `EDIT` control at the top-left for "Search".
*   **MANDATE:** Render a `BUTTON` control at the top-right for "Refresh".
*   **MANDATE:** On `WM_COMMAND` from the Edit control (`EN_CHANGE`), read the string and pass it to `populate_listview` as a filter.
*   **MANDATE:** The ListView should take up the remaining space below the controls.

## 2. The `CreateProcessW` Quotation Bug (Explorer.exe fix)
*   **CRITICAL FIX:** In Cycle C-003/004, `exe_path` retrieved from `GetModuleFileNameExW` was passed directly to `CreateProcessW`. If this path contains spaces (like `C:\Program Files\...`) or requires explicit shell targeting, it fails. 
*   **SOLUTION:** The Forge MUST wrap the executable path in double-quotes (`""`) before passing it as `lpCommandLine` to `CreateProcessW`.

## 3. Zero Dependency Policy
*   Remain constrained strictly to Win32 API. No RegEx crates. Use `str::to_lowercase().contains()` for search.

---

> **THE FORGE:** You are now authorized to begin code generation.
