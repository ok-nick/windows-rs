[![crates.io](https://img.shields.io/crates/v/windows.svg)](https://crates.io/crates/windows)
[![docs.rs](https://docs.rs/windows/badge.svg)](https://microsoft.github.io/windows-docs-rs/)
[![build](https://github.com/microsoft/windows-rs/workflows/build/badge.svg?event=push)](https://github.com/microsoft/windows-rs/actions)

## Rust for Windows

The `windows` crate lets you call any Windows API past, present, and future using code generated on the fly directly from the metadata describing the API and right into your Rust package where you can call them as if they were just another Rust module. The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs.

* [Documentation](https://microsoft.github.io/windows-docs-rs/)
* [Samples](https://github.com/microsoft/windows-samples-rs/)
* [Changelog](https://github.com/microsoft/windows-rs/blob/master/.github/changelog.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows]
version = "0.28.0"
features = [
    "alloc",
    "Data_Xml_Dom",
    "Win32_Foundation",
    "Win32_Security",
    "Win32_System_Threading",
    "Win32_UI_WindowsAndMessaging",
]
```

Make use of any Windows APIs as needed.

```rust
use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml("<html>hello world</html>")?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, "Text", "Caption", MB_OK);
    }

    Ok(())
}
```

## windows-sys

The `windows-sys` crate is a zero-overhead fallback for the most demanding situations and primarily where the absolute best compile time is essential. It only includes function declarations (externs), structs, and constants. No convenience helpers, traits, or wrappers are provided.

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-sys]
version = "0.28.0"
features = [
    "Win32_Foundation",
    "Win32_Security",
    "Win32_System_Threading",
    "Win32_UI_WindowsAndMessaging",
]
```

Make use of any Windows APIs as needed.

```rust
use windows_sys::{Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null_mut());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(0, b"Text\0".as_ptr() as _, b"Caption\0".as_ptr() as _, MB_OK);
    }
}
```
