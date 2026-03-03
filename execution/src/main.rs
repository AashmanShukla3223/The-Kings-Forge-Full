use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;

use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

// Helper to convert UTF-16 Win32 strings to Rust strings, stopping at null
fn wstr_to_string(wstr: &[u16]) -> String {
    let len = wstr.iter().position(|&c| c == 0).unwrap_or(wstr.len());
    OsString::from_wide(&wstr[..len])
        .to_string_lossy()
        .into_owned()
}

fn get_memory_usage(pid: u32) -> usize {
    unsafe {
        // Open the process specifically to read its memory info
        let handle: HANDLE = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if handle == 0 {
            return 0; // Access denied or process died
        }

        let mut mem_counters: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        let cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

        let memory_usage = if GetProcessMemoryInfo(handle, &mut mem_counters, cb) != 0 {
            mem_counters.WorkingSetSize
        } else {
            0
        };

        CloseHandle(handle);
        memory_usage
    }
}

fn main() {
    unsafe {
        // Create a snapshot of all current processes
        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            eprintln!("Failed to create snapshot. Error code: {}", GetLastError());
            std::process::exit(1);
        }

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..std::mem::zeroed()
        };

        // Get the first process
        if Process32FirstW(snapshot, &mut entry) == 0 {
            eprintln!("Failed to read process list. Error code: {}", GetLastError());
            CloseHandle(snapshot);
            std::process::exit(1);
        }

        println!("{:<8} | {:<12} | {}", "PID", "MEMORY (KB)", "PROCESS NAME");
        println!("{:-<8}-+-{:-<12}-+-{:-<30}", "", "", "");

        // Iterate through all processes
        loop {
            let pid = entry.th32ProcessID;
            let process_name = wstr_to_string(&entry.szExeFile);
            let memory_bytes = get_memory_usage(pid);
            let memory_kb = memory_bytes / 1024;

            println!("{:<8} | {:<12} | {}", pid, memory_kb, process_name);

            // Move to next process
            if Process32NextW(snapshot, &mut entry) == 0 {
                break;
            }
        }

        CloseHandle(snapshot);
    }
}
