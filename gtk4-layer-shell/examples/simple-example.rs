use gio::prelude::*;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

// https://github.com/wmww/gtk4-layer-shell/blob/v1.3.0/examples/simple-example.c
fn activate(application: &gtk::Application) {
    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Order below normal windows
    window.set_layer(Layer::Top);

    // Push other windows out of the way
    window.auto_exclusive_zone_enable();

    // We don't need to get keyboard input
    // window.set_keyboard_mode(KeyboardMode::None); // None is default

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    window.set_margin(Edge::Left, 40);
    window.set_margin(Edge::Right, 40);
    window.set_margin(Edge::Top, 20);
    window.set_margin(Edge::Bottom, 0); // 0 is default

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    // Edge enum order matches GTK_LAYER_SHELL_EDGE_*: Left, Right, Top, Bottom
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, false),
        (Edge::Top, false),
        (Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    // Set up a widget
    let label = gtk::Label::new(Some(""));
    label.set_markup(
        "<span font_desc=\"100.0\">\
            GTK Layer\nShell example!\
        </span>",
    );
    window.set_child(Some(&label));
    window.present();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.wmww.gtk4-layer-shell.example"),
        Default::default(),
    );

    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}
