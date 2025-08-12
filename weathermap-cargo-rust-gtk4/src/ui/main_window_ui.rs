// main_window_ui.rs
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, CssProvider, Label, Stack, gdk::Display, HeaderBar};
use crate::ui::weather_ui::WeatherView; // Importa WeatherView

// Define the main window structure
pub struct MainWindow {
    window: ApplicationWindow,
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
        
        // Create a HeaderBar
        let header_bar = HeaderBar::new();
        header_bar.set_title_widget(Some(&Label::new(Some("Weather App"))));
        
        // Create a back button for the HeaderBar (initially hidden)
        let back_button = Button::with_label("Back");
        back_button.set_visible(false);
        header_bar.pack_start(&back_button);
        
        // Set the HeaderBar as the title bar
        window.set_titlebar(Some(&header_bar));
        
        // Load CSS styling from external file
        let provider = CssProvider::new();
        provider.load_from_path("src/ui/style.css");
        // Apply CSS styles to the entire application
        if let Some(display) = Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,                                  // Target display
                &provider,                                 // CSS provider
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION, // Priority level
            );
        }
        
        // Create a stack for navigation
        let stack = Stack::new();
        
        // Create the main view
        let main_box = Box::new(gtk4::Orientation::Vertical, 0);
        main_box.set_halign(gtk4::Align::Center);
        main_box.set_valign(gtk4::Align::Center);
        let welcome_label = Label::builder()
            .label("Welcome to Weather App") // Set the text content
            .css_classes(["welcome-title"]) // Apply CSS class for styling
            .build();
        // Add the welcome label to our main container
        main_box.append(&welcome_label);
        
        // Create a button with text
        let button = Button::with_label("Check Weather");
        // Apply CSS styling to the button
        button.add_css_class("small-button");
        // Add some space above the button
        button.set_margin_top(12);
        main_box.append(&button);
        
        // Add main view to stack
        stack.add_named(&main_box, Some("main"));
        
        // Create weather view using WeatherView
        let weather_view = WeatherView::new();
        
        // Add weather view to stack
        stack.add_named(weather_view.get_widget(), Some("weather"));
        
        // Set up button click handlers
        let stack_clone = stack.clone();
        button.connect_clicked(move |_| {
            // Switch to weather view
            stack_clone.set_visible_child_name("weather");
        });
        
        // Connect back button handler in HeaderBar
        let stack_clone = stack.clone();
        back_button.connect_clicked(move |_| {
            // Switch back to main view
            stack_clone.set_visible_child_name("main");
        });
        
        // Update HeaderBar when stack changes
        let back_button_clone = back_button.clone();
        stack.connect_notify_local(Some("visible-child-name"), move |stack, _pspec| {
            if let Some(name) = stack.visible_child_name() {
                if name == "weather" {
                    back_button_clone.set_visible(true);
                } else {
                    back_button_clone.set_visible(false);
                }
            }
        });
        
        // Set the stack as the main window's child
        window.set_child(Some(&stack));
        
        Self { window }
    }
    
    // Method to show the window
    pub fn present(&self) {
        self.window.present();
    }
}