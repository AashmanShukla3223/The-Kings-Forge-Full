# Objective: `nt-proc-lens` v0.0.5 (Search & Manual Refresh)

Update the Win32 GUI according to the user's latest dictates:
1. Remove the 2-second auto-refresh timer.
2. Add a manual "Refresh" button.
3. Add a "Search" text box to filter processes by name.

## Proposed Changes

### Directives & Architecture
#### [NEW] [BUILD_SPEC_v0.0.5.md](file:///d:/Antigravity/Agents/planning/BUILD_SPEC_v0.0.5.md)
Will outline the architecture for the new Win32 Edit and Button controls, and the messaging layout.
#### [NEW] [DIRECTIVES_0.0.5.md](file:///d:/Antigravity/Agents/directives/DIRECTIVES_0.0.5.md)
Constraints ensuring no bloated frameworks or `regex` libraries are used. String matching will be done using simple rust `str::contains` (case-insensitive).

### Source Generation (Execution Phase)
#### [MODIFY] [main.rs](file:///d:/Antigravity/Agents/execution/src/main.rs)
*   **Remove Auto-Refresh**: Delete `SetTimer`, `KillTimer`, and `WM_TIMER` handling.
*   **Add Controls**: Use `CreateWindowExW` with classes `"EDIT"` and `"BUTTON"`.
*   **Layout**: Adjust `WM_SIZE` to cleanly align the Edit box, Refresh button, and List-View.
*   **Search Logic**: Handle `EN_CHANGE` (Edit Control text changed) via `WM_COMMAND`, read the text via `GetWindowTextW`, and pass it to `populate_listview(filter: &str)`.

## Verification Plan
*   Run the executable. The GUI should have a Search box and a Refresh button at the top.
*   Auto-refresh should be gone.
*   Typing in the search box should instantly filter the list.
