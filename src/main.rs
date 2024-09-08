use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Crab");
    window.set_default_size(300, 100);

    // Create a vertical box to hold the button and label
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.add(&vbox);

    // Create a label
    let label = Label::new(None);
    vbox.pack_start(&label, true, true, 0);

    // Create a button
    let button = Button::with_label("Let's crab!");
    vbox.pack_start(&button, true, true, 0);

    // Connect the button's clicked signal to a closure
    button.connect_clicked(move |_| {
        label.set_text("CRAB. A simple application written in rust. 🦀");
    });

    // Connect the window's delete event to exit the application
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all widgets
    window.show_all();

    // Start the GTK main loop
    gtk::main();
}
