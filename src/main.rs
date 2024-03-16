mod task_object;
mod task_row;
mod window;



use gtk::{prelude::*, CssProvider};
use gtk::{gio, glib, Application};
use window::Window;
use gtk::gdk;

// ANCHOR: main
fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("todo_app.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk_rs.todo_app")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    let file = gio::File::for_path("styles/style.css");
    provider.load_from_file(&file);

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create a new custom window and present it
    let window = Window::new(app);
    window.present();
}
// ANCHOR_END: main