use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Settings, glib};
use gtk4 as gtk;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // Get the default settings
        let settings = Settings::default().expect("Failed to get settings");

        // Enable dark mode
        settings.set_gtk_application_prefer_dark_theme(true);

        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.present();
    });

    app.run()
}
