use core::mem;

/**
    osspec is the file that defines os specific operations.
 */
#[cfg(windows)]
extern crate winapi;

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

    let process_info_ptr : *mut PROCESSENTRY32= &mut process_info;

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
    fn remove_zeros(vec : Vec<i8>) -> Vec<i8> {
        let mut output : Vec<i8> = Vec::new();
        for mt in vec {
            if mt != 0 {
                output.push(mt);
            }
            else{
                return output;
            }
        }
        return output;
    }
}

// TODO implement this.
#[cfg(linux)]
pub unsafe fn findProcessId(process_name: &str) -> u32 {
    return 0;
}
