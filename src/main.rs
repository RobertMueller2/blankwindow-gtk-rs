use debug_print::debug_println;

use gtk::{gdk::ModifierType, prelude::*};

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

    window.connect_key_press_event(move |_: &Window, eventkey| {
        debug_println!("{:?} {:?}", eventkey.state(), eventkey.keycode().unwrap());


        if eventkey.state().contains(ModifierType::CONTROL_MASK) {
            // this is possibly extended in the future ;)
            #[allow(clippy::single_match)]
            match eventkey.keycode().unwrap() {
                24 => {
                    gtk::main_quit();
                }
                _ => {}
            }
        }
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
