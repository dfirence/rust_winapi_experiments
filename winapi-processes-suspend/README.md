# CreateProcessW - Suspended
This program shows how to use the `rust` `winapi` to experiment with
the `CreateProcessW` api from Microsoft Windows.

## Program Objective
This program when compiled creates a `detached` windows process in a `suspended` state.

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