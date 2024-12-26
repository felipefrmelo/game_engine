#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {{
        let sprite = create_sprite($x, $y, $width, $height, $r, $g, $b);
        render_sprite(sprite);
        sprite
    }};
}

#[macro_export]
macro_rules! move_sprite {
    ($sprite:expr, $x:expr, $y:expr) => {{
        clear_screen();
        update_sprite_position($sprite, $x, $y);
        render_sprite($sprite);
    }};
}

#[macro_export]
macro_rules! tick {
    ($duration:expr) => {{
        update_game_window();
        std::thread::sleep(std::time::Duration::from_millis($duration));
    }};
    () => {
        tick!(10);
    };
}

#[macro_export]
macro_rules! on_key_press {
    ($key:expr, $action:block) => {{
        if get_key(get_window(), $key as std::ffi::c_int) == GLFW_PRESS as std::ffi::c_int {
            $action
        }
    }};
}

#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title:expr, $width:expr, $height:expr, $start:block, $loop:block, $exit:block) => {{
        unsafe {
            let title = std::ffi::CString::new($title).expect("CString::new failed");
            create_game_window(title.as_ptr(), $width, $height);

            $start

            while window_should_close() == 0 {
                $loop
                tick!();
            }

            $exit
        }
    }};
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        start_window_and_game_loop!(
            "Rust Test Game",
            800,
            600,
            {
                println!("Game starting...");
            },
            {
                // Game loop
            },
            {
                println!("Game exiting...");
            }
        );
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        start_window_and_game_loop!(
            "Rust Test Game",
            800,
            600,
            {
                println!("Initializing sprite rendering...");
            },
            {
                spawn_sprite!(0.0, 0.0, 100, 100, 255, 0, 0);
            },
            {
                println!("Exiting sprite rendering test...");
            }
        );
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        start_window_and_game_loop!(
            "Rust Test Game",
            800,
            600,
            {
                spawn_sprite!(200.0, 200.0, 100, 100, 255, 0, 0);
                tick!(2000);
            },
            {
                clear_screen();
                spawn_sprite!(200.0, 200.0, 100, 100, 0, 255, 0);
            },
            {
                println!("Finished screen clearing test...");
            }
        );
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        let mut keys: std::collections::HashMap<u32, bool> = [
            (GLFW_KEY_RIGHT, false),
            (GLFW_KEY_LEFT, false),
            (GLFW_KEY_UP, false),
            (GLFW_KEY_DOWN, false),
            (GLFW_KEY_SPACE, false),
        ]
        .into_iter()
        .collect();

        start_window_and_game_loop!(
            "Press Right, Left, Up, Down and Space keys to exit",
            800,
            600,
            {
                println!("Test starting: Press keys to continue...");
            },
            {
                for (key, pressed) in keys.iter_mut() {
                    on_key_press!(*key, {
                        *pressed = true;
                    });
                }

                if keys.values().all(|&v| v) {
                    println!("All keys pressed! Exiting test.");
                    break;
                }
            },
            {}
        );
    }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        let mut x = 0.0;
        let mut y = 0.0;
        start_window_and_game_loop!(
            "Rust Test Game",
            800,
            600,
            {
                println!("Starting sprite position update test...");
            },
            {
                let sprite = spawn_sprite!(x, y, 100, 100, 255, 0, 0);
                move_sprite!(sprite, x, y);
                x += 2.0;
                y += 2.0;
            },
            {
                println!("Finished sprite position update test...");
            }
        );
    }
}
