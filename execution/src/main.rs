use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use std::collections::HashSet;

use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, GetModuleFileNameExW};
use windows_sys::Win32::System::Threading::{
    OpenProcess, TerminateProcess, CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW,
    PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PROCESS_TERMINATE,
};

fn wstr_to_string(wstr: &[u16]) -> String {
    let len = wstr.iter().position(|&c| c == 0).unwrap_or(wstr.len());
    OsString::from_wide(&wstr[..len])
        .to_string_lossy()
        .into_owned()
}

fn get_memory_usage(pid: u32) -> usize {
    unsafe {
        let handle: HANDLE = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if handle == 0 {
            return 0;
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

// Builds a flat list of all processes in the snapshot
fn snapshot_processes() -> Vec<PROCESSENTRY32W> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            eprintln!("Failed to create snapshot. Error code: {}", GetLastError());
            std::process::exit(1);
        }

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..std::mem::zeroed()
        };

        let mut processes = Vec::new();

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                processes.push(entry);
                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
        processes
    }
}

// Recursively find all child PIDs
fn find_process_tree(target_pid: u32, all_procs: &[PROCESSENTRY32W]) -> Vec<u32> {
    let mut tree = HashSet::new();
    tree.insert(target_pid);

    let mut current_size = 0;

    // Keep scanning until the tree stops growing
    while tree.len() > current_size {
        current_size = tree.len();
        for proc in all_procs {
            if tree.contains(&proc.th32ParentProcessID) {
                tree.insert(proc.th32ProcessID);
            }
        }
    }

    tree.into_iter().collect()
}

fn execute_kill(pid: u32) -> bool {
    if pid == 0 || pid == 4 {
        eprintln!("[-] REFUSED: Refusing to kill critical system PID {}.", pid);
        return false;
    }

    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle == 0 {
            return false;
        }

        let result = TerminateProcess(handle, 1);
        CloseHandle(handle);
        result != 0
    }
}

fn get_executable_path(pid: u32) -> Option<Vec<u16>> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if handle == 0 {
            return None;
        }

        let mut path_buf = vec![0u16; 1024];
        let len = GetModuleFileNameExW(handle, 0, path_buf.as_mut_ptr(), path_buf.len() as u32);
        CloseHandle(handle);

        if len > 0 {
            path_buf.truncate(len as usize);
            path_buf.push(0); // Null terminator
            Some(path_buf)
        } else {
            None
        }
    }
}

fn execute_restart(pid: u32, all_procs: &[PROCESSENTRY32W]) {
    let exe_path = get_executable_path(pid);
    if exe_path.is_none() {
        eprintln!("[-] Cannot restart PID {}: Failed to read executable path.", pid);
        return;
    }
    let exe_path_wstr = exe_path.unwrap();
    let display_path = wstr_to_string(&exe_path_wstr);

    println!("[*] Restarting application: {}", display_path);

    // 1. Kill the tree
    let tree = find_process_tree(pid, all_procs);
    for child_pid in tree {
        if execute_kill(child_pid) {
            println!("[+] Terminated PID {}", child_pid);
        }
    }

    // 2. Spawn new instance
    unsafe {
        let mut si = STARTUPINFOW {
            cb: std::mem::size_of::<STARTUPINFOW>() as u32,
            ..std::mem::zeroed()
        };
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();

        // Pass the path as string (requires mutable buffer for command line if not using lpApplicationName alone, but we'll try lpApplicationName)
        let mut cmd_line = exe_path_wstr.clone();

        let success = CreateProcessW(
            ptr::null(),
            cmd_line.as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            0,
            0,
            ptr::null(),
            ptr::null(),
            &mut si,
            &mut pi,
        );

        if success != 0 {
            println!("[+] Restart successful. New PID: {}", pi.dwProcessId);
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
        } else {
            eprintln!("[-] Restart failed. CreateProcessW Error: {}", GetLastError());
        }
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let all_procs = snapshot_processes();

    if args.len() >= 3 {
        let command = &args[1];
        let target_pid: u32 = args[2].parse().unwrap_or(0);

        if target_pid == 0 || target_pid == 4 {
             eprintln!("[-] REFUSED: Target PID {} is system critical.", target_pid);
             std::process::exit(1);
        }

        if command == "--kill" {
            let tree = find_process_tree(target_pid, &all_procs);
            println!("[*] Found {} processes in the tree for PID {}.", tree.len(), target_pid);
            for pid in tree {
                if execute_kill(pid) {
                    println!("[+] Terminated PID {}", pid);
                } else {
                    eprintln!("[-] Failed to terminate PID {}", pid);
                }
            }
            return;
        } else if command == "--restart" {
            execute_restart(target_pid, &all_procs);
            return;
        }
    }

    // Default Viewer Mode
    println!("{:<8} | {:<12} | {}", "PID", "MEMORY (KB)", "PROCESS NAME");
    println!("{:-<8}-+-{:-<12}-+-{:-<30}", "", "", "");

    for proc in all_procs {
        let pid = proc.th32ProcessID;
        let process_name = wstr_to_string(&proc.szExeFile);
        let memory_bytes = get_memory_usage(pid);
        let memory_kb = memory_bytes / 1024;
        println!("{:<8} | {:<12} | {}", pid, memory_kb, process_name);
    }
}
