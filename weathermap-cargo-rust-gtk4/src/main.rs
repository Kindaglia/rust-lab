mod ui;

use gtk::prelude::*;
use gtk::{Application, Settings, glib};
use gtk4 as gtk;
use ui::MainWindow;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        // Enable dark mode using GTK settings
        if let Some(settings) = Settings::default() {
            settings.set_gtk_application_prefer_dark_theme(true);
        }

        // Create and show the main window
        let window = MainWindow::new(app);
        window.present();
    });

    application.run()
}
