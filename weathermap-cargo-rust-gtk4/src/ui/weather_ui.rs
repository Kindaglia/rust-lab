// weather_ui.rs
use gtk4::prelude::*;
use gtk4::{Box, Label};

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
        Self { container: box_container }
    }

    // Method to get the container widget
    pub fn get_widget(&self) -> &Box {
        &self.container
    }
}