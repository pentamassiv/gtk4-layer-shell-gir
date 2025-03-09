use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk4_session_lock::Instance as SessionLockInstance;

fn main() {
    if !gtk4_session_lock::is_supported() {
        println!("Session lock not supported")
    }

    let app = gtk::Application::new(
        Some("com.github.wmww.gtk4-layer-shell.session-lock-example"),
        Default::default(),
    );

    app.connect_activate(activate);
    app.run();
}

fn activate(app: &gtk::Application) {
    let lock = SessionLockInstance::new();
    lock.connect_locked(locked);
    lock.connect_failed(failed);

    lock.connect_unlocked(clone!(
        #[weak]
        app,
        move |_| unlocked(&app)
    ));

    if !lock.lock() {
        // Error message already shown when handling the ::failed signal
        return;
    }

    let display = gdk::Display::default().unwrap();
    let monitors = display.monitors();
    for monitor in monitors.iter::<glib::Object>() {
        let monitor = monitor.unwrap().downcast::<gdk::Monitor>().unwrap();
        let window = gtk::ApplicationWindow::new(app);
        lock.assign_window_to_monitor(&window, &monitor);

        let bbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .spacing(10)
            .build();

        let label = gtk::Label::new(Some("GTK4 Session Lock Example"));
        let button = gtk::Button::builder()
            .label("Unlock")
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        let lock = lock.clone();
        button.connect_clicked(move |_| lock.unlock());

        // Not displayed, but allows testing that creating popups doesn't crash GTK
        button.set_tooltip_text(Some("Foo Bar Baz"));

        bbox.append(&label);
        bbox.append(&button);
        window.set_child(Some(&bbox));
        window.present();
    }
}

fn locked(_: &SessionLockInstance) {
    println!("Session locked successfully");
}

fn failed(_: &SessionLockInstance) {
    eprintln!("The session could not be locked");
}

fn unlocked(app: &gtk::Application) {
    println!("Session unlocked");
    app.quit();
}
