use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{LPARAM, WPARAM},
        UI::WindowsAndMessaging::{FindWindowW, SendMessageW, WM_CLOSE},
    },
};

fn main() {
    unsafe {
        // open '记事本'
        let handle = FindWindowW(
            PCWSTR::null(),
            PCWSTR::from_raw(
                "无标题 - Notepad"
                    .encode_utf16()
                    .collect::<Vec<u16>>()
                    .as_ptr(),
            ),
        );

        SendMessageW(handle, WM_CLOSE, WPARAM(0), LPARAM(0));
    }
}
