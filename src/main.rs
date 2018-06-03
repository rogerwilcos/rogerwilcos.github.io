extern crate rogercv;
extern crate yew;

use rogercv::Model;
use yew::prelude::*;

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
