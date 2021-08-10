use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::new(app);

    // Create a button
    let button = Button::with_label("Press me!");

    // Set the button margins
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // Connect callback
    button.connect_clicked(move |button| {
        button.set_label("Hello World");
    });

    // Set the window title
    window.set_title("Mynaptic Package Manager");

    // Add button
    window.add(&button);
    window.show_all();
}
