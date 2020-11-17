use minifb::{Scale, Window, WindowOptions};

pub fn create() {
    let mut window = Window::new(
        "title",
        1280,
        720,
        WindowOptions {
            borderless: false,
            title: true,
            resize: true,
            scale: Scale::X1,
        },
    )
    .unwrap();

    loop {}
}
