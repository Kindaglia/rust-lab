use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, CssProvider, Box}; // Aggiungi Box alle importazioni

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(360)
            .default_height(640)
            .build();
        
        // Carica il CSS
        let provider = CssProvider::new();
        provider.load_from_path("style.css");
        
        // Applica il CSS al display corrente
        if let Some(display) = Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        
        // Crea un Box per centrare il pulsante
        let box_container = Box::new(gtk4::Orientation::Vertical, 0);
        box_container.set_halign(gtk4::Align::Center);
        box_container.set_valign(gtk4::Align::Center);
        
        let button = Button::with_label("Click me!");
        button.add_css_class("small-button");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        
        // Aggiungi il pulsante al Box
        box_container.append(&button);
        
        // Imposta il Box come figlio della finestra
        window.set_child(Some(&box_container));
        
        Self { window }
    }
    
    pub fn present(&self) {
        self.window.present();
    }
}