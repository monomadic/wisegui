use raiden::{App, Layout};

struct Model {
    counter: u32,
}

#[derive(Debug)]
enum Msg {
    None,
}

impl App for Model {
    type Msg = Msg;

    fn view(&self) -> Layout {
        println!("view");
        Layout {}
    }

    fn update(&self, msg: &Self::Msg) {
        match msg {
            Msg::None => (),
        }
    }
}

fn main() {
    let app = Model { counter: 0 };
    app.run();
}
