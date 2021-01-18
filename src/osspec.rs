/**
    osspec is the file that defines os specific operations.
 */
#[cfg(windows)]
extern crate winapi;

use core::mem;
use std::fs;
use std::path::Path;

/**
   Find a process by its name.

   This function uses the Window API to get the process.

   # Params
   process_name -> The name of the process to find the id for. (Ex: `"chrome.exe"`)

   # Returns
   u32 -> The id of the process. (0 if not found).

   # Examples
   ```rust
   let pid : u32 = osspec::find_process_id("chrome.exe");
   ```
*/
#[cfg(windows)]
pub unsafe fn find_process_id(process_name: &str) -> u32 {
    use winapi::um::winnt;
    use winapi::um::tlhelp32;
    use winapi::um::winuser::WM_NULL;
    use self::winapi::um::handleapi::CloseHandle;
    use self::winapi::um::tlhelp32::{Process32Next, PROCESSENTRY32};

    let mut process_info: tlhelp32::PROCESSENTRY32 = tlhelp32::PROCESSENTRY32::default();
    process_info.dwSize = mem::size_of::<tlhelp32::PROCESSENTRY32>() as u32;

    let processes_snapshot: winnt::HANDLE = tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, WM_NULL);
    if processes_snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return 0;
    }

    let process_info_ptr: *mut PROCESSENTRY32 = &mut process_info;

    tlhelp32::Process32First(processes_snapshot, process_info_ptr);
    let temp_exe_file = String::from_utf8(mem::transmute::<Vec<i8>, Vec<u8>>(remove_zeros(process_info.szExeFile.to_vec()))).unwrap();
    let exe_file = temp_exe_file.split_whitespace().next().unwrap();
    if process_name == exe_file {
        CloseHandle(processes_snapshot);
        return process_info.th32ProcessID;
    }

    while Process32Next(processes_snapshot, process_info_ptr) != 0 {
        let temp_exe_file = String::from_utf8(mem::transmute::<Vec<i8>, Vec<u8>>(remove_zeros(process_info.szExeFile.to_vec()))).unwrap();
        let exe_file = temp_exe_file.split(" ").next().unwrap();
        if process_name == exe_file {
            CloseHandle(processes_snapshot);
            return process_info.th32ProcessID;
        }
    }

    CloseHandle(processes_snapshot);
    return 0;

    /// Cut off unused indices in the szExeFile array.
    fn remove_zeros(vec: Vec<i8>) -> Vec<i8> {
        let mut output: Vec<i8> = Vec::new();
        for mt in vec {
            if mt != 0 {
                output.push(mt);
            } else {
                return output;
            }
        }
        return output;
    }
}
//TODO implement windows support
#[cfg(windows)]
pub unsafe fn is_process_running(process_id: &i32) -> bool {
}

#[cfg(unix)]
pub unsafe fn is_process_running(process_id: &i32) -> bool {
    let mut result: bool = false;
    let file = Path::new("/proc").join(process_id.to_string()).join("cmdline");
    if file.exists() {
        result = true;
    }
    result
}

// TODO implement this.
#[cfg(unix)]
pub unsafe fn find_process_id(process_name: &str) -> u32 {
    let paths = fs::read_dir("/proc/").unwrap();
    let mut result: u32 = 0;
    for path in paths {
        let entry = path.unwrap();
        let file = Path::new(entry.path().as_path()).join("cmdline");
        if file.exists() {
            let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
            if contents.contains(process_name) {
                let string = entry.path().as_path().to_str().unwrap().replace("/proc/", "");
                result = string.parse().unwrap()
            }
        }
    }
    result
}
