# CreateProcessW - Suspended
This program shows how to use the `rust` `winapi` to experiment with
the `CreateProcessW` api from Microsoft Windows.

## Program Objective
This program when compiled creates a `detached` windows process in a `suspended` state.

<br/>

### **Functional Test**
In this test, Process ID `2416` was created with the `SUSPENDED` flag. You can see the cli instantiation and process explorer visual representation.

CLI|GUI|
---|---|
![image](https://user-images.githubusercontent.com/11415591/80323399-2aa0a900-87f9-11ea-8e06-b8772138d5a8.png)|![image](https://user-images.githubusercontent.com/11415591/80323495-aac70e80-87f9-11ea-9945-e24c5be220bf.png)|

<br/>

## Lessons Learned
- Ensure you research the Windows Structs and Apis from the main documentation portal for the rust winapi crate. [LINK](https://docs.rs/winapi/0.3.8/winapi/)

- Once you found the structs or functions you want to use, ensure you have updated the Cargo.toml to compile the features you will use.

```toml
[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3.8", features = ["processthreadsapi","winuser", "winbase", "handleapi"] }
```


- The Windows `NULL` type in rust is `std::ptr::null()` or when you require a mutable instance then `std::ptr::null_mut()`

- To call into the **Win32Api** you prefix with an `unsafe {...}` code block.

<br/>

## Static Inspection - Compiled EXE
Intresting to see the amount of imports being added to the binary although I only use the `CreateProcessW` and `GetCurrentProcessId` apis in the code.

<br/>

```json
{
  "pe_name": "winapi-test-pid-suspended.exe",
  "pe_type": 523,
  "pe_size": 164864,
  "pe_subsystem": 3,
  "pe_subsystem_caption": "The Windows character (Cosole UI) subsystem",
  "pe_path": ".\\winapi-processes-suspend\\target\\release\\winapi-test-pid-suspended.exe",
  "pe_timedate_stamp": 1587944353,
  "pe_timedate_human": "2020-04-26T23:39:13.000Z",
  "ImageDLLImports": [
    {
      "name": "kernel32.dll",
      "imports": 44,
      "functions": [
        "AddVectoredExceptionHandler",
        "CloseHandle",
        "CreateMutexA",
        "CreateProcessW",
        "DeleteCriticalSection",
        "EnterCriticalSection",
        "FormatMessageW",
        "GetConsoleMode",
        "GetCurrentDirectoryW",
        "GetCurrentProcess",
        "GetCurrentProcessId",
        "GetCurrentThread",
        "GetCurrentThreadId",
        "GetEnvironmentVariableW",
        "GetLastError",
        "GetModuleFileNameW",
        "GetModuleHandleW",
        "GetProcAddress",
        "GetProcessHeap",
        "GetStdHandle",
        "GetSystemTimeAsFileTime",
        "HeapAlloc",
        "HeapFree",
        "HeapReAlloc",
        "InitializeCriticalSection",
        "InitializeSListHead",
        "IsDebuggerPresent",
        "IsProcessorFeaturePresent",
        "LeaveCriticalSection",
        "LoadLibraryA",
        "QueryPerformanceCounter",
        "ReleaseMutex",
        "RtlCaptureContext",
        "RtlLookupFunctionEntry",
        "RtlVirtualUnwind",
        "SetLastError",
        "SetUnhandledExceptionFilter",
        "TlsAlloc",
        "TlsGetValue",
        "TlsSetValue",
        "UnhandledExceptionFilter",
        "WaitForSingleObjectEx",
        "WriteConsoleW",
        "WriteFile"
      ]
    },
    {
      "name": "vcruntime140.dll",
      "imports": 9,
      "functions": [
        "_CxxThrowException",
        "__C_specific_handler",
        "__CxxFrameHandler3",
        "__current_exception",
        "__current_exception_context",
        "memcmp",
        "memcpy",
        "memmove",
        "memset"
      ]
    },
    {
      "name": "api-ms-win-crt-runtime-l1-1-0.dll",
      "imports": 18,
      "functions": [
        "__p___argc",
        "__p___argv",
        "_c_exit",
        "_cexit",
        "_configure_narrow_argv",
        "_crt_atexit",
        "_exit",
        "_get_initial_narrow_environment",
        "_initialize_narrow_environment",
        "_initialize_onexit_table",
        "_initterm",
        "_initterm_e",
        "_register_onexit_function",
        "_register_thread_local_exe_atexit_callback",
        "_seh_filter_exe",
        "_set_app_type",
        "exit",
        "terminate"
      ]
    },
    {
      "name": "api-ms-win-crt-math-l1-1-0.dll",
      "imports": 1,
      "functions": [
        "__setusermatherr"
      ]
    },
    {
      "name": "api-ms-win-crt-stdio-l1-1-0.dll",
      "imports": 2,
      "functions": [
        "__p__commode",
        "_set_fmode"
      ]
    },
    {
      "name": "api-ms-win-crt-locale-l1-1-0.dll",
      "imports": 1,
      "functions": [
        "_configthreadlocale"
      ]
    },
    {
      "name": "api-ms-win-crt-heap-l1-1-0.dll",
      "imports": 2,
      "functions": [
        "_set_new_mode",
        "free"
      ]
    }
  ],
  "ImageDLLExports": {
    "exports": 0,
    "functions": []
  },
  "ImageHashSignatures": {
    "sha2": "f622fa6ad9fd8b8549df9d868d232f43102d6828a325cbaa562c7db2c04e6ddb"
  }
}
```