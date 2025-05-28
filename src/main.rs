#[allow(unused_imports)]
use debug_print::debug_println;
use std::fs;

use gtk::{
    CssProvider, EventControllerKey,
    Orientation::Vertical,
    gdk::{self, ModifierType},
    prelude::*,
};

fn usage(exe: String) {
    println!("{:?} --help", exe);
    println!("{:?} --app-id <app id>", exe);
}

fn main() -> glib::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mut args_gtk: Vec<String> = Vec::new();
    let exe = args[0].clone();
    let mut app_id = "com.github.RobertMueller2.blankwindow-gtk-rs";

    let mut i = 1;
    while i < args.len() {
        let a = &args[i];
        i += 1;
        match a.as_str() {
            "--app-id" => {
                app_id = args[i].as_str();
                i += 1;
            }
            "--help" => {
                usage(exe);
                std::process::exit(0);
            }
            _ => {
                args_gtk.push(a.clone());
            }
        }
    }

    let app = gtk::Application::builder().application_id(app_id).build();
    app.connect_activate(build_window);
    app.connect_startup(add_css);
    app.run_with_args(&args_gtk)
}

fn add_css(_: &gtk::Application) {
    let css_provider = CssProvider::new();
    css_provider.load_from_data(include_str!("default-style.css"));

    let user_css_path = dirs::config_dir().unwrap().join("blankwindow.css");
    if let Ok(user_css) = fs::read_to_string(user_css_path) {
        css_provider.load_from_data(user_css.as_str());
    };

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css_provider,
        800,
    );
}

fn build_window(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder().application(app).build();
    let vbox = gtk::Box::new(Vertical, 0);

    let key_ctrl = EventControllerKey::builder()
        .propagation_phase(gtk::PropagationPhase::Capture)
        .build();
    let app_clone = app.clone();
    key_ctrl.connect_key_pressed(move |_, keyval, _, modifiers| {
        if modifiers == ModifierType::CONTROL_MASK && keyval == gdk::Key::Q {
            app_clone.quit();
        }
        glib::Propagation::Stop
    });

    window.add_controller(key_ctrl);
    window.set_child(Some(&vbox));
    window.present();
}
