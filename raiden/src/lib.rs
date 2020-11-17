mod window;

pub struct Layout {}

pub trait App {
    type Msg;

    fn update(&self, msg: &Self::Msg);

    fn view(&self) -> Layout;

    fn run(&self) {
        // self.update(&msg);
        // let layout = self.view(&model);
        let window = window::create();
    }
}
