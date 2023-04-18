use debug_print::debug_println;

use gtk::{CssProvider, StyleContext, Window, WindowType, gdk::ModifierType, gdk, prelude::*};

fn main() {
    gtk::init();

    let window = gtk::Window::new(WindowType::Toplevel);
    

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let css_provider = CssProvider::new();
    let css = "
    window {
        background-color: #555555;
    }
    ";

    css_provider.load_from_data(css.as_bytes()).expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider"),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );

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
