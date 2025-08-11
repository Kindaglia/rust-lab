// Import necessary GTK4 components and traits
use gtk4::gdk::Display; // For display handling
use gtk4::prelude::*; // Brings in essential GTK traits
use gtk4::{Application, ApplicationWindow, Box, Button, CssProvider}; // Core GTK widgets

// Define the main window structure
pub struct MainWindow {
    window: ApplicationWindow, // The main application window
}

impl MainWindow {
    // Constructor for creating a new main window
    pub fn new(app: &Application) -> Self {
        // Create the application window with builder pattern
        let window = ApplicationWindow::builder()
            .application(app) // Associate with the application
            .title("First GTK Program") // Set window title
            .default_width(360) // Set default width
            .default_height(640) // Set default height
            .build(); // Build the window

        // Load CSS styling from external file
        let provider = CssProvider::new();
        provider.load_from_path("style.css"); // Load CSS from file

        // Apply CSS styles to the entire application
        if let Some(display) = Display::default() {
            // Get default display
            gtk4::style_context_add_provider_for_display(
                &display,                                  // Target display
                &provider,                                 // CSS provider
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION, // Priority level
            );
        }

        // Create a vertical box container for layout
        let box_container = Box::new(gtk4::Orientation::Vertical, 0); // Vertical orientation, no spacing
        box_container.set_halign(gtk4::Align::Center); // Center horizontally
        box_container.set_valign(gtk4::Align::Center); // Center vertically

        // Create a button with label
        let button = Button::with_label("Welcome");
        button.add_css_class("small-button"); // Apply custom CSS class

        // Set up button click handler
        button.connect_clicked(|_| {
            eprintln!("Clicked!"); // Print to stderr when clicked
        });

        // Add button to the container
        box_container.append(&button);

        // Set the container as the main window's child
        window.set_child(Some(&box_container));

        // Return the initialized MainWindow
        Self { window }
    }

    // Method to show the window
    pub fn present(&self) {
        self.window.present(); // Display the window
    }
}
