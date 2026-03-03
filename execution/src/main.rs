#![windows_subsystem = "windows"]

use std::collections::HashSet;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::Diagnostics::ToolHelp::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::ProcessStatus::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::UI::Controls::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

// ---- Constants ----
const IDC_LISTVIEW: u32 = 1001;
const IDM_KILL: u32 = 2001;
const IDM_RESTART: u32 = 2002;
const IDM_REFRESH: u32 = 2003;
const TIMER_REFRESH: usize = 5001;
const REFRESH_INTERVAL_MS: u32 = 2000;

// ---- Helpers ----
fn to_wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn wstr_to_string(wstr: &[u16]) -> String {
    let len = wstr.iter().position(|&c| c == 0).unwrap_or(wstr.len());
    OsString::from_wide(&wstr[..len])
        .to_string_lossy()
        .into_owned()
}

// ---- Process Snapshot Logic ----
struct ProcessInfo {
    pid: u32,
    parent_pid: u32,
    name: String,
    memory_kb: usize,
}

fn snapshot_processes() -> Vec<ProcessInfo> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Vec::new();
        }

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..std::mem::zeroed()
        };

        let mut processes = Vec::new();

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let pid = entry.th32ProcessID;
                let parent_pid = entry.th32ParentProcessID;
                let name = wstr_to_string(&entry.szExeFile);
                let memory_kb = get_memory_usage(pid) / 1024;

                processes.push(ProcessInfo {
                    pid,
                    parent_pid,
                    name,
                    memory_kb,
                });

                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
        processes
    }
}

fn get_memory_usage(pid: u32) -> usize {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if handle == 0 {
            return 0;
        }
        let mut mc: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        let cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
        let mem = if GetProcessMemoryInfo(handle, &mut mc, cb) != 0 {
            mc.WorkingSetSize
        } else {
            0
        };
        CloseHandle(handle);
        mem
    }
}

fn find_process_tree(target_pid: u32, all_procs: &[ProcessInfo]) -> Vec<u32> {
    let mut tree = HashSet::new();
    tree.insert(target_pid);
    let mut current_size = 0;
    while tree.len() > current_size {
        current_size = tree.len();
        for proc in all_procs {
            if tree.contains(&proc.parent_pid) {
                tree.insert(proc.pid);
            }
        }
    }
    tree.into_iter().collect()
}

fn execute_kill(pid: u32) -> bool {
    if pid == 0 || pid == 4 {
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
            path_buf.push(0);
            Some(path_buf)
        } else {
            None
        }
    }
}

fn execute_restart(pid: u32) {
    let procs = snapshot_processes();
    let exe_path = get_executable_path(pid);
    if exe_path.is_none() {
        return;
    }
    let exe_path_wstr = exe_path.unwrap();
    let tree = find_process_tree(pid, &procs);
    for child_pid in tree {
        execute_kill(child_pid);
    }

    unsafe {
        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
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
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
        }
    }
}

// ---- GUI: ListView Population ----
unsafe fn populate_listview(hwnd_lv: HWND) {
    // Clear existing items
    SendMessageW(hwnd_lv, LVM_DELETEALLITEMS, 0, 0);

    let processes = snapshot_processes();

    for (i, proc) in processes.iter().enumerate() {
        let name_w = to_wstr(&proc.name);
        let pid_str = to_wstr(&proc.pid.to_string());
        let mem_str = to_wstr(&format!("{} KB", proc.memory_kb));

        // Insert row with Process Name
        let mut item: LVITEMW = std::mem::zeroed();
        item.mask = LVIF_TEXT;
        item.iItem = i as i32;
        item.iSubItem = 0;
        item.pszText = name_w.as_ptr() as *mut u16;
        SendMessageW(hwnd_lv, LVM_INSERTITEMW, 0, &item as *const _ as isize);

        // Set PID column
        item.iSubItem = 1;
        item.pszText = pid_str.as_ptr() as *mut u16;
        SendMessageW(hwnd_lv, LVM_SETITEMTEXTW, i, &item as *const _ as isize);

        // Set Memory column
        item.iSubItem = 2;
        item.pszText = mem_str.as_ptr() as *mut u16;
        SendMessageW(hwnd_lv, LVM_SETITEMTEXTW, i, &item as *const _ as isize);
    }
}

