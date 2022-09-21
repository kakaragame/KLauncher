/**
   osspec is the file that defines os specific operations.
*/
#[cfg(windows)]
extern crate winapi;

use core::mem;



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
    use self::winapi::um::handleapi::CloseHandle;
    use self::winapi::um::tlhelp32::{Process32Next, PROCESSENTRY32};
    use winapi::um::tlhelp32;
    use winapi::um::winnt;
    use winapi::um::winuser::WM_NULL;

    let mut process_info: tlhelp32::PROCESSENTRY32 = tlhelp32::PROCESSENTRY32 {
        dwSize: mem::size_of::<tlhelp32::PROCESSENTRY32>() as u32,
        ..tlhelp32::PROCESSENTRY32::default()
    };

    let processes_snapshot: winnt::HANDLE =
        tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, WM_NULL);
    if processes_snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return 0;
    }

    let process_info_ptr: *mut PROCESSENTRY32 = &mut process_info;

    tlhelp32::Process32First(processes_snapshot, process_info_ptr);
    let temp_exe_file = String::from_utf8(mem::transmute::<Vec<i8>, Vec<u8>>(remove_zeros(
        process_info.szExeFile.to_vec(),
    )))
    .unwrap();
    let exe_file = temp_exe_file.split_whitespace().next().unwrap();
    if process_name == exe_file {
        CloseHandle(processes_snapshot);
        return process_info.th32ProcessID;
    }

    while Process32Next(processes_snapshot, process_info_ptr) != 0 {
        let temp_exe_file = String::from_utf8(mem::transmute::<Vec<i8>, Vec<u8>>(remove_zeros(
            process_info.szExeFile.to_vec(),
        )))
        .unwrap();
        let exe_file = temp_exe_file.split(' ').next().unwrap();
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
        output
    }
}

/**
   Check if a process is running by its id.

   This function uses the Window API to get the process.

   # Params
   process_id -> The process id to find. (Note: process ids can be recycled.)

   # Returns
   bool -> If the process is running.

   # Examples
   ```rust
   let is_running : bool = osspec::is_process_running(555555);
   ```
*/
#[cfg(windows)]
pub unsafe fn is_process_running(process_id: &u32) -> bool {
    use self::winapi::shared::minwindef::FALSE;
    use self::winapi::shared::winerror::WAIT_TIMEOUT;
    use self::winapi::um::handleapi::CloseHandle;
    use self::winapi::um::winnt::SYNCHRONIZE;
    use winapi::um;
    use winapi::um::winnt;

    let process: winnt::HANDLE =
        um::processthreadsapi::OpenProcess(SYNCHRONIZE, FALSE, *process_id);
    let ret = um::synchapi::WaitForSingleObject(process, 0);
    CloseHandle(process);

    ret == WAIT_TIMEOUT
}

#[cfg(unix)]
pub unsafe fn is_process_running(process_id: &u32) -> bool {
    let mut result: bool = false;
    let file = Path::new("/proc")
        .join(process_id.to_string())
        .join("cmdline");
    if file.exists() {
        result = true;
    }
    result
}

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
                let string = entry
                    .path()
                    .as_path()
                    .to_str()
                    .unwrap()
                    .replace("/proc/", "");
                result = string.parse().unwrap()
            }
        }
    }
    result
}
