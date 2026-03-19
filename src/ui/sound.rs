/// System notification sound utilities.
/// This module encapsulates platform-specific FFI calls for playing notification sounds.

/// Play a system notification sound (F14)
pub fn play_notification_sound() {
    #[cfg(target_os = "windows")]
    {
        #[link(name = "user32")]
        unsafe extern "system" {
            fn MessageBeep(uType: u32) -> i32;
        }
        // SAFETY: MessageBeep is a standard Windows API call.
        // 0x40 is MB_ICONASTERISK / MB_ICONINFORMATION.
        unsafe {
            MessageBeep(0x40);
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        print!("\x07");
    }
}
