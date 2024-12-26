use game_engine::*;

fn main() {
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
