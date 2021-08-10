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

    // Main application window
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(app));

    // Menu bar from glade UI
    let menubar: MenuBar = builder.object("menu_bar").expect("Couldn't get menu bar");

    /* Menu bar and it's items */

    // Main Menu
    let menu = Menu::new();

    /* Menu options */
    let file = MenuItem::with_label("File");
    let about = MenuItem::with_label("About");


    // Draw window with all of it's components
    window.show_all();
}
