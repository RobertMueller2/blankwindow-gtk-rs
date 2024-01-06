use std::fs;

use debug_print::debug_println;

use gtk::{gdk::{self, ModifierType}, CssProvider, Orientation::Vertical, Window, WindowType, prelude::*};

fn main() {

    gtk::init().expect("Couldn't initialize GTK");

    let window = gtk::Window::new(WindowType::Toplevel);
    let vbox = gtk::Box::new(Vertical, 0);
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        glib::Propagation::Proceed
    });

    let user_css_path = dirs::config_dir().unwrap().join("blankwindow.css");

    let css_content = fs::read_to_string(user_css_path).unwrap_or("".to_owned());
    let css_bytes = match css_content.as_str() {
        "" => include_bytes!("default-style.css"),
        x => x.as_bytes()
    };
    
    let css_provider = CssProvider::new();
    css_provider.load_from_data(css_bytes).expect("Failed to load CSS");
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
        glib::Propagation::Proceed
    });

    window.add(&vbox);

    window.show_all();

    gtk::main();
}
