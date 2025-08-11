// Import necessary GTK4 components and traits
use gtk4::gdk::Display; // For display handling
use gtk4::{Application, ApplicationWindow, Box, Button, CssProvider};
use gtk4::{Label, prelude::*}; // Brings in essential GTK traits // Core GTK widgets

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
        provider.load_from_path("src/ui/style.css");

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

        // Create a welcome label with specific styling
        let welcome_label = Label::builder()
            .label("Welcome to Weather App") // Set the text content
            .css_classes(["welcome-title"]) // Apply CSS class for styling
            .build();
        // Add the welcome label to our main container
        box_container.append(&welcome_label);

        // Create a button with text
        let button = Button::with_label("Check Weather");
        // Apply CSS styling to the button
        button.add_css_class("small-button");
        // Add some space above the button
        button.set_margin_top(12);

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
