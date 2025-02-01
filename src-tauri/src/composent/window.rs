use anyhow::Result;
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use std::{thread, time::Duration};
use winapi::{
    shared::windef::{RECT,HWND},
    um::winuser::{
        EnumWindows, GetWindowRect, GetWindowTextA, IsIconic, IsWindowVisible,
        SetForegroundWindow, ShowWindow, SW_RESTORE,
    },
};

struct CallbackData {
    window_info: Option<(HWND, String)>,
    title_ptr: *const i8,
}

pub struct WindowManager {
    hwnd: Option<HWND>,
    window_rect: Option<RECT>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            hwnd: None,
            window_rect: None,
        }
    }

    pub fn find_window(&mut self, window_title: &str) -> Result<bool> {
        unsafe {
            let title_cstr = std::ffi::CString::new(window_title)?;
            let mut data = CallbackData {
                window_info: None,
                title_ptr: title_cstr.as_ptr(),
            };

            extern "system" fn callback(hwnd: HWND, lparam: isize) -> i32 {
                unsafe {
                    let data = &mut *(lparam as *mut CallbackData);
                    let window_title = std::ffi::CStr::from_ptr(data.title_ptr)
                        .to_string_lossy()
                        .into_owned();

                    if IsWindowVisible(hwnd) != 0 {
                        let mut buffer = [0u8; 256];
                        let len = GetWindowTextA(hwnd, buffer.as_mut_ptr() as *mut i8, 256);
                        let title = String::from_utf8_lossy(&buffer[..len as usize]).to_string();

                        if title.contains(&window_title) {
                            data.window_info = Some((hwnd, title));
                            return 0;
                        }
                    }
                    1
                }
            }

            EnumWindows(Some(callback), &mut data as *mut _ as isize);

            if let Some((hwnd, _)) = data.window_info {
                self.hwnd = Some(hwnd);
                let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                GetWindowRect(hwnd, &mut rect);
                self.window_rect = Some(rect);
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn bring_to_front(&self) -> Result<()> {
        unsafe {
            if let Some(hwnd) = self.hwnd {
                if IsIconic(hwnd) != 0 {
                    ShowWindow(hwnd, SW_RESTORE);
                    // Délai après restauration de la fenêtre
                    thread::sleep(Duration::from_millis(200));
                }
                SetForegroundWindow(hwnd);
                // Court délai après mise au premier plan
                thread::sleep(Duration::from_millis(100));
            }
            Ok(())
        }
    }

    pub fn send_travel_command(&self, x: i8, y: i8) -> Result<()> {
        let settings = Settings::default();
        let mut enigo = Enigo::new(&settings)?;
        let command = format!("/travel {},{}", x, y);

        // Simulation réaliste avec délais
        let _ = enigo.key(Key::Return, Click);
        thread::sleep(Duration::from_millis(50));

        let _ = enigo.text(&command);
        thread::sleep(Duration::from_millis(100));

        let _ = enigo.key(Key::Return, Click);
        thread::sleep(Duration::from_millis(50));

        let _ = enigo.key(Key::Return, Click);
        thread::sleep(Duration::from_millis(50));

        Ok(())
    }
}