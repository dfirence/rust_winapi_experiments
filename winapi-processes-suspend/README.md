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