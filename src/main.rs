use gtk::prelude::*;

/*
false warning, impl code generate via macro
*/
#[allow(unused_imports)]
use gtk::Window;
use gtk::WindowType;

fn main() {
    gtk::init();

    let window = gtk::Window::new(WindowType::Toplevel);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

   /*  window.connect_key_press_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    }); */

    window.show_all();

    gtk::main();
}
