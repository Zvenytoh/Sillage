#[cfg(windows)]
pub fn open_audio() -> Result<Option<std::path::PathBuf>, &'static str> {
    use std::os::windows::ffi::OsStringExt;
    use windows::Win32::UI::Controls::Dialogs::{
        CommDlgExtendedError, GetOpenFileNameW, OFN_EXPLORER, OFN_FILEMUSTEXIST, OFN_NOCHANGEDIR,
        OFN_PATHMUSTEXIST, OPENFILENAMEW,
    };
    use windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow;
    use windows::core::{PCWSTR, PWSTR};
    let mut filename = vec![0_u16; 32_768];
    let filter: Vec<u16> = "Audio files\0*.mp3;*.m4a;*.wav;*.flac;*.ogg\0All files\0*.*\0\0"
        .encode_utf16()
        .collect();
    // The buffers outlive this synchronous dialog; the owner belongs to this UI thread.
    unsafe {
        let mut options = OPENFILENAMEW {
            lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
            hwndOwner: GetActiveWindow(),
            lpstrFilter: PCWSTR(filter.as_ptr()),
            lpstrFile: PWSTR(filename.as_mut_ptr()),
            nMaxFile: filename.len() as u32,
            Flags: OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_NOCHANGEDIR,
            ..Default::default()
        };
        if GetOpenFileNameW(&mut options).as_bool() {
            let length = filename
                .iter()
                .position(|value| *value == 0)
                .ok_or("The selected path is invalid.")?;
            return Ok(Some(
                std::ffi::OsString::from_wide(&filename[..length]).into(),
            ));
        }
        if CommDlgExtendedError().0 != 0 {
            return Err("Windows could not open the file picker. Try again.");
        }
    }
    Ok(None)
}

#[cfg(not(windows))]
pub fn open_audio() -> Result<Option<std::path::PathBuf>, &'static str> {
    Err("This build currently supports Windows only.")
}

pub fn show_error(message: &str) {
    #[cfg(windows)]
    {
        use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};
        use windows::core::PCWSTR;
        let text: Vec<u16> = message.encode_utf16().chain([0]).collect();
        let title: Vec<u16> = "Sillage".encode_utf16().chain([0]).collect();
        // Both terminated buffers remain valid until the synchronous call returns.
        unsafe {
            MessageBoxW(
                None,
                PCWSTR(text.as_ptr()),
                PCWSTR(title.as_ptr()),
                MB_OK | MB_ICONERROR,
            );
        }
    }
    #[cfg(not(windows))]
    eprintln!("Sillage: {message}");
}
