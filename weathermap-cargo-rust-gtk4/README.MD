

# GTK4-Rust Navigation with Stack: Detailed Guide

This README explains how to implement view navigation using `gtk4::Stack` in a Rust application with GTK4, with particular focus on integrating the weather view.

## Table of Contents
1. [Introduction](#introduction)
2. [Project Structure](#project-structure)
3. [What is GTK4 Stack](#what-is-gtk4-stack)
4. [Step-by-Step Implementation](#step-by-step-implementation)
5. [Key Code Explanation](#key-code-explanation)
6. [Navigation Patterns](#navigation-patterns)
7. [Best Practices](#best-practices)
8. [Future Extensions](#future-extensions)

## Introduction

In this guide, we implement a view navigation system using `gtk4::Stack` for a weather application. The goal is to create smooth navigation between the main screen and weather view without creating new windows.

### Objectives
- ✅ Navigate between views without creating new windows
- ✅ Modularized code without duplication
- ✅ Centralized navigation management
- ✅ Easy to extend for adding new views

## Project Structure

```
src/
├── main.rs
└── ui/
    ├── mod.rs
    ├── main_window_ui.rs
    ├── weather_ui.rs
    └── style.css
```

### File Descriptions

- **`main.rs`**: Application entry point
- **`ui/mod.rs`**: Module that exposes UI components
- **`ui/main_window_ui.rs`**: Main window with navigation management
- **`ui/weather_ui.rs`**: Autonomous weather view
- **`ui/style.css`**: Shared CSS styles

## What is GTK4 Stack

`gtk4::Stack` is a container widget that **displays only one child at a time**, functioning like a deck of cards where you can only see one card at a time.

### Key Features
- **Single Display**: Shows only one widget child at a time
- **Animated Transitions**: Supports transitions between views
- **Identifier Names**: Each view has a unique name
- **State Management**: Maintains state of all views

### Advantages over Other Solutions
- 🚀 **Performance**: Doesn't create/destroy windows
- 🧠 **Memory Efficient**: All views remain in memory
- 🎨 **Smooth Transitions**: Built-in animations
- 📱 **Responsive**: Adaptable to different sizes

## Step-by-Step Implementation

### Step 1: Create the Autonomous Weather View

**File: `weather_ui.rs`**

```rust
use gtk4::prelude::*;
use gtk4::{Box, Button, CssProvider, Label, gdk::Display};

pub struct WeatherView {
    container: Box,
}

impl WeatherView {
    pub fn new() -> Self {
        // Create the main container
        let box_container = Box::new(gtk4::Orientation::Vertical, 0);
        box_container.set_halign(gtk4::Align::Center);
        box_container.set_valign(gtk4::Align::Center);

        // Add title
        let weather_title = Label::builder()
            .label("Ecco il meteo")
            .css_classes(["weather-title"])
            .build();
        box_container.append(&weather_title);

        // Add weather information
        let weather_info = Label::builder()
            .label("Qui verranno mostrate le informazioni meteo")
            .css_classes(["weather-info"])
            .build();
        box_container.append(&weather_info);

        // Add back button
        let back_button = Button::with_label("Torna indietro");
        back_button.add_css_class("small-button");
        back_button.set_margin_top(12);
        box_container.append(&back_button);

        Self { container: box_container }
    }

    // Returns the widget to add to the Stack
    pub fn get_widget(&self) -> &Box {
        &self.container
    }

    // Connects the back button handler
    pub fn connect_back_button<F: Fn() + 'static>(&self, callback: F) {
        if let Some(button) = self.container.first_child()
            .and_then(|w| w.next_sibling())
            .and_then(|w| w.next_sibling())
            .and_then(|w| w.downcast::<Button>().ok()) {
            button.connect_clicked(move |_| callback());
        }
    }
}
```

**Explanation:**
- `WeatherView` is a struct that encapsulates all the weather view logic
- `new()` creates all necessary widgets
- `get_widget()` allows getting the container to add to the Stack
- `connect_back_button()` allows connecting a callback function to the back button

### Step 2: Create the Main Window with Stack

**File: `main_window_ui.rs`**

```rust
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, CssProvider, Label, Stack, gdk::Display};
use crate::ui::weather_ui::WeatherView;

pub struct MainWindow {
    window: ApplicationWindow,
    stack: Stack,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        // Create the window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(360)
            .default_height(640)
            .build();

        // Load CSS styles
        let provider = CssProvider::new();
        provider.load_from_path("src/ui/style.css");
        
        if let Some(display) = Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        // Create the Stack for navigation
        let stack = Stack::new();

        // Create the main view
        let main_box = Self::create_main_view();
        stack.add_named(&main_box, Some("main"));

        // Create and add the weather view
        let weather_view = WeatherView::new();
        stack.add_named(weather_view.get_widget(), Some("weather"));

        // Configure event handlers
        Self::setup_navigation(&stack, &main_box, &weather_view);

        // Set the Stack as the window's child
        window.set_child(Some(&stack));

        Self { window, stack }
    }

    fn create_main_view() -> Box {
        let main_box = Box::new(gtk4::Orientation::Vertical, 0);
        main_box.set_halign(gtk4::Align::Center);
        main_box.set_valign(gtk4::Align::Center);

        let welcome_label = Label::builder()
            .label("Welcome to Weather App")
            .css_classes(["welcome-title"])
            .build();
        main_box.append(&welcome_label);

        let button = Button::with_label("Check Weather");
        button.add_css_class("small-button");
        button.set_margin_top(12);
        main_box.append(&button);

        main_box
    }

    fn setup_navigation(stack: &Stack, main_box: &Box, weather_view: &WeatherView) {
        // Find the "Check Weather" button
        if let Some(button) = main_box.first_child()
            .and_then(|w| w.next_sibling())
            .and_then(|w| w.downcast::<Button>().ok()) {
            
            let stack_clone = stack.clone();
            button.connect_clicked(move |_| {
                stack_clone.set_visible_child_name("weather");
            });
        }

        // Connect the weather view's back button
        let stack_clone = stack.clone();
        weather_view.connect_back_button(move || {
            stack_clone.set_visible_child_name("main");
        });
    }

    pub fn present(&self) {
        self.window.present();
    }
}
```

### Step 3: Configure the Module

**File: `ui/mod.rs`**

```rust
pub mod main_window_ui;
pub mod weather_ui;
```

### Step 4: Create CSS Styles

**File: `ui/style.css`**

```css
.welcome-title {
    font-size: 24px;
    font-weight: bold;
    margin-bottom: 20px;
}

.weather-title {
    font-size: 24px;
    font-weight: bold;
    margin-bottom: 20px;
    color: #2c3e50;
}

.weather-info {
    font-size: 16px;
    margin-bottom: 20px;
    padding: 10px;
}

.small-button {
    padding: 8px 16px;
    font-size: 14px;
}
```

### Step 5: Application Entry Point

**File: `main.rs`**

```rust
mod ui;

use gtk4::prelude::*;
use gtk4::Application;
use ui::main_window_ui::MainWindow;

fn main() {
    let app = Application::builder()
        .application_id("org.example.WeatherApp")
        .build();

    app.connect_activate(|app| {
        let window = MainWindow::new(app);
        window.present();
    });

    app.run();
}
```

## Key Code Explanation

### 1. Stack Creation

```rust
let stack = Stack::new();
```

Creates a new empty Stack that will contain all views.

### 2. Adding Views to the Stack

```rust
stack.add_named(&main_box, Some("main"));
stack.add_named(weather_view.get_widget(), Some("weather"));
```

- `add_named()` adds a widget to the Stack with an identifier name
- The name allows referring to the view uniquely
- The "main" view is the one shown at startup

### 3. Navigation Management

```rust
let stack_clone = stack.clone();
button.connect_clicked(move |_| {
    stack_clone.set_visible_child_name("weather");
});
```

- `stack.clone()` creates a shared reference to the Stack
- `connect_clicked()` registers a closure for the click event
- `set_visible_child_name()` changes the visible view

### 4. Back Button Pattern

```rust
weather_view.connect_back_button(move || {
    stack_clone.set_visible_child_name("main");
});
```

- The weather view exposes a method to connect the callback
- This pattern keeps the weather view autonomous
- Navigation logic remains in the main window

## Navigation Patterns

### Pattern 1: Centralized Stack

```
MainWindow (manages Stack)
├── Stack (navigation container)
    ├── Main View ("main")
    └── Weather View ("weather")
```

**Advantages:**
- 🎯 Single control point for navigation
- 🔄 Easy to add new views
- 📊 Centralized state

### Pattern 2: Autonomous Views

Each view is responsible for:
- Creating its own widgets
- Managing its internal events
- Exposing methods to interact with the outside

**Example:**
```rust
// The weather view knows nothing about the Stack
// It only knows that someone wants to connect a callback to the back button
weather_view.connect_back_button(|| {
    // Navigation logic managed externally
});
```

### Pattern 3: Callback Navigation

Instead of passing the Stack to views, we use callbacks:

```rust
// ❌ Wrong approach: strong coupling
impl WeatherView {
    pub fn new(stack: Stack) -> Self {
        // The view knows the Stack -> coupling!
    }
}

// ✅ Correct approach: callback
impl WeatherView {
    pub fn connect_back_button<F: Fn() + 'static>(&self, callback: F) {
        // The view is autonomous, only receives a callback
    }
}
```

## Best Practices

### 1. Separation of Concerns

**✅ Done well:**
- `MainWindow` manages navigation
- `WeatherView` manages only its own UI
- Each module has a clear responsibility

**❌ To avoid:**
- Mixing navigation logic with UI logic
- Creating circular dependencies between modules

### 2. Memory Management

**✅ Done well:**
```rust
let stack_clone = stack.clone();
button.connect_clicked(move |_| {
    stack_clone.set_visible_child_name("weather");
});
```

**Explanation:**
- `clone()` creates a new reference to the same object
- The closure takes ownership of the cloned reference
- The original remains available for other uses

### 3. Clear and Consistent Naming

**✅ Good examples:**
```rust
stack.add_named(&main_box, Some("main"));
stack.add_named(weather_view.get_widget(), Some("weather"));

stack_clone.set_visible_child_name("weather");
stack_clone.set_visible_child_name("main");
```

**Tips:**
- Use descriptive names for views
- Be consistent in naming
- Avoid generic names like "view1", "view2"

### 4. Error Handling

**✅ Done well:**
```rust
if let Some(button) = self.container.first_child()
    .and_then(|w| w.next_sibling())
    .and_then(|w| w.downcast::<Button>().ok()) {
    // Use the button safely
}
```

**Explanation:**
- Use `if let` to handle cases where the widget doesn't exist
- Chain `and_then()` to navigate the hierarchy safely
- Use `downcast()` to verify the widget type

## Future Extensions

### 1. Add Other Views

```rust
// In main_window_ui.rs
let settings_view = SettingsView::new();
stack.add_named(settings_view.get_widget(), Some("settings"));

// Navigation to settings
stack_clone.set_visible_child_name("settings");
```

### 2. Custom Transitions

```rust
// In the Stack constructor
let stack = Stack::builder()
    .transition_type(gtk4::StackTransitionType::SlideLeftRight)
    .transition_duration(500) // milliseconds
    .build();
```

### 3. Navigation with Parameters

```rust
// To pass data between views
pub struct NavigationData {
    pub city: String,
    pub temperature: f32,
}

// Modify callbacks to accept data
weather_view.connect_back_button_with_data(|data| {
    println!("Returning from {} view", data.city);
});
```

### 4. Stack with Complex Animations

```rust
stack.set_transition_type(gtk4::StackTransitionType::OverUp);
stack.set_transition_duration(300);
```

**Available transition types:**
- `None`: No transition
- `Crossfade`: Fade
- `SlideLeftRight`: Horizontal slide
- `SlideUpDown`: Vertical slide
- `OverUp`, `OverDown`, `OverLeft`, `OverRight`: Overlay

## Common Troubleshooting

### Problem 1: View doesn't change
```rust
// ❌ WRONG
button.connect_clicked(|_| {
    stack.set_visible_child_name("weather"); // stack is not available
});

// ✅ CORRECT
let stack_clone = stack.clone();
button.connect_clicked(move |_| {
    stack_clone.set_visible_child_name("weather");
});
```

### Problem 2: Widget not found
```rust
// ❌ WRONG - assumes the button is the first child
let button = container.first_child().unwrap();

// ✅ CORRECT - safe navigation
if let Some(button) = container.first_child()
    .and_then(|w| w.next_sibling())
    .and_then(|w| w.downcast::<Button>().ok()) {
    // Use the button
}
```

### Problem 3: CSS not applied
```rust
// Make sure to load CSS BEFORE creating widgets
let provider = CssProvider::new();
provider.load_from_path("src/ui/style.css");

if let Some(display) = Display::default() {
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

// Now create widgets
let label = Label::builder()
    .css_classes(["welcome-title"])
    .build();
```

## Conclusion

This implementation demonstrates how to use `gtk4::Stack` to create smooth navigation between views in a GTK4-Rust application. The key points are:

1. **Separation of concerns**: Each view manages its own UI
2. **Centralized Stack**: Single control point for navigation
3. **Callback navigation**: Pattern that avoids strong coupling
4. **Modularized code**: Easy to extend and maintain

This approach can be easily extended for more complex applications with many views and advanced navigation.