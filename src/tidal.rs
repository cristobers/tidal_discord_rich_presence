use windows_sys::{
    core::*, Win32::{Foundation::HWND, UI::WindowsAndMessaging::*}
};

#[derive(Debug)]
pub struct Title {
    pub song: String,
    pub artist: String,
}

pub fn get_tidal_hwnd() -> Result<HWND, String> {
    // Tries to get the `HWND` for TIDAL.
    unsafe {
        let res: HWND = FindWindowA(std::ptr::null(), s!("TIDAL"));
        match res {
            0 => Err("couldn't find tidal".to_string()),
            _ => Ok(res)
        }
    }
}

pub fn check_title(w: HWND) -> Result<String, String> {
    // Tries to find the window title "TIDAL" for a given HWND.
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let res = GetWindowTextW(w, text.as_mut_ptr(), text.len() as i32);
        match res {
            0 => Err("TIDALERROR: failed to get text.".to_string()),
            _ => Ok(String::from_utf16(&text).unwrap())
        }
    }
}
