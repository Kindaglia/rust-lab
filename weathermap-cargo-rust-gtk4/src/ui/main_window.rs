use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        window.set_child(Some(&button));

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
