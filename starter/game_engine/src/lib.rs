#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::c_int;
    use std::{collections::HashMap, ffi::CString};

    use super::*;

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        unsafe {
            let title = CString::new("Rust Test Game").expect("CString::new failed");
            create_game_window(title.as_ptr(), 800, 600);

            while window_should_close() == 0 {
                update_game_window();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        unsafe {
            let title = CString::new("Rust Test Game").expect("CString::new failed");
            create_game_window(title.as_ptr(), 800, 600);

            let sprite = create_sprite(0.0, 0.0, 100, 100, 255, 0, 0);

            while window_should_close() == 0 {
                clear_screen();
                render_sprite(sprite);
                update_game_window();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        unsafe {
            let title = CString::new("Rust Test Game").expect("CString::new failed");
            create_game_window(title.as_ptr(), 800, 600);

            let red_sprite = create_sprite(0.0, 0.0, 100, 100, 255, 0, 0);
            let green_sprite = create_sprite(200.0, 200.0, 100, 100, 0, 255, 0);

            render_sprite(red_sprite);
            update_game_window();
            std::thread::sleep(std::time::Duration::from_secs(5));
            while window_should_close() == 0 {
                update_game_window();
                clear_screen();
                render_sprite(green_sprite);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        unsafe {
            let title = CString::new("Press Right, Left, Up, Down and Space keys to exit")
                .expect("CString::new failed");
            create_game_window(title.as_ptr(), 800, 600);

            let mut keys: HashMap<u32, bool> = ([
                (GLFW_KEY_RIGHT, false),
                (GLFW_KEY_LEFT, false),
                (GLFW_KEY_UP, false),
                (GLFW_KEY_DOWN, false),
                (GLFW_KEY_SPACE, false),
            ])
            .into();

            while !keys.iter().all(|(_, v)| *v) {
                if get_key(get_window(), GLFW_KEY_RIGHT as c_int) == GLFW_PRESS as c_int {
                    keys.insert(GLFW_KEY_RIGHT, true);
                }

                if get_key(get_window(), GLFW_KEY_LEFT as c_int) == GLFW_PRESS as c_int {
                    keys.insert(GLFW_KEY_LEFT, true);
                }

                if get_key(get_window(), GLFW_KEY_UP as c_int) == GLFW_PRESS as c_int {
                    keys.insert(GLFW_KEY_UP, true);
                }

                if get_key(get_window(), GLFW_KEY_DOWN as c_int) == GLFW_PRESS as c_int {
                    keys.insert(GLFW_KEY_DOWN, true);
                }

                if get_key(get_window(), GLFW_KEY_SPACE as c_int) == GLFW_PRESS as c_int {
                    keys.insert(GLFW_KEY_SPACE, true);
                }

                clear_screen();

                update_game_window();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        unsafe {
            let title = CString::new("Rust Test Game").expect("CString::new failed");
            create_game_window(title.as_ptr(), 800, 600);

            let sprite = create_sprite(0.0, 0.0, 100, 100, 255, 0, 0);

            let mut x = 0.0;
            let mut y = 0.0;

            while window_should_close() == 0 {
                clear_screen();
                update_sprite_position(sprite, x, y);
                x += 2.0;
                y += 2.0;
                render_sprite(sprite);
                update_game_window();
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }
}
