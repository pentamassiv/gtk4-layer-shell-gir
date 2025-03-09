#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(warnings)]

use gtk4_session_lock_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

mod auto;
pub use auto::Instance;

mod manual;
pub use manual::is_supported;
