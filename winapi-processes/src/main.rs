#[cfg(windows)] extern crate winapi;

// Rust Std Lib
use std::ptr;
use std::mem;
use std::ffi::{OsStr};
use std::os::windows::prelude::*;

// WinApi Imports & Types
use winapi::shared::minwindef::{
    TRUE,
    FALSE, 
    LPVOID,
    DWORD};

use winapi::um::handleapi::{CloseHandle};

use winapi::um::processthreadsapi::{
    PROCESS_INFORMATION,
    STARTUPINFOW,
    GetCurrentProcessId,
    CreateProcessW};

use winapi::um::winbase::{
    CREATE_UNICODE_ENVIRONMENT,
    CREATE_SUSPENDED,
    DETACHED_PROCESS,
    CREATE_NEW_PROCESS_GROUP};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    create_process_suspended()?;
    Ok(())
}

#[cfg(windows)]
fn create_process_suspended() -> std::io::Result<()>
{
    let mut _msg: String = String::from("");
    std::env::current_exe().and_then(|exe_path| {
        let mut _exe = OsStr::new(&exe_path)
                            .encode_wide()
                            .chain(Some(0u16))
                            .collect::<Vec<u16>>();
                            
        let mut _pi = PROCESS_INFORMATION {
            hProcess:    ptr::null_mut(), // Equivalent to Windows NULL type
            hThread:     ptr::null_mut(),
            dwProcessId: 0,
            dwThreadId:  0,
        };
    
        let mut _si: STARTUPINFOW = unsafe { mem::zeroed() };
                _si.cb = mem::size_of::<STARTUPINFOW>() as DWORD;
    
        if unsafe { CreateProcessW(_exe.as_mut_ptr(),
                                   ptr::null_mut(),
                                   ptr::null_mut(),
                                   ptr::null_mut(),
                                   FALSE,
                                   CREATE_UNICODE_ENVIRONMENT | DETACHED_PROCESS | CREATE_SUSPENDED | CREATE_NEW_PROCESS_GROUP,
                                   ptr::null_mut() as LPVOID,
                                   ptr::null(),
                                   &mut _si,
                                   &mut _pi) == TRUE } {
            // Get MetatData for The Newly Spawned Process
            let mut _pid: DWORD = 0;
            unsafe { _pid = GetCurrentProcessId(); };
            unsafe {
                CloseHandle(_pi.hProcess);
                CloseHandle(_pi.hThread);
            }
            _msg = format!(
                "\n{:<32}:\t{}\n{:<32}:\t{}\n{:<32}:\t{}",
                "Process Create",
                "Successful",
                "Process Suspended",
                "True",
                "Calling Process Id",
                _pid
            );
            println!("{}", _msg);
            Ok(())
        } else {
            _msg = format!(
                "\n{:<32}:\t{}\n{:<32}:\t{}",
                "Process Create",
                "Successful",
                "Process Suspended",
                "True"
            );
            println!("{}", _msg);
            Err(std::io::Error::last_os_error())
        }
    })
}