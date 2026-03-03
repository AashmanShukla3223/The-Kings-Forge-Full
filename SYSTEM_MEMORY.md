# SYSTEM_MEMORY.md

> **WARNING:** This file is the Source of Truth. All Agents MUST read this before modifying code. All Agents MUST update this before termination. Failure to update this file constitutes a failed cycle.

## 1. System State
* **Current Version:** `v0.0.1-alpha`
* **Last Successful Build:** `2026-03-02` (Init)
* **Last Failed Build:** `N/A`
* **Current Stability Score:** `0/10` (Target: 9/10)
* **Active Directive:** "Initial Bootstrap: Create a window that opens, stays open, and closes cleanly."

---

## 2. The Prime Directives (Immutable)
1.  **Performance is King:** If it runs slower than the previous version, revert it.
2.  **Windows Native:** Use Win32 API / COM. No Electron, no Qt, no .NET bloat unless unavoidable.
3.  **Zero Dependency:** Statically link everything. The output must be a single portable `.exe`.
4.  **Ephemerality:** You are temporary. Your code is forever. Document *why* you made a choice, or the next agent will delete it.

---

## 3. The Ledger (Cycle History)

| Cycle ID | Date | Agent Focus | Outcome | Size Delta | Key Decision |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **INIT** | 2026-03-02 | Repository Setup | **SUCCESS** | +4KB | Established project structure. |
| **C-001** | 2026-03-02 | Initial Bootstrap | **SUCCESS** | 234KB | Used Raw FFI instead of windows-sys to bypass missing MSVC SDK dependency. |
| **C-002** | 2026-03-03 | Sysadmin Micro-Utility | **SUCCESS** | 244KB | Implemented `nt-proc-lens` process explorer using pure Win32 API (`CreateToolhelp32Snapshot`). |
| **C-003** | 2026-03-03 | Process Termination | **SUCCESS** | 256KB | Added manual cli argument parsing, `TerminateProcess`, and `CreateProcessW` for recursive tree killing/restarting. |
| **C-004** | *Pending* | *Pending* | *Pending* | *Pending* | *Pending* |

---

## 4. The Graveyard (Failed Strategies)
> **CRITICAL:** Do NOT attempt these approaches again. They have been tried and failed.

* *Example:* Do not use `SetWindowCompositionAttribute` for blur on Windows 11; it causes flickering. Use `DwmSetWindowAttribute` instead.
* *[Empty]*

---

## 5. The Codex (Learned Patterns & Snippets)
> **NOTE:** Proven solutions to recurring problems. Copy-paste these patterns.

### Window Creation
* *Preferred Pattern:* `CreateWindowExW` with `CS_HREDRAW | CS_VREDRAW`.
* *Reason:* Handles resizing without artifacts on high-DPI displays.

### Error Handling
* *Preferred Pattern:* `GetLastError()` wrapped in a custom `Result<T, E>` type.
* *Reason:* Windows errors are opaque; wrapping them enforces handling.

### Process Management
* *Preferred Pattern:* `CreateToolhelp32Snapshot` combined with `Process32FirstW` / `Process32NextW`. Use `OpenProcess` with `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` and pass the handle to `GetProcessMemoryInfo` for lightweight memory profiling.
* *Reason:* Avoids heavy abstractions. Provides direct, exact access to OS process data.

### Process Termination & Restarting
* *Preferred Pattern:* When killing processes, traverse the tree by checking `th32ParentProcessID` against the target PID to avoid zombies. Use `GetModuleFileNameExW` before killing to store the `.exe` path if a restart via `CreateProcessW` is required.
* *Reason:* Native Windows handles are strict. A process cannot easily be restarted unless its full physical path is known prior to termination.

---

## 6. Target Niche Application Categories
> **STRATEGY:** General-purpose software is bloated. We build hyper-specific, highly optimized native tools that solve exactly one niche problem perfectly.

Based on our intelligence capabilities, future builds should eventually target these niches:
1.  **Sysadmin & Power User Micro-Utilities:** Tools like *WizTree* (hyper-fast disk scanning), *Process Explorer* (advanced task management), or *EarTrumpet* (per-app volume control).
2.  **Workflow Automation:** Tools integrating with the Windows clipboard history, window snapping (like *PowerToys FancyZones*), and macro automation (like *AutoHotkey* execution engines).
3.  **Specialized Industry Dashboards:** Highly optimized local native dashboards for reading localized data (e.g., inventory tracking, specialized point-of-sale data, or high-performance financial data visualizations) without relying on heavy web views.
4.  **Hardware & Sensor Monitoring:** Low-level hardware interfacing tools like *HWiNFO* or *Fan Control* that require direct driver or strict native API access.
5.  **Ultra-Low Latency Media Applications:** Direct-to-disk video recorders, audio DSP effects, or simple low-latency live streaming relays that avoid complex runtimes.
6.  **Network Diagnostics & Packet Sniffing:** Extremely lightweight Wireshark alternatives that monitor specific ports, pings, or protocols with absolutely zero GUI overhead.
7.  **Cryptography & Security Enclaves:** Offline password vault interfaces, local-only hashing/encryption utilities, or air-gapped cryptography tools.
8.  **Minimalist Writing Environments:** Distraction-free native text editors that start up in under a millisecond while consistently taking under 1MB of memory.
9.  **Reverse Engineering & Memory Hex Editors:** Fast, huge-file capable hex editors and memory inspectors similar to an optimized, single-purpose *HxD*.
10. **Data Recovery & File Signatures:** Sector-by-sector disk reading tools for finding deleted files without relying on OS indexing.