unsafe fn get_selected_pid(hwnd_lv: HWND) -> Option<u32> {
    let sel = SendMessageW(hwnd_lv, LVM_GETNEXTITEM, usize::MAX, LVNI_SELECTED as isize);
    if sel < 0 {
        return None;
    }

    let mut buf = [0u16; 32];
    let mut item: LVITEMW = std::mem::zeroed();
    item.mask = LVIF_TEXT;
    item.iItem = sel as i32;
    item.iSubItem = 1; // PID column
    item.pszText = buf.as_mut_ptr();
    item.cchTextMax = buf.len() as i32;
    SendMessageW(hwnd_lv, LVM_GETITEMTEXTW, sel as usize, &item as *const _ as isize);

    let pid_str = wstr_to_string(&buf);
    pid_str.trim().parse::<u32>().ok()
}

// ---- GUI: Create ListView ----
unsafe fn create_listview(hwnd_parent: HWND, h_instance: HANDLE) -> HWND {
    let class_name = to_wstr("SysListView32");

    let hwnd_lv = CreateWindowExW(
        0,
        class_name.as_ptr(),
        ptr::null(),
        WS_CHILD | WS_VISIBLE | WS_BORDER | LVS_REPORT | LVS_SINGLESEL | LVS_SHOWSELALWAYS,
        0,
        0,
        800,
        560,
        hwnd_parent,
        IDC_LISTVIEW as HMENU,
        h_instance as HINSTANCE,
        ptr::null(),
    );

    // Enable full-row select and grid lines
    SendMessageW(
        hwnd_lv,
        LVM_SETEXTENDEDLISTVIEWSTYLE,
        0,
        (LVS_EX_FULLROWSELECT | LVS_EX_GRIDLINES | LVS_EX_DOUBLEBUFFER) as isize,
    );

    // Insert columns
    let cols = [("Process Name", 300), ("PID", 100), ("Memory", 200)];
    for (i, (title, width)) in cols.iter().enumerate() {
        let title_w = to_wstr(title);
        let mut col: LVCOLUMNW = std::mem::zeroed();
        col.mask = LVCF_TEXT | LVCF_WIDTH | LVCF_SUBITEM;
        col.cx = *width;
        col.pszText = title_w.as_ptr() as *mut u16;
        col.iSubItem = i as i32;
        SendMessageW(hwnd_lv, LVM_INSERTCOLUMNW, i, &col as *const _ as isize);
    }

    hwnd_lv
}

