// weather_ui.rs
use gtk4::prelude::*;
use gtk4::{Box, Button, Label};

pub struct WeatherView {
    container: Box,
}

impl WeatherView {
    pub fn new() -> Self {
        // Create main container
        let box_container = Box::new(gtk4::Orientation::Vertical, 0);
        box_container.set_halign(gtk4::Align::Center);
        box_container.set_valign(gtk4::Align::Center);

        // Create weather title
        let weather_title = Label::builder()
            .label("Ecco il meteo")
            .css_classes(["weather-title"])
            .build();
        
        box_container.append(&weather_title);

        // Add some weather info (placeholder)
        let weather_info = Label::builder()
            .label("Qui verranno mostrate le informazioni meteo")
            .css_classes(["weather-info"])
            .build();
        
        box_container.append(&weather_info);

        // Create back button
        let back_button = Button::with_label("Torna indietro");
        back_button.add_css_class("small-button");
        back_button.set_margin_top(12);
        
        box_container.append(&back_button);

        Self { container: box_container }
    }

    // Method to get the container widget
    pub fn get_widget(&self) -> &Box {
        &self.container
    }

    // Method to connect the back button handler
    pub fn connect_back_button<F: Fn() + 'static>(&self, callback: F) {
        // Find the back button in the container
        if let Some(button) = self.container.first_child() // weather_title
            .and_then(|w| w.next_sibling()) // weather_info
            .and_then(|w| w.next_sibling()) // back_button
            .and_then(|w| w.downcast::<Button>().ok()) {
            button.connect_clicked(move |_| {
                callback();
            });
        }
    }
}