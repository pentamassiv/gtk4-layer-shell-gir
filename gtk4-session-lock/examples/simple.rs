use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk4_session_lock::Instance as SessionLockInstance;

fn on_session_locked(_: &SessionLockInstance) {
    println!("Session locked successfully");
}

fn on_session_lock_failed(app: &gtk::Application) {
    eprintln!("The session could not be locked");
    app.quit();
}

fn on_session_unlocked(app: &gtk::Application) {
    println!("Session unlocked");
    app.quit();
}

fn on_unlock_button_clicked(lock: &SessionLockInstance) {
    lock.unlock();
}

fn on_monitor_present(lock: &SessionLockInstance, monitor: gdk::Monitor, app: &gtk::Application) {
    // This function will be called once for each monitor (aka output) present when the session becomes locked, and also
    // whenever a new monitor is plugged in while the session is locked.

    let window = gtk::ApplicationWindow::new(app);

    let bbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(10)
        .build();

    let label = gtk::Label::new(Some("GTK4 Session Lock Example"));
    bbox.append(&label);

    let button = gtk::Button::builder()
        .label("Unlock")
        //.halign(gtk::Align::Center)
        //.valign(gtk::Align::Center)
        .build();

    button.connect_clicked(clone!(
        #[weak]
        lock,
        move |_| {
            on_unlock_button_clicked(&lock);
        }
    ));
    bbox.append(&button);
    // Not displayed, but allows testing that creating popups doesn't crash GTK
    button.set_tooltip_text(Some("Foo Bar Baz"));

    window.set_child(Some(&bbox));

    lock.assign_window_to_monitor(&window, &monitor);
    // DONT call present, gtk_session_lock_instance_assign_window_to_monitor() does that for us
}

fn on_lock_button_clicked(lock: &SessionLockInstance) {
    lock.lock();
}

fn on_quit_button_clicked(lock: &SessionLockInstance, app: &gtk::Application) {
    if lock.is_locked() {
        eprintln!("Quit button somehow pressed while session lock was locked??");
        // If we quit now there would be no way for the user to unlock, don't do that
        return;
    }

    println!("Quitting");
    app.quit();
}

fn create_control_window(lock: &SessionLockInstance, app: &gtk::Application) {
    let lock_button = gtk::Button::with_label("Lock");
    lock_button.connect_clicked(clone!(
        #[weak]
        lock,
        move |_| {
            on_lock_button_clicked(&lock);
        }
    ));

    let quit_button = gtk::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(
        #[weak]
        lock,
        #[weak]
        app,
        move |_| {
            on_quit_button_clicked(&lock, &app);
        }
    ));

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::Start)
        .spacing(6)
        .build();

    vbox.append(&lock_button);
    vbox.append(&quit_button);

    let control_window = gtk::ApplicationWindow::new(app);
    control_window.set_child(Some(&vbox));
    control_window.present();
}

fn activate(app: &gtk::Application) {
    let lock = SessionLockInstance::new();
    lock.connect_locked(on_session_locked);
    lock.connect_failed(clone!(
        #[weak]
        app,
        move |_| on_session_lock_failed(&app)
    ));

    lock.connect_unlocked(clone!(
        #[weak]
        app,
        move |_| on_session_unlocked(&app)
    ));

    lock.connect_monitor(clone!(
        #[weak]
        app,
        move |lock, monitor| on_monitor_present(&lock, monitor.clone(), &app)
    ));

    // Note that you can't create windows while display is locked, but doing it here is fine
    create_control_window(&lock, &app);

    // This actually locks the session, the session may or may not be done locking by the time it returns
    lock.lock();
}

fn main() {
    if !gtk4_session_lock::is_supported() {
        println!("Session lock not supported");
        std::process::exit(1);
    }

    let app = gtk::Application::new(
        Some("com.github.wmww.gtk4-layer-shell.session-lock-example"),
        Default::default(),
    );

    app.connect_activate(activate);
    app.run();
}