// ---- GUI: Window Procedure ----
static mut HWND_LISTVIEW: HWND = 0;

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            let h_instance = GetModuleHandleW(ptr::null());
            let icc = INITCOMMONCONTROLSEX {
                dwSize: std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
                dwICC: ICC_LISTVIEW_CLASSES,
            };
            InitCommonControlsEx(&icc);

            HWND_LISTVIEW = create_listview(hwnd, h_instance as HANDLE);
            populate_listview(HWND_LISTVIEW);

            // Set a timer for auto-refresh
            SetTimer(hwnd, TIMER_REFRESH, REFRESH_INTERVAL_MS, None);
            0
        }
        WM_SIZE => {
            let width = (lparam & 0xFFFF) as i32;
            let height = ((lparam >> 16) & 0xFFFF) as i32;
            if HWND_LISTVIEW != 0 {
                SetWindowPos(HWND_LISTVIEW, 0, 0, 0, width, height, SWP_NOZORDER);
            }
            0
        }
        WM_TIMER => {
            if wparam == TIMER_REFRESH {
                populate_listview(HWND_LISTVIEW);
            }
            0
        }
        WM_NOTIFY => {
            let nmhdr = &*(lparam as *const NMHDR);
            if nmhdr.idFrom == IDC_LISTVIEW as usize && nmhdr.code == NM_RCLICK {
                // Show context menu at cursor position
                let mut pt: POINT = std::mem::zeroed();
                GetCursorPos(&mut pt);

                let hmenu = CreatePopupMenu();
                let kill_str = to_wstr("Kill Process Tree");
                let restart_str = to_wstr("Restart Process");
                let refresh_str = to_wstr("Refresh");
                AppendMenuW(hmenu, MF_STRING, IDM_KILL as usize, kill_str.as_ptr());
                AppendMenuW(hmenu, MF_STRING, IDM_RESTART as usize, restart_str.as_ptr());
                AppendMenuW(hmenu, MF_SEPARATOR, 0, ptr::null());
                AppendMenuW(hmenu, MF_STRING, IDM_REFRESH as usize, refresh_str.as_ptr());
                TrackPopupMenu(hmenu, TPM_RIGHTBUTTON, pt.x, pt.y, 0, hwnd, ptr::null());
                DestroyMenu(hmenu);
            }
            0
        }
        WM_COMMAND => {
            let cmd = (wparam & 0xFFFF) as u32;
            match cmd {
                IDM_KILL => {
                    if let Some(pid) = get_selected_pid(HWND_LISTVIEW) {
                        if pid == 0 || pid == 4 {
                            let title = to_wstr("Warning");
                            let msg = to_wstr("Cannot kill critical system process.");
                            MessageBoxW(hwnd, msg.as_ptr(), title.as_ptr(), MB_ICONWARNING | MB_OK);
                        } else {
                            let title = to_wstr("Confirm Kill");
                            let msg_text = to_wstr(&format!(
                                "Kill process tree for PID {}?",
                                pid
                            ));
                            let result = MessageBoxW(
                                hwnd,
                                msg_text.as_ptr(),
                                title.as_ptr(),
                                MB_ICONQUESTION | MB_YESNO,
                            );
                            if result == IDYES {
                                let procs = snapshot_processes();
                                let tree = find_process_tree(pid, &procs);
                                for child_pid in tree {
                                    execute_kill(child_pid);
                                }
                                populate_listview(HWND_LISTVIEW);
                            }
                        }
                    }
                }
                IDM_RESTART => {
                    if let Some(pid) = get_selected_pid(HWND_LISTVIEW) {
                        if pid == 0 || pid == 4 {
                            let title = to_wstr("Warning");
                            let msg = to_wstr("Cannot restart critical system process.");
                            MessageBoxW(hwnd, msg.as_ptr(), title.as_ptr(), MB_ICONWARNING | MB_OK);
                        } else {
                            let title = to_wstr("Confirm Restart");
                            let msg_text = to_wstr(&format!(
                                "Restart process tree for PID {}?",
                                pid
                            ));
                            let result = MessageBoxW(
                                hwnd,
                                msg_text.as_ptr(),
                                title.as_ptr(),
                                MB_ICONQUESTION | MB_YESNO,
                            );
                            if result == IDYES {
                                execute_restart(pid);
                                // Small delay for process to die before refresh
                                std::thread::sleep(std::time::Duration::from_millis(500));
                                populate_listview(HWND_LISTVIEW);
                            }
                        }
                    }
                }
                IDM_REFRESH => {
                    populate_listview(HWND_LISTVIEW);
                }
                _ => {}
            }
            0
        }
        WM_DESTROY => {
            KillTimer(hwnd, TIMER_REFRESH);
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

// ---- Entry Point ----
fn main() {
    unsafe {
        let h_instance = GetModuleHandleW(ptr::null());
        let class_name = to_wstr("NtProcLensClass");
        let window_title = to_wstr("nt-proc-lens  |  The King's Process Explorer");

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: LoadIconW(0, IDI_APPLICATION),
            hCursor: LoadCursorW(0, IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: LoadIconW(0, IDI_APPLICATION),
        };

        RegisterClassExW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            820,
            600,
            0,
            0,
            h_instance,
            ptr::null(),
        );

        if hwnd == 0 {
            return;
        }

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
