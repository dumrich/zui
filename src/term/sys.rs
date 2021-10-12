// System Specific Functions
// Not portable outside of Linux due to use of Linux specific feature

// Imports
use std::env::consts::OS;
use std::os::raw;

#[derive(Debug)]
#[repr(C)]
struct WinSize {
    pub ws_row: raw::c_ushort,
    pub ws_col: raw::c_ushort,
    pub ws_xpixel: raw::c_ushort,
    pub ws_ypixel: raw::c_ushort,
}

extern "C" {
    fn ioctl(fd: raw::c_int, request: raw::c_ulong, ...) -> raw::c_int;
}

type Size = ((u16, u16), (u16, u16));
pub fn term_size() -> Result<Size, ()> {
    let mut size: WinSize;
    unsafe {
        size = core::mem::zeroed();
        // Apple uses 0x40087468
        if OS == "macos" {
            if ioctl(1, 0x40087468, &mut size) == -1 {
                panic!("Could not determine terminal size.");
            }
        } else if OS == "linux" {
            if ioctl(1, 0x5413, &mut size) == -1 {
                panic!("Could not determine terminal size.");
            }
        } else {
            panic!("How many times must I remind you not to use Windows, huh?")
        }
    }

    Ok(((size.ws_row, size.ws_col), (size.ws_xpixel, size.ws_ypixel)))
}
