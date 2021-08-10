use gtk::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, ApplicationWindow, Builder, Menu, MenuBar, MenuItem};

fn main() {
    // Create a new application
    let app = Application::new(Some("codes.upvent.mynaptic"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {

    // Indicate where is our main window file
    let glade_src = include_str!("ui/main.glade");

    // Create a new builder using our glade form
    let builder = Builder::from_string(glade_src);

    /* Window objects go here */


    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(app));


    // Draw window with all of it's components
    window.show_all();
}
