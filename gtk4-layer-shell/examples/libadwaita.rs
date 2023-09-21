// This example is copied from https://gitlab.gnome.org/World/Rust/libadwaita-rs/-/blob/master/libadwaita/examples/hello-world.rs
// The only part that is specific to the usage of gtk4-layer-shell is encased by comments

use libadwaita as adw;

use adw::prelude::*;
use adw::{ActionRow, ApplicationWindow, HeaderBar};
use gtk::{Application, Box, ListBox, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_startup(|_| {
        adw::init().unwrap();
    });

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(true)
            .selectable(false)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            // the content class makes the list look nicer
            .css_classes(vec![String::from("content")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("First App", ""))
                .build(),
        );
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            // add content to window
            .content(&content)
            .build();

        // #################################
        // Part that is specific to use gtk4-layer-shell begins

        // Before the window is first realized, set it up to be a layer surface
        window.init_layer_shell();

        // Display above normal windows
        window.set_layer(Layer::Overlay);

        // Push other windows out of the way
        window.auto_exclusive_zone_enable();

        // Anchors are if the window is pinned to each edge of the output
        let anchors = [
            (Edge::Left, true),
            (Edge::Right, true),
            (Edge::Top, false),
            (Edge::Bottom, true),
        ];

        for (anchor, state) in anchors {
            window.set_anchor(anchor, state);
        }
        // Part that is specific to use gtk4-layer-shell ends
        // #################################

        window.show();
    });

    application.run();
}
