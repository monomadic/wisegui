use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

fn draw_point(buffer: Vec<u32>, width: u32, height: u32, x: u32, y: u32) -> Vec<u32> {
    buffer
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; 640 * 480];
    let mut window = Window::new(
        "title",
        700,
        100,
        WindowOptions {
            borderless: true,
            title: false,
            resize: false,
            scale: Scale::X1,
            scale_mode: ScaleMode::Center,
            topmost: true,
            transparency: true,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // window.update_with_buffer(&buffer, 640, 480).unwrap();

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // for i in buffer.iter_mut() {
        //     *i = 0; // write something more funny here!
        // }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, 640, 480).unwrap();
    }
}
